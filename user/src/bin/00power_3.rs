#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

//hy
use log::debug;
use user_lib::{get_time, yield_};
use user_lib::task_info;
use user_lib::logging;
use user_lib::taInfo::*;
use user_lib::syscall::{SYSCALL_EXIT,SYSCALL_GET_TIME,SYSCALL_TASK_INFO,SYSCALL_WRITE,SYSCALL_YIELD};

const LEN: usize = 100;

//hy:为该程序实例化taskInfo
static  mut y : TaskInfo =TaskInfo{
    id : 0,
    status : TaskStatus::UnInit,
    call : [SyscallInfo{sysid :0,times : 0 }; MAX_SYSCALL_NUM],
    time : 0
};

#[no_mangle]
fn main() -> i32 {
    //hy:打印起始时间
    logging::init();
    let appbegin = get_time();
    debug!("the 00_APP start at  {}ms on user",appbegin);
    
    let p = 3u64;
    let m = 998244353u64;
    let iter: usize = 200000;
    let mut s = [0u64; LEN];
    let mut cur = 0usize;
    s[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else { cur + 1 };
        s[next] = s[cur] * p % m;
        cur = next;
        if i % 10000 == 0 {
            println!("power_3 [{}/{}]", i, iter);
        }
    }
    println!("{}^{} = {}(MOD {})", p, iter, s[cur], m);
    println!("Test power_3 OK!");
    
    //hy 打印结束时间
    let append= get_time();
    debug!("the 0_APP end at  {}ms on user",append);
    
    //hy     

    unsafe{task_info(&y as *const TaskInfo as usize)};
    println!("--------------------------00user------------------------------------");
    unsafe{ 
        println!("app id                    =====> {}",y.id);
        println!("app status                =====> {:?}",y.status);
        println!("app time                  =====> {}",y.time);
        println!("SYSCALL_WRITE times       =====> {:?}",y.call[SYSCALL_WRITE]);
        println!("SYSCALL_EXIT times        =====> {:?}",y.call[SYSCALL_EXIT]);
        println!("SYSCALL_YIELD times       =====> {:?}",y.call[SYSCALL_YIELD]);
        println!("SYSCALL_GET_TIME times    =====> {:?}",y.call[SYSCALL_GET_TIME]);
        println!("SYSCALL_TASK_INFO times   =====> {:?}",y.call[SYSCALL_TASK_INFO]);
    };

    0
}
