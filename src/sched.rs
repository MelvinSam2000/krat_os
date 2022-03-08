use spin::Mutex;
use lazy_static::lazy_static;

use core::arch::asm;
use alloc::collections::vec_deque::VecDeque;

use crate::proc::*;
use crate::trap::TrapFrame;
use crate::riscv::timer_int;

const SCHED_TIME_SLICE_USEC: usize = 1000000;

// Round Robin scheduler
struct Scheduler {
    tasks: VecDeque<Process>,
}

lazy_static! {
    static ref SCHED: Mutex<Scheduler> = {
        Mutex::new(Scheduler { tasks: VecDeque::new() })
    };
}


pub fn init() {
    
    // begin timer interrupts
    timer_int(SCHED_TIME_SLICE_USEC);

    SCHED.lock().tasks.push_front(Process::spawn(0));
    SCHED.lock().tasks.push_front(Process::spawn(1));
    SCHED.lock().tasks.push_front(Process::spawn(2));

    log::info!("Scheduler initialized.");
    loop {}
}

pub fn sched(trap_frame: &mut TrapFrame) {

    let mut sched = SCHED.lock();

    if sched.tasks.is_empty() {
        log::info!("No tasks to schedule...");
        timer_int(SCHED_TIME_SLICE_USEC);
        return;
    }

    // stop current process
    let mut task = sched.tasks.front_mut().unwrap();
    task.state = ProcessState::Ready;
    task.context = *trap_frame;
    sched.tasks.rotate_left(1);

    // get next ready process
    while sched.tasks.front().unwrap().state != ProcessState::Ready {
        let task = sched.tasks.front().unwrap();
        match task.state {
            ProcessState::Dead => {
                let _ = sched.tasks.pop_front();
            },
            _ => {}
        }
        sched.tasks.rotate_left(1);
    }

    if sched.tasks.is_empty() {
        timer_int(SCHED_TIME_SLICE_USEC);
        return;
    }

    // switch to new task
    let mut task = sched.tasks.front_mut().unwrap();
    log::info!("Scheduling process {}", task.pid);
    task.state = ProcessState::Running;
    // *trap_frame = task.context;

    timer_int(SCHED_TIME_SLICE_USEC);
    
}