use crate::mcp::constants::{VARINT_CONTINUE_BIT_I32, VARINT_SEGMENT_BITS_I32};
use bytes::{BufMut, BytesMut};

pub(crate) trait MinecraftPacket {
    fn to_bytes(&self) -> BytesMut;
}

pub fn create_packet(id: u8, data: BytesMut) -> BytesMut {
    let mut packet_data = BytesMut::new();
    write_varint(&mut packet_data, id as i32);
    packet_data.extend(data);
    
    let mut packet = BytesMut::new();
    write_varint(&mut packet, packet_data.len() as i32);
    packet.extend(packet_data);

    packet
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