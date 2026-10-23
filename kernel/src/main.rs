#![no_std]
#![no_main]

mod framebuffer;

use bootloader_api::{entry_point, BootInfo};
use core::fmt::Write;
use core::panic::PanicInfo;
use framebuffer::FrameBufferWriter;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        let info = fb.info();
        let mut writer = FrameBufferWriter::new(fb.buffer_mut(), info);
        writeln!(writer, "Hello from Krabby!").unwrap();
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
