use core::fmt::Write;
use noto_sans_mono_bitmap::{get_raster, FontWeight, RasterHeight};
use crate::framebuffer::color::RGBColor;
use crate::framebuffer::frame_buffer_writer::FrameBufferWriter;

pub struct ConsoleWriter {
    frame_buffer_writer: FrameBufferWriter,
    cursor_position_x: usize,
    cursor_position_y: usize,
    foreground_color: RGBColor,
    background_color: RGBColor,
}

impl ConsoleWriter {
    pub fn new(mut frame_buffer_writer: FrameBufferWriter) -> Self {
        frame_buffer_writer.clear(None);

        Self {
            frame_buffer_writer,
            cursor_position_x: BORDER_PADDING,
            cursor_position_y: BORDER_PADDING,
            foreground_color: FOREGROUND_COLOR,
            background_color: RGBColor::default(),
        }
    }

    pub fn foreground_color(&mut self, foreground_color: RGBColor) {
        self.foreground_color = foreground_color;
    }

    pub fn background_color(&mut self, background_color: RGBColor) {
        self.background_color = background_color;
    }

    pub fn width(&self) -> usize {
        self.frame_buffer_writer.width()
    }

    pub fn height(&self) -> usize {
        self.frame_buffer_writer.height()
    }

    pub fn clear(&mut self) {
        self.frame_buffer_writer.clear(Some(self.background_color));
        self.cursor_position_x = BORDER_PADDING;
        self.cursor_position_y = BORDER_PADDING;
    }

    pub fn write_character(&mut self, character: char) {
        match character {
            '\n' => self.new_line(),
            '\r' => self.carriage_return(),
            '\t' => self.write_tab(),
            character => self.write_printable_character(character),
        }
    }

    fn write_printable_character(&mut self, character: char) {
        let Some(rasterized_character) = get_raster(
            character,
            FONT_WEIGHT,
            FONT_HEIGHT
        ).or_else(|| get_raster(
            '\u{FFFD}',
            FONT_WEIGHT,
            FONT_HEIGHT,
        )) else {
            return;
        };

        let character_width = rasterized_character.width();
        let character_height = rasterized_character.height();

        if self.cursor_position_x + character_width > self.frame_buffer_writer.width() - BORDER_PADDING {
            self.new_line()
        }
        if self.cursor_position_y + character_height > self.frame_buffer_writer.height() - BORDER_PADDING {
            self.scroll_up()
        }

        for (glyph_y, row) in rasterized_character.raster().iter().enumerate() {
            for (glyph_x, intensity) in row.iter().copied().enumerate() {
                let blended_color = self.foreground_color
                    .blend(self.background_color, intensity);

                self.frame_buffer_writer.write_pixel(
                    self.cursor_position_x + glyph_x,
                    self.cursor_position_y + glyph_y,
                    blended_color,
                )
            }
        }

        self.cursor_position_x += character_width + LETTER_SPACING;
    }

    fn new_line(&mut self) {
        self.cursor_position_x = BORDER_PADDING;
        self.cursor_position_y += self.line_height();

        let bottom = self.frame_buffer_writer.height() - BORDER_PADDING;

        if self.cursor_position_y + self.line_height() > bottom {
            self.scroll_up()
        }
    }

    fn write_tab(&mut self) {
        const TAB_WIDTH: usize = 4;

        for _ in 0..TAB_WIDTH {
            self.write_character(' ');
        }
    }

    fn scroll_up(&mut self) {
        let scroll_amount = self.line_height();
        self.frame_buffer_writer.scroll_up(
            scroll_amount,
            self.background_color
        );

        self.cursor_position_y = self.cursor_position_y.saturating_sub(scroll_amount);
    }

    fn carriage_return(&mut self) {
        self.cursor_position_x = BORDER_PADDING;
    }

    fn line_height(&self) -> usize {
        FONT_HEIGHT.val() + LINE_SPACING
    }
}

impl Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for character in s.chars() {
            self.write_character(character);
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write_character(c);

        Ok(())
    }
}

const BORDER_PADDING: usize = 8;
const FOREGROUND_COLOR: RGBColor = RGBColor::WHITE;
const FONT_WEIGHT: FontWeight = FontWeight::Regular;
const FONT_HEIGHT: RasterHeight = RasterHeight::Size16;
const LETTER_SPACING: usize = 1;
const LINE_SPACING: usize = 2;