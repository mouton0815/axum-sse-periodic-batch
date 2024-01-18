use std::collections::VecDeque;
use std::fmt::Debug;
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use std::time::Duration;

use futures_util::Stream;
use log::error;
use tokio::time::{Interval, interval};

///
/// Trait for custom fetcher implementations needed by [ScheduledStream](ScheduledStream).
///
pub trait Fetcher<T, E> {
    fn fetch(&mut self) -> Result<Vec<T>, E>;
}

pub type BoxedFetcher<T, E> = Box<dyn Fetcher<T, E> + Send>;

///
/// An implementation of [Stream](futures_util::Stream) that periodically fetches items
/// from a source through a [Fetcher](Fetcher). While ``Fetcher::fetch()`` returns a vector
/// of items, method ``poll_next()`` returns the items one-by-one, utilizing a buffer.
///
pub struct ScheduledStream<T, E> {
    interval: Interval,
    buffer: Box<VecDeque<T>>,
    fetcher: BoxedFetcher<T, E>
}

impl<T, E> ScheduledStream<T, E> {
    pub fn new(duration: Duration, fetcher: BoxedFetcher<T, E>) -> Self {
        Self {
            interval: interval(duration),
            buffer: Box::new(VecDeque::new()),
            fetcher
        }
    }
}

impl<T, E: Debug> Stream for ScheduledStream<T, E> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>> {
        if self.buffer.len() == 0 {
            ready!(self.interval.poll_tick(cx));
            match self.fetcher.fetch() {
                Ok(batch) => {
                    for item in batch {
                        self.buffer.push_back(item);
                    }
                }
                Err(err) => {
                    error!("Fetcher returned error {:?}, stop polling", err);
                    return Poll::Ready(None)
                }
            }
        }
        return match self.buffer.pop_front() {
            Some(x) => Poll::Ready(Some(x)),
            None => Poll::Pending
        }
    }
}
