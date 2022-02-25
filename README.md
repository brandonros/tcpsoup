# tcpsoup
Really confusing TCP tunnels

## How to use

`diag-tunnel` binds on `0.0.0.0:5555`, acceps connection from `vehicle-tunnel`

`diag-tunnel` also binds on `diag:127.0.0.1:3000` to intercept requests from `diag-client`

`vehicle-tunnel` connects to `DIAG_TUNNEL_EXTERNAL_IP:55555` to receive requests from `diag-tunnel`

`vehicle-tunnel` also connects to `vehicle-client:127.0.0.1:3000`

1. On diag machine: Start `diag-tunnel`, record `DIAG_TUNNEL_EXTERNAL_IP`, unblock port `5555` to external internet.
2. On vehicle machine: Start `vehicle-server`.
3. On vehicle machine: Set `DIAG_PROXY_EXTERNAL_IP` then start `vehicle-tunnel`.
4. On diag machine: Run `diag-client`, expect request to go to `diag-client->diag-tunnel->vehicle-tunnel->vehicle-server` and back.
