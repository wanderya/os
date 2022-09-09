#![no_std]
#![no_main]
#![feature(panic_info_message)]


#[macro_use]
mod console;
mod lang_items;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn start() -> ! {
    clear_bss();
    println!("Hello OS!");
    panic!("shutdown machine.");
}


/// 对bss段清零。 在任何被分配到.bss段的全局变量之前，我们须确保.bss已被清零
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|x| {
        unsafe { (x as *mut u8).write_volatile(0) }
    });
}
