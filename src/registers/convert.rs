pub trait Convert {
    fn from_u32(value: u32) -> Self;
    fn to_u32(&self) -> u32;
}

impl Convert for u8 {
    #[inline]
    fn from_u32(value: u32) -> Self {
        value as Self
    }
    #[inline]
    fn to_u32(&self) -> u32 {
        *self as u32
    }
}

impl Convert for u32 {
    #[inline]
    fn from_u32(value: u32) -> Self {
        value
    }
    #[inline]
    fn to_u32(&self) -> u32 {
        *self
    }
}
