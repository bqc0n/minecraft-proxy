# Simple Minecraft Proxy

Very WIP. not production ready.

I created this for learning rust and TCP/IP programming.

## Configuration (Not Implemented)

### Minimal

```yaml
vanilla:
  bind: "0.0.0.0:25565"
  host: "192.0.2.1"
```

### Extended

```yaml
servers:
  vanilla_server:
    # if no 'bind' found, it binds to "0.0.0.0:25565" and "[::]:25565"
    server: "minecraft.example.com" # Port defaults to 25565
  modded_server:
    proxy_protocol: true # Optional, default is false
    bind:
      - "0.0.0.0:25565"
      - "[::]:25565"
    server: "192.0.2.1:25656" # Port can be specified
# You can configure Sorry server, which is used when all servers are offline
sorry:
  version: "§cOffline"
  motd:
    - "§cServer is currently offline."
    - "§bPlease try again later."
  kick_message:
    - "§cServer is currently offline."
    - "§bPlease try again later."
```