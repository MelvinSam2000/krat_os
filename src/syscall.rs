pub fn do_syscall(code: u64, args: &[u64]) -> u64 {
    match code {
        // exit
        0 => exit(args[0]),
        _ => panic!("Unknown syscall code: {}", code),
    }
}

fn exit(pid: u64) -> u64 {
    log::info!("Proc:{} called the exit syscall!", pid);
    0
}
