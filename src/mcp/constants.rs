pub(super) const HANDSHAKE: u8 = 0x00;
pub const VARINT_SEGMENT_BITS: u8 = 0x7F;
pub const VARINT_SEGMENT_BITS_I32: i32 = 0x7F;
pub const VARINT_CONTINUE_BIT: u8 = 0x80;
pub const VARINT_CONTINUE_BIT_I32: i32 = 0x80;
pub const DEFAULT_PROTOCOL: u32 = 763;