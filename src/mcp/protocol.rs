use crate::mcp::constants::{VARINT_CONTINUE_BIT, VARINT_CONTINUE_BIT_I32, VARINT_SEGMENT_BITS, VARINT_SEGMENT_BITS_I32};
use bytes::{Buf, BufMut, BytesMut};

pub(crate) trait MinecraftPacket {
    fn to_bytes(&self) -> BytesMut;
}

pub fn create_packet(id: u32, data: BytesMut) -> BytesMut {
    let mut packet_data = BytesMut::new();
    write_varint(&mut packet_data, id as i32);
    packet_data.extend(data);

    let mut packet = BytesMut::new();
    write_varint(&mut packet, packet_data.len() as i32);
    packet.extend(packet_data);

    packet
}

/// returns (length, packet_id)
/// given buffer will contain the rest of the packet i.e. data
pub fn read_packet(buf: &mut BytesMut) -> (u32, u32) {
    let length = read_varint(buf).unwrap();
    let packet_id = read_varint(buf).unwrap();
    return (length, packet_id);
}

/// returns the length of the varint in bytes
pub(super) fn write_varint(buf: &mut BytesMut, mut value: i32) -> u8 {
    let mut length = 0;
    loop {
        length += 1;
        if (value & !VARINT_SEGMENT_BITS_I32) == 0 {
            buf.put_u8(value as u8);
            return length;
        } else {
            buf.put_u8(((value & VARINT_SEGMENT_BITS_I32) | VARINT_CONTINUE_BIT_I32) as u8);
            value >>= 7;
        }
    }
}

pub(crate) fn read_varint(buf: &mut BytesMut) -> anyhow::Result<u32> {
    let mut value = 0;
    let mut pos = 0;

    loop {
        let byte = buf.get_u8();
        value |= ((byte & VARINT_SEGMENT_BITS) as u32) << pos;
        if byte & VARINT_CONTINUE_BIT == 0 {
            break;
        }
        pos += 7;
        if pos >= 32 {
            return Err(anyhow::anyhow!("Varint is too big"));
        }
    }

    Ok(value)
}
