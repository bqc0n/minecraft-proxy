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

## Configuration (WIP)

Refer to the [examples](examples) for example configurations.

TOOD: use toml

### servers

In the `servers` section, you can define multiple `server` objects:

|      Field       |   Type   |                                                     Description                                                     |
|:----------------:|:--------:|:-------------------------------------------------------------------------------------------------------------------:|
|      `host`      |  string  | `IP:port` or `domain-name:port` of the origin server. Port can be ommitted, in which case it will default to 25565. |
|      `bind`      | string[] |        Socket addresses to bind. Default is `[::]:25565`. Note: IPv6 wildcard bind `::` will contains IPv4.         |
| `proxy_protocol` |   bool   |                                     Enable Proxy Protocol v2. Default is false.                                     |
