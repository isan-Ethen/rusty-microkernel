#![no_std]
#![no_main]

use core::{slice::Iter, str::Chars};

#[allow(improper_ctypes)]
extern "C" {
    fn putchar(ch: char);
}

pub enum Argument<'a> {
    Decimal(i32),
    Hexadecimal(i32),
    String(&'a str),
}

impl<'a> Argument<'a> {
    pub fn new_decimal(arg: i32) -> Argument<'a> {
        Argument::Decimal(arg)
    }

    pub fn new_hexadecimal(arg: i32) -> Argument<'a> {
        Argument::Hexadecimal(arg)
    }

    pub fn new_string(arg: &'a str) -> Argument<'a> {
        Argument::String(arg)
    }

    fn into_decimal(&self) -> Option<&i32> {
        if let Argument::Decimal(inner_val) = self {
            Some(&inner_val)
        } else {
            None
        }
    }

    fn into_hexadecimal(&self) -> Option<&i32> {
        if let Argument::Hexadecimal(inner_val) = self {
            Some(&inner_val)
        } else {
            None
        }
    }

    fn into_string(&self) -> Option<&str> {
        if let Argument::String(inner_val) = self {
            Some(inner_val)
        } else {
            None
        }
    }
}

#[no_mangle]
pub fn print(string: &str, args: &[Argument]) {
    let mut str_iter: Chars<'_> = string.chars();
    let mut args_iter: Iter<'_, Argument<'_>> = args.iter();

    while let Some(fmt) = str_iter.next() {
        match fmt {
            '%' => {
                if let Some(fmt) = str_iter.next() {
                    match fmt {
                        'd' => {
                            if let Some(arg) = args_iter.next() {
                                print_decimal(arg);
                            }
                        }
                        'x' => {
                            if let Some(arg) = args_iter.next() {
                                print_hexadecimal(arg);
                            }
                        }
                        's' => {
                            if let Some(arg) = args_iter.next() {
                                print_string(arg);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => unsafe { putchar(fmt) },
        }
    }
}

fn print_decimal(arg: &Argument) {
    if let Some(inner_val) = arg.into_decimal() {
        let mut value: i32 = *inner_val;
        if value < 0 {
            unsafe { putchar('-') };
            value = -value;

            let mut divisor: i32 = 1;
            while value / divisor > 9 {
                divisor *= 10;
            }

            while divisor > 0 {
                unsafe { putchar((value / divisor) as u8 as char) };
                value %= divisor;
                divisor /= 10;
            }
        }
    }
}

fn print_hexadecimal(arg: &Argument) {
    if let Some(inner_val) = arg.into_hexadecimal() {
        for i in (0..=7).rev() {
            let nibble: usize = ((inner_val >> (i * 4)) & 0xf) as usize;
            unsafe {
                putchar(
                    [
                        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e',
                        'f',
                    ][nibble],
                );
            }
        }
    }
}

fn print_string(arg: &Argument) {
    if let Some(inner_val) = arg.into_string() {
        for c in inner_val.chars() {
            unsafe {
                putchar(c);
            }
        }
    }
}
