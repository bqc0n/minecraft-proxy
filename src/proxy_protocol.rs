use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs};
use tokio::process::Command;

const PROXY_PROTOCOL_START: &'static [u8] = &[0x0D, 0x0A, 0x0D, 0x0A, 0x00, 0x0D, 0x0A, 0x51, 0x55, 0x49, 0x54, 0x0A];

const AF_UNSPEC: u8 = 0x00;
const AF_INET: u8 = 0x10;
const AF_INET6: u8 = 0x20;
const AF_UNIX: u8 = 0x30;

const TRANSPORT_UNSPEC: u8 = 0x00;
const TRANSPORT_STREAM: u8 = 0x01;
const TRANSPORT_DGRAM: u8 = 0x02;

pub enum CommandV2 {
    Local,
    Proxy,
}

impl CommandV2 {
    fn get_num(&self) -> u8 {
        match self {
            CommandV2::Local => 0x20,
            CommandV2::Proxy => 0x21,
        }
    }
}

enum AddressFamily {
    AfUnspec,
    AfInet,
    AfInet6,
    AfUnix,
}

enum TransportProtocol {
    Unspec,
    /// TCP
    Stream,
    /// UDP
    Dgram,
}

struct ProxyHeaderV2 {
    sig: [u8; 12],
    version_and_command: u8,
    af_and_transport: u8,
    length: u16,
}

impl ProxyHeaderV2 {
    fn create_v4(addr: SocketAddrV4, command: CommandV2, transport: TransportProtocol) -> ProxyHeaderV2 {
        let af_and_transport = match transport {
                TransportProtocol::Unspec => AF_INET | TRANSPORT_UNSPEC,
                TransportProtocol::Stream => AF_INET | TRANSPORT_STREAM,
                TransportProtocol::Dgram => AF_INET | TRANSPORT_DGRAM,
            };
        let length = 12;
        ProxyHeaderV2 {
            sig: <[u8; 12]>::try_from(PROXY_PROTOCOL_START).unwrap(),
            version_and_command: command.get_num(),
            af_and_transport,
            length,
        }
    }

    fn create_v6(addr: SocketAddrV6, command: CommandV2, transport: TransportProtocol) -> ProxyHeaderV2 {
        let af_and_transport = match transport {
                TransportProtocol::Unspec => AF_INET6 | TRANSPORT_UNSPEC,
                TransportProtocol::Stream => AF_INET6 | TRANSPORT_STREAM,
                TransportProtocol::Dgram => AF_INET6 | TRANSPORT_DGRAM,
            };
        let length = 36;
        ProxyHeaderV2 {
            sig: <[u8; 12]>::try_from(PROXY_PROTOCOL_START).unwrap(),
            version_and_command: command.get_num(),
            af_and_transport,
            length,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.sig);
        data.push(self.version_and_command);
        data.push(self.af_and_transport);
        data.extend_from_slice(&self.length.to_be_bytes());
        data
    }
}

struct V4ProxyAddress {
    src: SocketAddrV4,
    dest: SocketAddrV4,
}

impl V4ProxyAddress {
    fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.src.ip().octets());
        data.extend_from_slice(&self.dest.ip().octets());
        data.extend_from_slice(&self.src.port().to_be_bytes());
        data.extend_from_slice(&self.dest.port().to_be_bytes());
        data
    }
}

struct V6ProxyAddress {
    src: SocketAddrV6,
    dest: SocketAddrV6,
}

impl V6ProxyAddress {
    fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.src.ip().octets());
        data.extend_from_slice(&self.dest.ip().octets());
        data.extend_from_slice(&self.src.port().to_be_bytes());
        data.extend_from_slice(&self.dest.port().to_be_bytes());
        data
    }
}

struct UnixProxyAddress {
    src: [u8; 108],
    dest: [u8; 108],
}

pub fn append_pp_v2_ipv4(data: &mut Vec<u8>, src: SocketAddrV4, dest: SocketAddrV4, command: CommandV2) -> anyhow::Result<()> {
    let header = ProxyHeaderV2::create_v4(dest, command, TransportProtocol::Stream);
    data.extend_from_slice(&header.to_bytes());
    let addr = V4ProxyAddress { src, dest };
    data.extend_from_slice(&addr.to_bytes());

    Ok(())
}

pub fn append_pp_v2_ipv6(data: &mut Vec<u8>, src: SocketAddrV6, dest: SocketAddrV6, command: CommandV2) -> anyhow::Result<()> {
    let header = ProxyHeaderV2::create_v6(dest, command, TransportProtocol::Stream);
    data.extend_from_slice(&header.to_bytes());
    let addr = V6ProxyAddress { src, dest };
    data.extend_from_slice(&addr.to_bytes());

    Ok(())
}