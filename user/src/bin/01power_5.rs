#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{get_time};
const LEN: usize = 100;
use user_lib::logging;
use log::*;

#[no_mangle]
fn main() -> i32 {
    //hy
    logging::init();
    let appbegin = get_time();
    debug!("the 01_APP start at  {}ms on user",appbegin);
    
    let p = 5u64;
    let m = 998244353u64;
    let iter: usize = 140000;
    let mut s = [0u64; LEN];
    let mut cur = 0usize;
    s[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else { cur + 1 };
        s[next] = s[cur] * p % m;
        cur = next;
        if i % 10000 == 0 {
            println!("power_5 [{}/{}]", i, iter);
        }
    }
    println!("{}^{} = {}(MOD {})", p, iter, s[cur], m);
    println!("Test power_5 OK!");
    //hy
    let append = get_time();
    debug!("the 01_APP end at  {}ms on user",append);
    0
}
