#![allow(dead_code)]

use bytemuck::{Pod, Zeroable};

#[repr(C, packed(1))] // Type `T` will only be `i32` or `f32` so packing tightly will be fine.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Pod, Zeroable)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

pub type Vector2i = Vector2<i32>;
pub type Vector2f = Vector2<f32>;

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[repr(C, packed(1))] // Type `T` will only be `i32` or `f32` so packing tightly will be fine.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Pod, Zeroable)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vector3i = Vector3<i32>;
pub type Vector3f = Vector3<f32>;

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[repr(C, packed(1))] // Type `T` will only be `i32` or `f32` so packing tightly will be fine.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Pod, Zeroable)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

pub type Vector4i = Vector4<i32>;
pub type Vector4f = Vector4<f32>;

impl<T> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

#[repr(C, packed(1))] // Type `T` will only be `i32` or `f32` so packing tightly will be fine.
#[derive(Copy, Clone, PartialEq, Debug, Default, Pod, Zeroable)]
pub struct Matrix4<T: Zero + One> {
    pub x: Vector4<T>,
    pub y: Vector4<T>,
    pub z: Vector4<T>,
    pub w: Vector4<T>,
}

impl<T: Zero + One> Matrix4<T> {
    pub fn new(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>, w: Vector4<T>) -> Self {
        Self { x, y, z, w }
    }

    pub fn identity() -> Matrix4<T> {
        Matrix4::new(
            Vector4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vector4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }
}

pub type Matrix4i = Matrix4<i32>;
pub type Matrix4f = Matrix4<f32>;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Size2<T> {
    width: T,
    height: T,
}

pub type Size2i = Size2<i32>;
pub type Size2f = Size2<f32>;

impl<T: Copy> Size2<T> {
    pub fn width(&self) -> T {
        self.width
    }

