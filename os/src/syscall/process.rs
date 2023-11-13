//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    mm::{write_val_translated, MapPermission, MemorySetOccupancy, VPNRange, VirtAddr},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next,
        suspend_current_and_run_next, TaskStatus, TASK_MANAGER,
    },
    timer::Instant,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");

    let val = {
        let us = Instant::now().as_micros();
        TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        }
    };

    write_val_translated(&val, current_user_token(), ts);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");

    let task_info = TASK_MANAGER.with_tcb_of_current(|tcb| TaskInfo {
        status: tcb.task_status,
        syscall_times: tcb.syscall_times,
        time: tcb
            .task_start_instant
            .as_ref()
            .unwrap()
            .elapsed()
            .as_millis(),
    });

    write_val_translated(&task_info, current_user_token(), ti);
    0
}

pub fn sys_mmap(start: usize, mut len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap");

    // Not aligned by page.
    if start & ((1 << 12) - 1) != 0 {
        return -1;
    }
    let end = VirtAddr::from(start + len).ceil();
    let start = VirtAddr::from(start).floor();
    let vpn_range = VPNRange::new(start, end);

    // Meaningless bits set.
    if prot & !0b111 != 0 {
        return -1;
    }
    // Memory without any permission.
    if prot == 0 {
        return -1;
    }

    TASK_MANAGER.with_tcb_of_current(|tcb| {
        if tcb.memory_set.check_occupancy(vpn_range) != MemorySetOccupancy::Free {
            -1
        } else {
            let mut permission = unsafe { MapPermission::from_bits_unchecked((prot as u8) << 1) };
            permission.set(MapPermission::U, true);
            tcb.memory_set
                .insert_framed_area(start.into(), end.into(), permission);
            0
        }
    })
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");

    // Not aligned by page.
    if start & ((1 << 12) - 1) != 0 {
        return -1;
    }
    let end = VirtAddr::from(start + len).ceil();
    let start = VirtAddr::from(start).floor();
    let vpn_range = VPNRange::new(start, end);

    TASK_MANAGER.with_tcb_of_current(|tcb| {
        if tcb.memory_set.check_occupancy(vpn_range) != MemorySetOccupancy::Occupied {
            return -1;
        }

        if tcb.memory_set.remove_area(vpn_range) {
            0
        } else {
            1
        }
    })
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
