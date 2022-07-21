use std::mem::size_of_val;

fn main() {
    let ma_variable_bool: bool = true;
    let ma_variable_u8: u8 = 42;
    let ma_variable_u8: u16 = 42;
    let ma_variable_u8: i8 = -42;
    dbg!(ma_variable_bool);
    dbg!(ma_variable_u8);
    dbg!(u8::MIN);
    dbg!(u8::MAX);
    dbg!(u16::MAX);
    dbg!(u32::MAX);
    dbg!(u64::MAX);
    dbg!(u128::MAX);
    dbg!(i8::MIN);
    dbg!(i8::MAX);
    dbg!(i128::MIN);
    dbg!(i128::MAX);
}
