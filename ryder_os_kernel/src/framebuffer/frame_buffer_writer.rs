use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use crate::framebuffer::color::RGBColor;

pub struct FrameBufferWriter {
    buffer: &'static mut [u8],
    info: FrameBufferInfo,
}

impl FrameBufferWriter {
    pub fn new(frame_buffer: &'static mut FrameBuffer) -> Self {
        let info = frame_buffer.info();
        let buffer = frame_buffer.buffer_mut();

        Self {
            buffer,
            info
        }
    }

    pub fn height(&self) -> usize {
        self.info.height
    }

    pub fn width(&self) -> usize {
        self.info.width
    }

    pub fn clear(
        &mut self,
        rgb_color: Option<RGBColor>,
    ) {
        for y in 0..self.info.height {
            for x in 0..self.info.width {
                self.write_pixel(x, y, rgb_color.unwrap_or_default())
            }
        }
    }

    pub fn write_pixel(
        &mut self,
        x: usize,
        y: usize,
        rgb_color: RGBColor
    ) {
        // Outside Framebuffer bounds
        if x >= self.info.width || y >= self.info.height {
            return;
        }

        let offset = (y * self.info.stride + x) * self.info.bytes_per_pixel;
        let pixel = match self.info.pixel_format {
            PixelFormat::Rgb => [
                rgb_color.red,
                rgb_color.green,
                rgb_color.blue,
                0
            ],
            PixelFormat::Bgr => [
                rgb_color.blue,
                rgb_color.green,
                rgb_color.red,
                0
            ],
            PixelFormat::U8 => [
                grayscale(rgb_color),
                0,
                0,
                0,
            ],
            _ => return,
        };

        let byte_count = self.info.bytes_per_pixel.min(pixel.len());
        let end = offset + byte_count;
        if end > self.buffer.len() {
            return;
        }

        self.buffer[offset..end]
            .copy_from_slice(&pixel[..byte_count]);
    }

    pub fn scroll_up(&mut self, pixel_rows: usize, background_color: RGBColor) {
        if pixel_rows == 0 {
            return;
        }
        if pixel_rows >= self.info.height {
            self.clear(Some(background_color));
            return;
        }

        let bytes_per_row = self.info.stride * self.info.bytes_per_pixel;
        let start = pixel_rows * bytes_per_row;
        let end = self.info.height * bytes_per_row;
        self.buffer.copy_within(start..end, 0);

        let clear_start_y = self.info.height - pixel_rows;
        for y in clear_start_y..self.info.height {
            for x in 0..self.info.width {
                self.write_pixel(x, y, background_color);
            }
        }
    }
}

fn grayscale(rgb_color: RGBColor) -> u8 {
    let red = u16::from(rgb_color.red);
    let green = u16::from(rgb_color.green);
    let blue = u16::from(rgb_color.blue);

    ((red + green + blue) / 3 ) as u8
}