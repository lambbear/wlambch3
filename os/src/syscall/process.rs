//! Process management syscalls
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::get_time_ms;

//hy:导入任务信息模块
use super::taInfo::{TaskInfo,get_task_info};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time in milliseconds
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

///hy:新增系统调用
pub fn sys_task_info(tsk :&mut TaskInfo) ->isize{
    // println!("sys_task_info");
    get_task_info(tsk);
    0
}
