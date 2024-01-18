# Serve axum SSE from a batched source  

This snippet demonstrates how an `axum` SSE ("Server Sent Events") handler can receive messages
from a "batched source", such as a database table. The source is queried periodically and may return
varying numbers of results (including no results).

The pattern is useful, for example, to implement the
[outbox pattern](https://microservices.io/patterns/data/transactional-outbox.html).
There, a message relay periodically reads events from the outbox table and sends them to a message broker.
In our case, the message broker is the `axum` SSE channel.

# Running the Server

```shell
RUST_LOG=debug cargo run
```

# Connecting a Client
```shell
curl http://localhost:3000/sse
```
