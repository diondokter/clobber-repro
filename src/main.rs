#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub fn show(val: u32) {
    unsafe {
        core::arch::asm!(
            "mov r0, {exp}",
            exp = in(reg) val,
            out("r0") _,
            clobber_abi("C"),
        )
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
