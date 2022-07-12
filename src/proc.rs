use crate::trap::TrapFrame;

#[derive(PartialEq)]
pub enum ProcessState {
    Creating,
    Ready,
    Running,
    Blocked,
    Dead,
}

pub struct Process {
    pub pid: u64,
    pub context: TrapFrame,
    pub state: ProcessState,
}

impl Process {
    pub fn spawn(pid: u64) -> Self {
        let mut proc = Process {
            pid,
            context: Default::default(),
            state: ProcessState::Creating,
        };

        proc.state = ProcessState::Ready;
        proc
    }
}
