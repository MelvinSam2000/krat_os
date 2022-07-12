use alloc::collections::vec_deque::VecDeque;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::proc::*;
use crate::riscv::timer_int;
use crate::trap::TrapFrame;

const SCHED_TIME_SLICE_USEC: usize = 1000000;

// Round Robin scheduler
struct Scheduler {
    tasks: VecDeque<Process>,
}

lazy_static! {
    static ref SCHED: Mutex<Scheduler> = {
        Mutex::new(Scheduler {
            tasks: VecDeque::new(),
        })
    };
}

#[allow(clippy::empty_loop)]
pub fn init() -> ! {
    // Push some tasks
    let mut sched = SCHED.lock();
    let tasks = &mut sched.tasks;
    tasks.push_back(Process::spawn(0));
    tasks.push_back(Process::spawn(1));
    tasks.push_back(Process::spawn(2));
    // Must unlock scheduler mutex before timer interrupts start
    drop(sched);

    // begin timer interrupts
    log::info!("Scheduler initialized.");
    timer_int(SCHED_TIME_SLICE_USEC);

    loop {}
}

pub fn sched(trap_frame: &mut TrapFrame) {
    let mut sched = SCHED.lock();
    let tasks = &mut sched.tasks;

    if tasks.is_empty() {
        log::info!("No tasks to schedule...");
        timer_int(SCHED_TIME_SLICE_USEC);
        return;
    }

    // stop current process
    let mut task = tasks.front_mut().unwrap();
    task.state = ProcessState::Ready;
    task.context = *trap_frame;
    tasks.rotate_left(1);

    // get next ready process
    while tasks.front().unwrap().state != ProcessState::Ready {
        let task = tasks.front().unwrap();
        match task.state {
            ProcessState::Dead => {
                tasks.pop_front();
            }
            _ => unimplemented!(),
        }
        tasks.rotate_left(1);
    }

    if tasks.is_empty() {
        timer_int(SCHED_TIME_SLICE_USEC);
        return;
    }

    // switch to new task
    let mut task = tasks.front_mut().unwrap();
    log::info!("Scheduling process {}", task.pid);
    task.state = ProcessState::Running;
    // *trap_frame = task.context;

    timer_int(SCHED_TIME_SLICE_USEC);
}
