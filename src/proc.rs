use crate::trap::TrapFrame;

#[derive(PartialEq)]
pub enum ProcessState {
    Created,
    Ready,
    Running,
    Blocked,
    Dead,
}

pub struct Process {
    pub pid: u64,
    pub pc: u64,
    pub context: TrapFrame,
    pub state: ProcessState,
}

impl Process {

    pub fn spawn(pid: u64) -> Self {
        let mut proc = Process {
            pid,
            pc: 0,
            context: Default::default(),
            state: ProcessState::Created,
        };
        proc.state = ProcessState::Ready;
        proc
    }
}
