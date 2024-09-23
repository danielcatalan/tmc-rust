pub(crate) const fn create_mask(lsb: u8, msb: u8) -> u32 {
    let bits = ((msb as u16) + 1) - lsb as u16;
    let mask = 2u64.pow(bits as u32) - 1;
    (mask << lsb) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_mask() {
        let mask = create_mask(0, 0); // 0b0001
        assert_eq!(1, mask);

        let mask = create_mask(0, 1); // 0b0011
        assert_eq!(3, mask);

        let mask = create_mask(2, 2); // 0b0100
        assert_eq!(4, mask);

        let mask = create_mask(0, 2); // 0b0111
        assert_eq!(7, mask);

        let mask = create_mask(0, 31);
        assert_eq!(4_294_967_295, mask);
    }
}
