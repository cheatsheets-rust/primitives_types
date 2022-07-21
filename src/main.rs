use std::mem::size_of_val;

fn main() {

    // variables
    let ma_variable_bool: bool = true;
    let ma_variable_u8: u8 = 42;
    let ma_variable_u16: u16 = 42;
    let ma_variable_i8: i8 = -42;
    let ma_variable_f32: f32 = 42.5;
    let ma_variable_char: char = 'c';
    let ma_variable_str: &str = "Hello world !";
    let ma_variable_ref_bool: &[bool] = &[true, false, true, true, false];
    let ma_variable_tuple: (bool, u8, char) = (true, 42, 'c');

    // Value variables
    dbg!(ma_variable_bool);
    dbg!(ma_variable_u8);
    dbg!(ma_variable_u16);
    dbg!(ma_variable_i8);
    dbg!(ma_variable_f32);
    dbg!(ma_variable_char);
    dbg!(ma_variable_str);
    dbg!(ma_variable_ref_bool);
    dbg!(ma_variable_tuple);

    // size in bits
    dbg!(size_of_val(ma_variable_str));
    dbg!(size_of_val(ma_variable_ref_bool));
    dbg!(size_of_val(&ma_variable_tuple));
    dbg!(size_of_val(&ma_variable_char));
    dbg!(size_of_val(&ma_variable_f32));
    dbg!(size_of_val(&ma_variable_u8));

    // size MAX / MIN in bits of primitives types
    dbg!(u8::MIN);
    dbg!(u8::MAX);
    dbg!(f32::MAX);
    dbg!(u16::MAX);
    dbg!(u32::MAX);
    dbg!(u64::MAX);
    dbg!(u128::MAX);
    dbg!(i8::MIN);
    dbg!(i8::MAX);
    dbg!(i128::MIN);
    dbg!(i128::MAX);

    // assign and mutate a variable
    let variable1 : u8 = 32;
    let variable2 : u8 = 42;
    dbg!(variable1, variable2);
    let mut variable_mut : u8 = 12;
    variable_mut = variable2;
    dbg!(variable_mut);
}
