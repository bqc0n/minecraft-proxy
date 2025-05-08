use bytes::{Buf, BufMut, BytesMut};
use crate::mcp::constants::VARINT_CONTINUE_BIT;
use crate::mcp::protocol::McVarInt;

#[tokio::test]
async fn test_read_varint() -> anyhow::Result<()> {
    let mut buf = BytesMut::with_capacity(2);
    // writing 0x2BA2 (0b00101011_10100010) as VarInt
    // varint: 7 bits and CONTINUE_BIT = byte, and LSByte is written first.
    // also, MSB is CONTINUE_BIT
    // 0x2BA2 7 bits separation: 0b 00_1010111_0100010
    // varint repr for 0x2BA2: [10100010, 01010111]
    buf.put_u8(0b_10100010);
    buf.put_u8(0b_01010111);
    let read = McVarInt::read(&mut buf)?.int();

    assert_eq!(buf.len(), 0); // ensure the buf is consumed
    assert_eq!(read, 0x2BA2);

    Ok(())
}

#[tokio::test]
async fn test_write_varint() {
    let var_int = McVarInt::new(0x952B); // 10010101_00101011
    // 7 bits: 0b 10_0101010_0101011
    // Varint repr for 0x952B: [10101011, 10101010, 00000010]
    let mut buf = BytesMut::with_capacity(3);
    var_int.write(&mut buf);
    assert_eq!(buf.len(), 3);
    assert_eq!(buf[0], 0b_10101011);
    assert_eq!(buf[1], 0b_10101010);
    assert_eq!(buf[2], 0b_00000010);
}
