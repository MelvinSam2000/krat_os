use fdt::Fdt;

use alloc::string::String;
use alloc::format;

pub fn init(fdt_ptr: u64) {
    
    let fdt = unsafe { Fdt::from_ptr(fdt_ptr as *const u8).unwrap() };
    
    print_fdt(&fdt);
}

/// Print device tree information.
#[cfg(debug_assertions)]
fn print_fdt(fdt: &Fdt) {

    let soc = fdt.find_node("/soc");
    if let Some(soc) = soc {
        let mut msg = String::from("FDT Nodes:\n");
        for child in soc.children() {
            msg += &format!("\t{}\n", child.name);
        }
        log::info!("{}", msg);
    }
}