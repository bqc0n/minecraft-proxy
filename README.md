# Simple Minecraft Proxy

Very WIP. not production ready.

I created this for learning rust and TCP/IP programming.

## Features

### Implemented

- IPv4 and IPv6 support
- TCP (Java Edition) Proxy
- Proxy Protocol v2
- Health Checking
- Sorry Server (Fake MC Server) (Partially, only for server ping)

### Not Implemented

- UDP (Bedrock Edition) Proxy
- SRV Record Support
- Sanity Checks (e.g. panic when binding to the same address:port)
- Sorry Server Kick Message

## Configuration

Refer to the [examples](examples) for configurations.