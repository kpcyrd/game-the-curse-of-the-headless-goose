use crate::gfx;
use embedded_graphics::{
    Drawable,
    mono_font::MonoTextStyle,
    pixelcolor::Rgb666,
    prelude::{Dimensions, DrawTarget, Point, Size, Transform},
    primitives::{Rectangle, StyledDrawable},
    text::Text,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

impl Direction {
    pub const fn to_delta(&self, increment: i32) -> Point {
        Point::new(
            0,
            increment
                * match self {
                    Direction::Up => -1,
                    Direction::Down => 1,
                },
        )
    }
}

pub struct Glider<T> {
    primitive: T,
    direction: Direction,
    distance: u32,
    increment: i32,
    clear_previous: bool,
}

impl<T: Transform> Glider<T> {
    pub const fn new(primitive: T, direction: Direction, distance: u32) -> Self {
        Self {
            primitive,
            direction,
            distance,
            increment: 1,
            clear_previous: false,
        }
    }

    pub const fn finished(&self) -> bool {
        self.distance == 0
    }

    pub fn step(&mut self) {
        if self.finished() {
            return;
        }
        let increment = self.increment.min(self.distance as i32);

        let delta = self.direction.to_delta(increment);
        self.primitive = self.primitive.translate(delta);

        self.clear_previous = true;
        self.distance = self.distance.saturating_sub(increment.unsigned_abs());
    }
}

impl<T: Drawable<Color = Rgb666> + Dimensions> Drawable for Glider<T> {
    type Color = T::Color;
    type Output = T::Output;

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let output = self.primitive.draw(target)?;
        if self.clear_previous {
            let dimensions = self.primitive.bounding_box();

            let mut point = dimensions.top_left + self.direction.to_delta(-self.increment);
            if self.direction == Direction::Up {
                // If the text scrolls up, we need to clear below the bounding box
                point += Point::new(0, dimensions.size.height as i32)
            }

            Rectangle::new(
                point,
                Size::new(
                    self.primitive.bounding_box().size.width,
                    self.increment as u32,
                ),
            )
            .draw_styled(&gfx::BLACK_STYLE, target)?;
        }
        Ok(output)
    }
}

pub type TextGlider = Glider<Text<'static, MonoTextStyle<'static, Rgb666>>>;

impl TextGlider {
    pub const fn empty_text() -> Self {
        Glider::new(
            Text::new("", Point::zero(), gfx::TEXT_STYLE),
            Direction::Down,
            0,
        )
    }
}
