use fdt::Fdt;

use crate::uart_print;

pub fn init(fdt_ptr: u64) {
    
    let fdt = unsafe { Fdt::from_ptr(fdt_ptr as *const u8).unwrap() };
    
    print_fdt(&fdt);
}

/// Print device tree information.
/// Used example from fdt crate.
#[cfg(debug_assertions)]
fn print_fdt(fdt: &Fdt) {
    
    uart_print!("This is a devicetree representation of a {}\n", fdt.root().model());
    uart_print!("...which is compatible with at least: {}\n", fdt.root().compatible().first());
    uart_print!("...and has {} CPU(s)", fdt.cpus().count());
    uart_print!(
        "...and has at least one memory location at: {:#X}\n\n",
        fdt.memory().regions().next().unwrap().starting_address as usize
    );

    let chosen = fdt.chosen();
    if let Some(bootargs) = chosen.bootargs() {
        uart_print!("The bootargs are: {:?}\n", bootargs);
    }

    if let Some(stdout) = chosen.stdout() {
        uart_print!("It would write stdout to: {}\n", stdout.name);
    }

    let soc = fdt.find_node("/soc");
    uart_print!("Does it have a `/soc` node? {}\n", if soc.is_some() { "yes" } else { "no" });
    if let Some(soc) = soc {
        uart_print!("...and it has the following children:\n");
        for child in soc.children() {
            uart_print!("    {}\n", child.name);
        }
    }
}