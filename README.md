# tcpsoup
Really confusing TCP tunnels

## Goal

`diag-client` wants to talk to `vehicle-server` but `vehicle-server` has port blocked. However, `vehicle` can reach `diag` port wise. Therefore, the flow of traffic must be intercepted and reversed.

## TODO

How to achieve this with `netcat`, `socat`, or equivalent instead?

## How to use

`diag-tunnel-server` binds on `0.0.0.0:5555`, accepts connection from `vehicle-tunnel-client`

`diag-tunnel-server` also binds on `diag:127.0.0.1:3000` to intercept requests from `diag-client`

`vehicle-tunnel-client` connects to `DIAG_TUNNEL_EXTERNAL_IP:55555` to receive requests from `diag-tunnel-server`

`vehicle-tunnel-client` also connects to `vehicle-client:127.0.0.1:3000`

1. On diag machine: Start `diag-tunnel-server`, record `DIAG_TUNNEL_EXTERNAL_IP`, unblock port `5555` to external internet.
2. On vehicle machine: Start `vehicle-server`.
3. On vehicle machine: Set `DIAG_TUNNEL_EXTERNAL_IP` then start `vehicle-tunnel-client`.
4. On diag machine: Run `diag-client`, expect request to go to `diag-client -> diag-tunnel-server -> vehicle-tunnel-client -> vehicle-server` and back.
