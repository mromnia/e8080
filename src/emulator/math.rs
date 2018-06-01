pub fn add_4(x: u8, y: u8) -> (u8, bool) {
    let x = x & 0x0F;
    let y = y & 0x0F;
    let result = x + y;
    let carry = (result & 0xF0) > 0;

    (result, carry)
}

pub fn add_8(x: u8, y: u8) -> (u8, bool, bool) {
    let result: u16 = (x as u16) + (y as u16);
    let carry = (result & 0xFF00) > 0;
    let (_, acarry) = add_4(x, y);
    (result as u8, carry, acarry)
}

pub fn add_16(x: u16, y: u16) -> (u16, bool) {
    let result: u32 = (x as u32) + (y as u32);
    let carry = (result & 0xFFFF0000) > 0;
    (result as u16, carry)
}

pub fn sub_8(x: u8, y: u8) -> (u8, bool, bool) {
    let (res_neg, c_neg, ac_neg) = negate_8(y);
    let (result, carry, acarry) = add_8(x, res_neg);
    (result, !(carry || c_neg), (acarry || ac_neg))
}

pub fn negate_8(x: u8) -> (u8, bool, bool) {
    let x = !x;
    add_8(x, 1)
}

pub fn combine_8_to_16(x: u8, y: u8) -> u16 {
    ((x as u16) << 8) + (y as u16)
}

pub fn higher_8(x: u16) -> u8 {
    (x >> 8) as u8
}

pub fn lower_8(x: u16) -> u8 {
    x as u8
}

pub fn rot_left(x: u8, lowest_bit_override: Option<bool>) -> (u8, bool) {
    let mut x = x as u16;
    x = (x << 1) | (x >> 7);

    let carry = (x & 0xFF00) > 0;

    if let Some(b) = lowest_bit_override {
        if b {
            x |= 0x01
        } else {
            x &= !0x01
        }
    }

    (x as u8, carry)
}

pub fn rot_right(x: u8, highest_bit_override: Option<bool>) -> (u8, bool) {
    let carry = (x & 0x01) > 0;

    let mut x = x as u16;
    x = (x >> 1) | (x << 7);

    if let Some(b) = highest_bit_override {
        if b {
            x |= 0x80
        } else {
            x &= !0x80
        }
    }

    (x as u8, carry)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sub8() {
        let (result, carry, acarry) = sub_8(5, 0);
        assert_eq!(result, 5);
        assert!(carry);
        assert!(acarry);
    }
}
