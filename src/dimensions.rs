#![allow(dead_code)]

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub type Vector2i = Vector2<i32>;
pub type Vector2f = Vector2<f32>;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Size2<T> {
    width: T,
    height: T,
}

pub type Size2i = Size2<i32>;
pub type Size2f = Size2<f32>;

impl<T: Copy> Size2<T> {
    fn width(&self) -> T {
        self.width
    }

    fn height(&self) -> T {
        self.height
    }
}

impl<T: Copy + std::default::Default + PartialOrd> Size2<T> {
    /// If passing in a negative height or width, will convert to 0.
    pub fn new(width: T, height: T) -> Self {
        Self {
            width: max(width, T::default()),
            height: max(height, T::default()),
        }
    }
}

impl<
        T: Copy
            + std::default::Default
            + std::ops::Sub<Output = T>
            + std::ops::Add<Output = T>
            + PartialOrd,
    > From<Box2<T>> for Size2<T>
{
    fn from(rectangle: Box2<T>) -> Size2<T> {
        Self {
            width: rectangle.width(),
            height: rectangle.height(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Box2<T> {
    // Not public because we want to maintain these invariants:
    // x0 <= x1, y0 <= y1.
    x0: T,
    y0: T,
    x1: T,
    y1: T,
}

pub type Box2i = Box2<i32>;
pub type Box2f = Box2<f32>;

impl<T: Copy + std::default::Default + std::ops::Sub<Output = T>> From<Size2<T>> for Box2<T> {
    fn from(size: Size2<T>) -> Box2<T> {
        Self {
            x0: T::default(),
            y0: T::default(),
            x1: size.width,
            y1: size.height,
        }
    }
}

impl<
        T: Copy
            + std::default::Default
            + std::ops::Sub<Output = T>
            + std::ops::Add<Output = T>
            + PartialOrd,
    > Box2<T>
{
    /// Returns a new rectangle between two corners.
    pub fn new(corner0: Vector2<T>, corner1: Vector2<T>) -> Self {
        Self {
            x0: min(corner0.x, corner1.x),
            y0: min(corner0.y, corner1.y),
            x1: max(corner0.x, corner1.x),
            y1: max(corner0.y, corner1.y),
        }
    }

    pub fn at(top_left_corner: Vector2<T>, size: Size2<T>) -> Self {
        Self {
            x0: top_left_corner.x,
            y0: top_left_corner.y,
            x1: top_left_corner.x + size.width,
            y1: top_left_corner.y + size.height,
        }
    }

    /// Returns the left side of the rectangle (smallest x).
    pub fn left(&self) -> T {
        self.x0
    }

    /// Returns the right side of the rectangle (largest x).
    pub fn right(&self) -> T {
        self.x1
    }

    /// Returns the top side of the rectangle (smallest y).
    pub fn top(&self) -> T {
        self.y0
    }

    /// Returns the bottom side of the rectangle (largest y).
    pub fn bottom(&self) -> T {
        self.y1
    }

    pub fn size(&self) -> Size2<T> {
        Size2 {
            width: self.width(),
            height: self.height(),
        }
    }

    /// Returns the width of the rectangle.
    pub fn width(&self) -> T {
        self.x1 - self.x0
    }

    /// Returns the height of the rectangle.
    pub fn height(&self) -> T {
        self.y1 - self.y0
    }
}

fn min<T: Copy + PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn max<T: Copy + PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size2i() {
        let size2 = Size2i::new(10, 20);
        assert_eq!(size2.width(), 10);
        assert_eq!(size2.height(), 20);

        let size2 = Size2i::new(-5, -3);
        assert_eq!(size2.width(), 0);
        assert_eq!(size2.height(), 0);
    }

    #[test]
    fn test_size2f() {
        let size2 = Size2f::new(20.0, 15.0);
        assert_eq!(size2.width(), 20.0);
        assert_eq!(size2.height(), 15.0);

        let size2 = Size2f::new(-1.0, -3.5);
        assert_eq!(size2.width(), 0.0);
        assert_eq!(size2.height(), 0.0);
    }

    #[test]
    fn test_box2i_with_top_left_and_bottom_right() {
        let box2 = Box2i::new(Vector2i::new(10, 20), Vector2i::new(50, 25));
        assert_eq!(box2.left(), 10);
        assert_eq!(box2.right(), 50);
        assert_eq!(box2.top(), 20);
        assert_eq!(box2.bottom(), 25);

        assert_eq!(box2.size(), Size2::new(40, 5));
        assert_eq!(box2.width(), 40);
        assert_eq!(box2.height(), 5);
    }

    #[test]
    fn test_box2i_with_bottom_right_and_top_left() {
        let box2 = Box2i::new(Vector2i::new(60, 300), Vector2i::new(45, 290));
        assert_eq!(box2.left(), 45);
        assert_eq!(box2.right(), 60);
        assert_eq!(box2.top(), 290);
        assert_eq!(box2.bottom(), 300);

        assert_eq!(box2.size(), Size2::new(15, 10));
        assert_eq!(box2.width(), 15);
        assert_eq!(box2.height(), 10);
    }

    #[test]
    fn test_box2i_with_top_right_and_bottom_left() {
        let box2 = Box2i::new(Vector2i::new(-5, 1000), Vector2i::new(-55, 1007));
        assert_eq!(box2.left(), -55);
        assert_eq!(box2.right(), -5);
        assert_eq!(box2.top(), 1000);
        assert_eq!(box2.bottom(), 1007);

        assert_eq!(box2.size(), Size2::new(50, 7));
        assert_eq!(box2.width(), 50);
        assert_eq!(box2.height(), 7);
    }

    #[test]
    fn test_box2i_with_bottom_left_and_top_right() {
        // TODO:
        let box2 = Box2i::new(Vector2i::new(1, -50), Vector2i::new(101, -100));
        assert_eq!(box2.left(), 1);
        assert_eq!(box2.right(), 101);
        assert_eq!(box2.top(), -100);
        assert_eq!(box2.bottom(), -50);

        assert_eq!(box2.size(), Size2::new(100, 50));
        assert_eq!(box2.width(), 100);
        assert_eq!(box2.height(), 50);
    }
    // TODO: from(Size), at(Corner, Size)
}
