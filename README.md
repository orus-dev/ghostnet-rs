# ghostnet-rs
A rust library for the Ghost Net protocol


## Protocol
**Client connects to the router**
Client -> `[10, 7, 1, 21, 3000]` -> Router

**Client route message to `10.7.1.21`**
Client -> `[<message-bytes>]` -> Router -> (Client / Router) -> `10.7.1.21`

**`10.7.1.21` Responds back**
`10.7.1.21` -> `[<message-bytes>]` -> (Client / Router) -> Client

### How a client routes
Router -> `[10, 7, 1, 21, 3000]` -> Client

Router -> `[<message-bytes>]` -> Client

Client -> `[<message-bytes>]` -> Router

## What is a router?
A router manages the clients and routes messages from one client to another to ensure security.

## Todo
- End-To-End Encryption
- Mask the communications like **HTTPS**