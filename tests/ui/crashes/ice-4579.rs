#![allow(clippy::single_match)]

use std::ptr;

fn main() {
    if let Some(_) = Some(0_usize) {
        let s = "012345";
        unsafe { ptr::read(s.as_ptr().offset(1) as *const [u8; 5]) };
    };
}
