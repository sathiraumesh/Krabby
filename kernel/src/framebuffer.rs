use bootloader_api::info::FrameBufferInfo;
use noto_sans_mono_bitmap::{get_raster, FontWeight, RasterHeight};

const FONT_WEIGHT: FontWeight = FontWeight::Regular;
const FONT_HEIGHT: RasterHeight = RasterHeight::Size16;

pub struct FrameBufferWriter {
	buffer: &'static mut [u8],
	info: FrameBufferInfo,
	x: usize,
	y: usize,
}

const BORDER: usize = 4;

impl FrameBufferWriter {
    pub fn new(buffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut writer = Self {
            buffer,
            info,
            x: BORDER,
            y: BORDER,
        };
        writer.clear();
        writer
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
        self.x = BORDER;
        self.y = BORDER;
    }

		fn put_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let offset = (y * self.info.stride + x) * bytes_per_pixel;
        let pixel = &mut self.buffer[offset..offset + bytes_per_pixel];
        pixel[0] = intensity;
        pixel[1] = intensity;
        pixel[2] = intensity;
    }

    fn newline(&mut self) {
        self.x = BORDER;
        self.y += FONT_HEIGHT.val() + 2;
        if self.y + FONT_HEIGHT.val() >= self.info.height {
            self.clear();
        }
    }

    fn write_char(&mut self, c: char) {
        if c == '\n' {
            self.newline();
            return;
        }
        let glyph = get_raster(c, FONT_WEIGHT, FONT_HEIGHT)
            .or_else(|| get_raster('?', FONT_WEIGHT, FONT_HEIGHT))
            .unwrap();
        if self.x + glyph.width() + BORDER >= self.info.width {
            self.newline();
        }
        for (row, line) in glyph.raster().iter().enumerate() {
            for (col, &intensity) in line.iter().enumerate() {
                self.put_pixel(self.x + col, self.y + row, intensity);
            }
        }
        self.x += glyph.width();
    }
}

impl core::fmt::Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}