    pub fn height(&self) -> T {
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

impl<
        T: Copy
            + std::default::Default
            + std::ops::Sub<Output = T>
            + std::ops::Add<Output = T>
            + PartialOrd,
    > Box2<T>
{
    /// Returns a new rectangle between two corners.
    /// The corners can be any of top-left, top-right, bottom-left, bottom-right,
    /// as long as all four dimensions (left, right, top, bottom) are included.
    pub fn new(corner0: Vector2<T>, corner1: Vector2<T>) -> Self {
        Self {
            x0: min(corner0.x, corner1.x),
            y0: min(corner0.y, corner1.y),
            x1: max(corner0.x, corner1.x),
            y1: max(corner0.y, corner1.y),
        }
    }

    /// Returns a new rectangle with a specified top-left corner
    /// and the given size.
    pub fn at(top_left_corner: Vector2<T>, size: Size2<T>) -> Self {
        Self {
            x0: top_left_corner.x,
            y0: top_left_corner.y,
            x1: top_left_corner.x + size.width,
            y1: top_left_corner.y + size.height,
        }
    }

    /// Returns a new rectangle as the intersection of this box with another;
    /// if the two boxes don't intersect, then this will return a zero rectangle.
    pub fn intersect(&self, other: Box2<T>) -> Self {
        let x0 = max(self.x0, other.x0);
        let x1 = min(self.x1, other.x1);
        let y0 = max(self.y0, other.y0);
        let y1 = min(self.y1, other.y1);
        if x0 < x1 && y0 < y1 {
            Self { x0, y0, x1, y1 }
        } else {
            Self::default()
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

pub trait One {
    fn one() -> Self;
}

impl One for i32 {
    fn one() -> Self {
        1
    }
}

impl One for f32 {
    fn one() -> Self {
        1.0
    }
}

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0
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
        let box2 = Box2i::new(Vector2::new(10, 20), Vector2::new(50, 25));
        assert_eq!(box2.left(), 10);
        assert_eq!(box2.right(), 50);
        assert_eq!(box2.top(), 20);
        assert_eq!(box2.bottom(), 25);

        assert_eq!(box2.size(), Size2::new(40, 5));
        assert_eq!(box2.width(), 40);
        assert_eq!(box2.height(), 5);
    }

    #[test]
    fn test_box2f_with_top_left_and_bottom_right() {
        let box2 = Box2f::new(Vector2::new(10.5, 20.0), Vector2::new(50.5, 25.0));
        assert_eq!(box2.left(), 10.5);
        assert_eq!(box2.right(), 50.5);
        assert_eq!(box2.top(), 20.0);
        assert_eq!(box2.bottom(), 25.0);

        assert_eq!(box2.size(), Size2::new(40.0, 5.0));
        assert_eq!(box2.width(), 40.0);
        assert_eq!(box2.height(), 5.0);
    }

    #[test]
    fn test_box2i_with_bottom_right_and_top_left() {
        let box2 = Box2i::new(Vector2::new(60, 300), Vector2::new(45, 290));
        assert_eq!(box2.left(), 45);
        assert_eq!(box2.right(), 60);
        assert_eq!(box2.top(), 290);
        assert_eq!(box2.bottom(), 300);

        assert_eq!(box2.size(), Size2::new(15, 10));
        assert_eq!(box2.width(), 15);
        assert_eq!(box2.height(), 10);
    }

    #[test]
    fn test_box2f_with_bottom_right_and_top_left() {
        let box2 = Box2f::new(Vector2::new(60.5, 300.0), Vector2::new(45.0, 290.0));
        assert_eq!(box2.left(), 45.0);
        assert_eq!(box2.right(), 60.5);
        assert_eq!(box2.top(), 290.0);
        assert_eq!(box2.bottom(), 300.0);

        assert_eq!(box2.size(), Size2::new(15.5, 10.0));
        assert_eq!(box2.width(), 15.5);
        assert_eq!(box2.height(), 10.0);
    }

    #[test]
    fn test_box2i_with_top_right_and_bottom_left() {
        let box2 = Box2i::new(Vector2::new(-5, 1000), Vector2::new(-55, 1007));
        assert_eq!(box2.left(), -55);
        assert_eq!(box2.right(), -5);
        assert_eq!(box2.top(), 1000);
        assert_eq!(box2.bottom(), 1007);

        assert_eq!(box2.size(), Size2::new(50, 7));
        assert_eq!(box2.width(), 50);
        assert_eq!(box2.height(), 7);
    }

    #[test]
    fn test_box2f_with_top_right_and_bottom_left() {
        let box2 = Box2f::new(Vector2::new(-5.0, 1000.0), Vector2::new(-55.0, 1007.5));
        assert_eq!(box2.left(), -55.0);
        assert_eq!(box2.right(), -5.0);
        assert_eq!(box2.top(), 1000.0);
        assert_eq!(box2.bottom(), 1007.5);

        assert_eq!(box2.size(), Size2::new(50.0, 7.5));
        assert_eq!(box2.width(), 50.0);
        assert_eq!(box2.height(), 7.5);
    }

    #[test]
    fn test_box2i_with_bottom_left_and_top_right() {
        let box2 = Box2i::new(Vector2::new(1, -200), Vector2::new(101, -250));
        assert_eq!(box2.left(), 1);
        assert_eq!(box2.right(), 101);
        assert_eq!(box2.top(), -250);
        assert_eq!(box2.bottom(), -200);

        assert_eq!(box2.size(), Size2::new(100, 50));
        assert_eq!(box2.width(), 100);
        assert_eq!(box2.height(), 50);
    }

    #[test]
    fn test_box2f_with_bottom_left_and_top_right() {
        let box2 = Box2f::new(Vector2::new(1.0, -200.5), Vector2::new(101.0, -250.0));
        assert_eq!(box2.left(), 1.0);
        assert_eq!(box2.right(), 101.0);
        assert_eq!(box2.top(), -250.0);
        assert_eq!(box2.bottom(), -200.5);

        assert_eq!(box2.size(), Size2::new(100.0, 49.5));
        assert_eq!(box2.width(), 100.0);
        assert_eq!(box2.height(), 49.5);
    }

    #[test]
    fn test_box2i_at() {
        let box2 = Box2i::at(Vector2::new(-3, 7), Size2::new(123, 45));
        assert_eq!(box2.left(), -3);
        assert_eq!(box2.right(), 120);
        assert_eq!(box2.top(), 7);
        assert_eq!(box2.bottom(), 52);

        assert_eq!(box2.size(), Size2::new(123, 45));
        assert_eq!(box2.width(), 123);
        assert_eq!(box2.height(), 45);
    }

    #[test]
    fn test_box2f_at() {
        let box2 = Box2f::at(Vector2::new(3.4, -6.5), Size2::new(123.4, 56.7));
        assert_eq!(box2.left(), 3.4);
        assert_eq!(box2.right(), 126.8);
        assert_eq!(box2.top(), -6.5);
        assert_eq!(box2.bottom(), 50.2);

        assert_eq!(box2.size(), Size2::new(123.4, 56.7));
        assert_eq!(box2.width(), 123.4);
        assert_eq!(box2.height(), 56.7);
    }

    #[test]
    fn test_box2i_from_size() {
        let box2 = Box2i::from(Size2::new(1234, 567));
        assert_eq!(box2.left(), 0);
        assert_eq!(box2.right(), 1234);
        assert_eq!(box2.top(), 0);
        assert_eq!(box2.bottom(), 567);

        assert_eq!(box2.size(), Size2::new(1234, 567));
        assert_eq!(box2.width(), 1234);
        assert_eq!(box2.height(), 567);
    }

    #[test]
    fn test_box2f_from_size() {
        let box2 = Box2f::from(Size2::new(123.4, 56.7));
        assert_eq!(box2.left(), 0.0);
        assert_eq!(box2.right(), 123.4);
        assert_eq!(box2.top(), 0.0);
        assert_eq!(box2.bottom(), 56.7);

        assert_eq!(box2.size(), Size2::new(123.4, 56.7));
        assert_eq!(box2.width(), 123.4);
        assert_eq!(box2.height(), 56.7);
    }

    #[test]
    fn test_box2f_intersect() {
        // TODO: more tests with boxes above/below/etc.
        assert_eq!(
            Box2f::at(Vector2::new(-50.5, 1000.75), Size2::new(100.0, 50.0)).intersect(Box2f::at(
                Vector2::new(-100.5, 1025.0),
                Size2::new(100.75, 10.1),
            )),
            Box2f::new(Vector2::new(-50.5, 1025.0), Vector2::new(0.25, 1035.1)),
        );
    }

    #[test]
    fn test_box2f_intersect_with_no_overlap() {
        // Box after in both dimensions:
        assert_eq!(
            Box2f::at(Vector2::new(10.0, 100.0), Size2::new(1.0, 2.0)).intersect(Box2f::at(
                Vector2::new(11.0, 102.0),
                Size2::new(100.0, 200.0),
            )),
            Box2f::default(),
        );
        // Box after in dimension x:
        assert_eq!(
            Box2f::at(Vector2::new(10.0, 100.0), Size2::new(1.0, 2.0)).intersect(Box2f::at(
                Vector2::new(11.0, 100.5),
                Size2::new(100.0, 200.0),
            )),
            Box2f::default(),
        );
        // Box after in dimension y:
        assert_eq!(
            Box2f::at(Vector2::new(10.0, 100.0), Size2::new(1.0, 2.0)).intersect(Box2f::at(
                Vector2::new(9.0, 102.0),
                Size2::new(100.0, 200.0),
            )),
            Box2f::default(),
        );
    }
}
