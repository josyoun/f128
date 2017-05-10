#![feature(link_args)]
#![feature(i128_type)]
#![feature(i128)]
#![feature(zero_one)]
#![feature(libc)]
#![feature(concat_idents)]
#![feature(use_extern_macros)]
extern crate libc;
use std::f64;
mod ffi;
mod f128_t;

pub use f128_t::f128;

fn main() {
    let mut x = f128::new(123);
    let a: f128 = f64::NAN.into();
    println!("{:x}", a.into_inner());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let x = unsafe { add(0, 0) };
        println!("{:x}", x);
    }
}

