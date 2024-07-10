#![no_std]
#![no_main]
#![allow(unused)]

use core::{slice::Iter, str::Chars};

#[allow(improper_ctypes)]
extern "C" {
    fn putchar(ch: char);
}

pub enum Argument<'a> {
    Decimal(i32),
    Hexadecimal(i32),
    UInt(u32),
    HexUInt(u32),
    String(&'a str),
}

impl<'a> Argument<'a> {
    pub fn new_decimal(arg: i32) -> Argument<'a> {
        Argument::Decimal(arg)
    }

    pub fn new_hexadecimal(arg: i32) -> Argument<'a> {
        Argument::Hexadecimal(arg)
    }

    pub fn new_uint(arg: u32) -> Argument<'a> {
        Argument::UInt(arg)
    }

    pub fn new_hexuint(arg: u32) -> Argument<'a> {
        Argument::UInt(arg)
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

    fn into_uint(&self) -> Option<&u32> {
        if let Argument::UInt(inner_val) = self {
            Some(&inner_val)
        } else {
            None
        }
    }

    fn into_hexuint(&self) -> Option<&u32> {
        if let Argument::HexUInt(inner_val) = self {
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
                                print_hex(arg);
                            }
                        }
                        'u' => {
                            if let Some(arg) = args_iter.next() {
                                print_uint(arg);
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

pub fn println(string: &str, args: &[Argument]) {
    print(string, args);
    unsafe {
        putchar('\n');
    }
}

fn print_decimal(arg: &Argument) {
    if let Some(inner_val) = arg.into_decimal() {
        let mut value: i32 = *inner_val;
        if value < 0 {
            unsafe { putchar('-') };
            value = -value;
        }

        let mut divisor: i32 = 1;
        while value / divisor > 9 {
            divisor *= 10;
        }

        while divisor > 0 {
            unsafe { putchar(('0' as u8 + (value / divisor) as u8) as char) };
            value %= divisor;
            divisor /= 10;
        }
    } else {
        print("Argument must be a Decimal type!", &[]);
    }
}

fn print_hex(arg: &Argument) {
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
    } else if let Some(inner_val) = arg.into_hexuint() {
        for i in (0..7).rev() {
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
    } else {
        print("Argument must be a Hexadecimal type!", &[]);
    }
}

fn print_uint(arg: &Argument) {
    if let Some(inner_val) = arg.into_uint() {
        let mut value: u32 = *inner_val;
        let mut divisor: u32 = 1;
        while value / divisor > 9 {
            divisor *= 10;
        }

        while divisor > 0 {
            unsafe { putchar(('0' as u8 + (value / divisor) as u8) as char) };
            value %= divisor;
            divisor /= 10;
        }
    } else {
        print("Argument must be a UInt type!", &[]);
    }
}
fn print_string(arg: &Argument) {
    if let Some(inner_val) = arg.into_string() {
        for c in inner_val.chars() {
            unsafe {
                putchar(c);
            }
        }
    } else {
        print("Argument must be a String type!", &[]);
    }
}

// Define types those express memory address.
// Physical memory address
pub type PAddr = u32;
// Virtual memory address
pub type VAddr = u32;

pub fn align_up<T>(value: usize, align: usize) -> usize {
    let remainder = value % align;
    if remainder == 0 {
        value
    } else {
        value + (align - remainder)
    }
}

pub fn is_aligned(value: usize, align: usize) -> bool {
    value % align == 0
}

pub fn strcmp(s1: &str, s2: &str) -> i32 {
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            return if c1 > c2 { 1 } else { -1 };
        }
    }
    0
}

pub const PAGE_SIZE: u32 = 4096;
