use std::io;
use std::io::ErrorKind;
use bytes::{Buf, BufMut, BytesMut};

const VARINT_SEGMENT_BITS: u8 = 0x7F;
const VARINT_CONTINUE_BIT: u8 = 0x80;

/// パケットをバッファに書き出す（シリアライズ）
pub fn create_packet(id: u8, data: &[u8]) -> BytesMut {
    let mut packet = BytesMut::new();

    write_varint(&mut packet, data.len() as i32);
    write_varint(&mut packet, id as i32);
    packet.extend_from_slice(&data);

    packet
}

pub struct VarInt(pub i32);
impl VarInt {
    pub fn write_to_buf(&self, buf: &mut BytesMut) {
        let mut value = self.0 as u32; // i32を符号なしで扱う
        loop {
            let mut temp = (value & VARINT_SEGMENT_BITS as u32) as u8;
            value >>= 7;
            if value != 0 {
                temp |= VARINT_CONTINUE_BIT;
            }
            buf.put_u8(temp);
            if value == 0 {
                break;
            }
        }
    }

    pub fn read_from_buf(buf: &mut impl Buf) -> io::Result<Self> {
        let mut num_read = 0;
        let mut result = 0u32;

        loop {
            if !buf.has_remaining() {
                return Err(io::Error::new(ErrorKind::UnexpectedEof, "Buffer underflow reading VarInt"));
            }

            let byte = buf.get_u8();
            result |= ((byte & VARINT_SEGMENT_BITS) as u32) << (7 * num_read);

            num_read += 1;
            if num_read > 5 {
                return Err(io::Error::new(ErrorKind::InvalidData, "VarInt too long"));
            }

            if (byte & VARINT_CONTINUE_BIT) == 0 {
                break;
            }
        }

        Ok(VarInt(result as i32))
    }
}

fn write_varint(buf: &mut BytesMut, mut value: i32) {
    loop {
        if (value & !0x7F) == 0 {
            buf.put_u8(value as u8);
            return;
        } else {
            buf.put_u8(((value & 0x7F) | 0x80) as u8);
            value >>= 7;
        }
    }
}
