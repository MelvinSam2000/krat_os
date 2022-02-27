use core::arch::asm;
use alloc::collections::vec_deque::VecDeque;
use alloc::boxed::Box;

use crate::proc::*;
use crate::trap::TrapFrame;

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

        // create init process
        let mut pidle = Process::spawn(0);
        pidle.context.pc = (idle as *const ()) as u64;
        SCHED.tasks.as_mut()?.push_back(Box::new(pidle));

        // create p1 and p2 dummy processes
        let mut p1 = Process::spawn(1);
        SCHED.tasks.as_mut()?.push_back(Box::new(p1));
        let mut p2 = Process::spawn(2);
        SCHED.tasks.as_mut()?.push_back(Box::new(p2));
        
        // begin timer interrupts
        asm! {
            // add time
            "csrr   t0, time",
            "li     t1, 1000000",
            "add    t0, t0, t1",
            // call sbi sbi_set_time(time + 1000000)
            "li     a6, 0",
            "li     a7, 0x54494d45",
            "mv     a0, t0",
            "ecall",
        }

        log::info!("Scheduler initialized.");
        loop {}
    }
}

pub fn sched(trap_frame: &mut TrapFrame) -> Option<()> {

    unsafe {
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
    
        // fire timer interrupt
        asm! {
            // add time
            "csrr   t0, time",
            "li     t1, 5000000",
            "add    t0, t0, t1",
            // call sbi sbi_set_time(time + 1000000)
            "li     a6, 0",
            "li     a7, 0x54494d45",
            "mv     a0, t0",
            "ecall",
        }

        log::info!("Scheduling process {}", SCHED.tasks.as_mut()?.front()?.pid);
        
        // switch to new task
        let mut task = SCHED.tasks.as_mut()?.front_mut()?;
        task.state = ProcessState::Running;
        // *trap_frame = task.context;
        Some(())
    }
}

unsafe fn idle() -> ! {
    loop { asm!("wfi") };
}