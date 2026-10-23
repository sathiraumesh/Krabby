use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight};

const FONT_WEIGHT: FontWeight = FontWeight::Regular;
const FONT_HEIGHT: RasterHeight = RasterHeight::Size16;

const LOGO_WIDTH: usize = 128;
const LOGO_HEIGHT: usize = 128;

static LOGO: &[u8] = include_bytes!("logo.rgb");

static FRAMES: [&[u8]; 4] = [
    include_bytes!("logo_0.rgb"),
    include_bytes!("logo_1.rgb"),
    include_bytes!("logo_2.rgb"),
    include_bytes!("logo_3.rgb"),
];

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

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
        self.x = BORDER;
        self.y = BORDER;
    }

    fn put_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let offset = (y * self.info.stride + x) * bytes_per_pixel;
        let pixel = &mut self.buffer[offset..offset + bytes_per_pixel];
        match self.info.pixel_format {
            PixelFormat::Rgb => {
                pixel[0] = r;
                pixel[1] = g;
                pixel[2] = b;
            }
            PixelFormat::Bgr => {
                pixel[0] = b;
                pixel[1] = g;
                pixel[2] = r;
            }
            _ => {
                pixel[0] = r;
            }
        }
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
                self.put_pixel(self.x + col, self.y + row, intensity, intensity, intensity);
            }
        }
        self.x += glyph.width();
    }

    pub fn write_centered(&mut self, offset_y: usize, text: &str) {
        let char_width = get_raster_width(FONT_WEIGHT, FONT_HEIGHT);
        let text_width = text.chars().count() * char_width;
        let x = (self.info.width - text_width) / 2;
        let y = self.y + offset_y;
        self.set_cursor(x, y);
        for c in text.chars() {
            self.write_char(c);
        }
    }

    pub fn draw_image(&mut self, frame: &[u8], x0: usize, y0: usize) {
        for y in 0..LOGO_HEIGHT {
            for x in 0..LOGO_WIDTH {
                let i = (y * LOGO_WIDTH + x) * 3;
                self.put_pixel(x0 + x, y0 + y, frame[i], frame[i + 1], frame[i + 2]);
            }
        }
    }

    pub fn draw_image_centered(&mut self, frame: usize) {
        let x0 = (self.info.width - LOGO_WIDTH) / 2;
        let y0 = (self.info.height - LOGO_HEIGHT) / 2;
        self.draw_image(FRAMES[frame], x0, y0);
        self.set_cursor(BORDER, y0 + LOGO_HEIGHT);
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
