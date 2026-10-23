#![no_std]
#![no_main]

mod framebuffer;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use framebuffer::FrameBufferWriter;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        let info = fb.info();
        let mut writer = FrameBufferWriter::new(fb.buffer_mut(), info);
        writer.draw_image_centered();
        writer.write_centered(8, "Krabby");
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
