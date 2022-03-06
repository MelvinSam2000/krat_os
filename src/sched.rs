use core::arch::asm;
use alloc::collections::vec_deque::VecDeque;
use alloc::boxed::Box;

use crate::proc::*;
use crate::trap::TrapFrame;
use crate::riscv::timer_int;

const SCHED_TIME_SLICE_USEC: usize = 1000000;

// Round Robin scheduler
struct Scheduler {
    tasks: Option<VecDeque<Box<Process>>>,
}

static mut SCHED: Scheduler = Scheduler {
    tasks: None,
};


pub fn init() -> Option<()> {

    unsafe {
        SCHED.tasks = Some(VecDeque::new());
    }
    
    // begin timer interrupts
    timer_int(SCHED_TIME_SLICE_USEC);

    log::info!("Scheduler initialized.");
    loop {}
    
}

pub fn sched(trap_frame: &mut TrapFrame) -> Option<()> {

    unsafe {

        if SCHED.tasks.as_ref()?.is_empty() {
            log::info!("No tasks to schedule...");
            timer_int(SCHED_TIME_SLICE_USEC);
            return Some(());
        }

        // stop current process
        let mut task = SCHED.tasks.as_mut()?.front_mut()?;
        task.state = ProcessState::Ready;
        task.context = *trap_frame;
        SCHED.tasks.as_mut()?.rotate_left(1);

        // get next ready process
        while SCHED.tasks.as_ref()?.front()?.state != ProcessState::Ready {
            let task = SCHED.tasks.as_ref()?.front()?;
            match task.state {
                ProcessState::Dead => {
                    let _ = SCHED.tasks.as_mut()?.pop_front()?;
                },
                _ => {}
            }
            SCHED.tasks.as_mut()?.rotate_left(1);
        }
    

        log::info!("Scheduling process {}", SCHED.tasks.as_mut()?.front()?.pid);
        
        // switch to new task
        let mut task = SCHED.tasks.as_mut()?.front_mut()?;
        task.state = ProcessState::Running;
        // *trap_frame = task.context;

        timer_int(SCHED_TIME_SLICE_USEC);

        Some(())
    }
}