pub fn add_8(x: u8, y: u8) -> (u8, bool, bool) {
    let result: u16 = (x as u16) + (y as u16);
    let carry = (result & 0xFF00) > 0;
    let acarry = check_add_acarry(result as u8, x, y);
    (result as u8, carry, acarry)
}

fn check_add_acarry(result: u8, x: u8, y: u8) -> bool {
    result & 0x08 != ((x & 0x08) ^ (y & 0x08))
}

pub fn add_16(x: u16, y: u16) -> (u16, bool) {
    let result: u32 = (x as u32) + (y as u32);
    let carry = (result & 0xFFFF0000) > 0;
    (result as u16, carry)
}

pub fn sub_8(x: u8, y: u8) -> (u8, bool, bool) {
    let (result, carry, acarry) = add_8(x, negate_8(y));
    (result, !carry, acarry)
}

pub fn negate_8(x: u8) -> u8 {
    -(x as i8) as u8
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

pub fn rot_left(x: u8, with_carry: bool) -> (u8, bool) {
    let mut x = (x as u16) << 1;
    let carry = (x & 0xFF00) > 0;

    if with_carry {
        x |= 0x01;
    }

    (x as u8, carry)
}

pub fn rot_right(x: u8, with_carry: bool) -> (u8, bool) {
    let carry = (x & 0x01) > 0;
    let mut x = (x as u16) >> 1;

    if with_carry {
        x |= 0x80;
    }

    (x as u8, carry)
}
