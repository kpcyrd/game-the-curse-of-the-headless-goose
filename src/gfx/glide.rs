use crate::gfx;
use arrayvec::ArrayString;
use embedded_graphics::{
    Drawable,
    mono_font::MonoTextStyle,
    pixelcolor::Rgb666,
    prelude::{Dimensions, DrawTarget, Point, Size},
    primitives::{Rectangle, StyledDrawable},
    text::Text,
};

// The buffer size for the activity glider text
const GLIDER_WIDTH: usize = 13;

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

pub struct Glider {
    buf: ArrayString<GLIDER_WIDTH>,
    point: Point,
    style: MonoTextStyle<'static, Rgb666>,

    direction: Direction,
    distance: u32,
    increment: i32,
    clear_previous: bool,
    clear_all: bool,
}

impl Glider {
    pub const fn buf() -> ArrayString<GLIDER_WIDTH> {
        ArrayString::new_const()
    }

    pub const fn new(
        buf: ArrayString<GLIDER_WIDTH>,
        point: Point,
        style: MonoTextStyle<'static, Rgb666>,
        direction: Direction,
        distance: u32,
    ) -> Self {
        Self {
            buf,
            point,
            style,
            direction,
            distance,
            increment: 1,
            clear_previous: false,
            clear_all: false,
        }
    }

    pub const fn finished(&self) -> bool {
        self.distance == 0 && self.clear_all
    }

    pub fn step(&mut self) -> bool {
        if self.distance == 0 && !self.clear_all {
            self.clear_all = true;
        } else if self.finished() {
            return false;
        }
        let increment = self.increment.min(self.distance as i32);

        let delta = self.direction.to_delta(increment);
        self.point += delta;

        self.clear_previous = true;
        self.distance = self.distance.saturating_sub(increment.unsigned_abs());

        true
    }
}

impl Drawable for Glider {
    type Color = Rgb666;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let primitive = Text::new(&self.buf, self.point, self.style);
        let dimensions = primitive.bounding_box();

        if self.clear_all {
            Rectangle::new(
                dimensions.top_left,
                Size::new(
                    dimensions.size.width,
                    dimensions.size.height + self.increment as u32,
                ),
            )
            .draw_styled(&gfx::BLACK_STYLE, target)?;
        } else {
            primitive.draw(target)?;
        }

        if self.clear_previous {
            let mut point = dimensions.top_left + self.direction.to_delta(-self.increment);
            if self.direction == Direction::Up {
                // If the text scrolls up, we need to clear below the bounding box
                point += Point::new(0, dimensions.size.height as i32)
            }

            Rectangle::new(
                point,
                Size::new(dimensions.size.width, self.increment as u32),
            )
            .draw_styled(&gfx::BLACK_STYLE, target)?;
        }

        Ok(())
    }
}
