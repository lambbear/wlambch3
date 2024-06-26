//! Types related to task management
//hy?
#![allow(missing_docs)]

use super::TaskContext;
//hy:导入tainfo模块
use crate::syscall::taInfo::{SyscallInfo,MAX_SYSCALL_NUM} ;

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    //hy:新增控制信息
    pub task_begin :usize,
    pub task_stop : usize, // 任务开始的时间
    pub task_continue : usize,   // 任务执行的时间
    pub sys_statistics:[SyscallInfo; MAX_SYSCALL_NUM],
    pub id:usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
