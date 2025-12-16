use crate::gfx::BLACK_STYLE;
use core::fmt;
use embedded_graphics::{
    Drawable,
    mono_font::MonoTextStyle,
    pixelcolor::Rgb666,
    prelude::{DrawTarget, Point, Size},
    primitives::{Rectangle, StyledDrawable},
    text::{Baseline, Text},
};

pub struct TextBox<'a> {
    point: Point,
    style: &'a MonoTextStyle<'a, Rgb666>,
    width_chars: usize,
}

impl<'a> TextBox<'a> {
    pub const fn new(
        point: Point,
        style: &'a MonoTextStyle<'a, Rgb666>,
        width_chars: usize,
    ) -> Self {
        Self {
            point,
            style,
            width_chars,
        }
    }

    pub fn render<D: DrawTarget<Color = Rgb666>>(&mut self, display: &mut D, text: &str)
    where
        D::Error: fmt::Debug,
    {
        Text::with_baseline(text, self.point, *self.style, Baseline::Top)
            .draw(display)
            .unwrap();

        self.point += Point::new(
            self.style.font.character_size.width as i32 * text.len() as i32,
            0,
        );
        self.width_chars = self.width_chars.saturating_sub(text.len());
    }

    pub fn clear_remaining<D: DrawTarget<Color = Rgb666>>(self, display: &mut D)
    where
        D::Error: fmt::Debug,
    {
        if self.width_chars == 0 {
            return;
        }

        Rectangle::new(
            self.point,
            Size::new(
                self.style.font.character_size.width * self.width_chars as u32,
                self.style.font.character_size.height,
            ),
        )
        .draw_styled(&BLACK_STYLE, display)
        .unwrap();
    }

    pub fn render_and_clear<D: DrawTarget<Color = Rgb666>, T: AsRef<str>>(
        mut self,
        display: &mut D,
        text: Option<T>,
    ) where
        D::Error: fmt::Debug,
    {
        if let Some(text) = text {
            self.render(display, text.as_ref());
        }
        self.clear_remaining(display);
    }
}
