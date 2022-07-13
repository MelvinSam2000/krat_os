pub fn prep_syscall(code: usize, args: &[usize]) -> usize {
    match code {
        // exit
        0 => exit(args[0]),
        _ => panic!("Unknown syscall code: {}", code),
    }
}

fn exit(pid: usize) -> usize {
    log::info!("Proc:{} called the exit syscall!", pid);
    0
}
