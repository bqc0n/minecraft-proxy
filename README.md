# Simple Minecraft Proxy

Very WIP. not production ready.

I created this for learning rust and TCP/IP programming.

## Features

### Implemented

- IPv4 and IPv6 support
- TCP (Java Edition) Proxy
- Proxy Protocol v2
- Health Checking
- Sorry Server (Fake MC Server)

### Not Implemented

- UDP (Bedrock Edition) Proxy
- SRV Record Support
- Sanity Checks (e.g. panic when binding to the same address:port)

## Configuration

Refer to the [examples](examples) for example configurations.

```toml
[[servers]]
# ....
[health_check] # optional
# ....
[sorry_server] # optional
# ....
```

### `servers` (array)

|      Field       |   Type   |                                                     Description                                                     |
|:----------------:|:--------:|:-------------------------------------------------------------------------------------------------------------------:|
|      `host`      |  String  | `IP:port` or `domain-name:port` of the origin server. Port can be ommitted, in which case it will default to 25565. |
|      `bind`      | String[] |        Socket addresses to bind. Default is `[::]:25565`. Note: IPv6 wildcard bind `::` will contains IPv4.         |
| `proxy_protocol` |   bool   |                                     Enable Proxy Protocol v2. Default is false.                                     |

### `health_check`

This table is optional.

|     Field      |  Type   |    Description    |
|:--------------:|:-------:|:-----------------:|
|   `enabled`    |  bind   | Default is false. | 
| `interval_sec` | Integer |  Default is 5s.   |
| `timeout_sec`  | Integer |  Default is 2s.   |

### `sorry_server`

This table is optional.

|     Field      |  Type  |                         Description                         |
|:--------------:|:------:|:-----------------------------------------------------------:|
|   `version`    | String |      A string that is displayed next to the ping bars.      |
|     `motd`     | String |                            MOTD.                            |
| `kick_message` | String | A message that is displayed when a player attempts to join. |
