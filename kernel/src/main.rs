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

        writer.draw_image_centered(0);
        writer.write_centered(8, "Krabby");

        let sequence = [0, 1, 2, 1, 0, 3];
        loop {
            for &frame in sequence.iter() {
                writer.draw_image_centered(frame);
                for _ in 0..2_000_000 {
                    core::hint::spin_loop();
                }
            }
        }
    }

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
