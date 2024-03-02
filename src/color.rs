#![allow(dead_code)]

use crate::dimensions::{Vector3f, Vector4f};

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Pod, Zeroable)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Returns true iff this color is not transparent or translucent at all.
    pub fn is_opaque(&self) -> bool {
        self.a == 255
    }

    /// Returns true iff this color is neither opaque nor 100% transparent.
    pub fn is_translucent(&self) -> bool {
        self.a > 0 && self.a < 255
    }

    /// Returns true iff this color is 100% transparent.
    pub fn is_transparent(&self) -> bool {
        self.a == 0
    }

    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    pub fn red(r: u8) -> Self {
        Color {
            r,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn green(g: u8) -> Self {
        Color {
            r: 0,
            g,
            b: 0,
            a: 255,
        }
    }

    pub fn blue(b: u8) -> Self {
        Color {
            r: 0,
            g: 0,
            b,
            a: 255,
        }
    }
}

impl From<Color> for wgpu::Color {
    fn from(color: Color) -> wgpu::Color {
        Self {
            r: color.r as f64 / 255.0,
            g: color.g as f64 / 255.0,
            b: color.b as f64 / 255.0,
            a: color.a as f64 / 255.0,
        }
    }
}

impl From<Color> for Vector4f {
    fn from(color: Color) -> Vector4f {
        Self {
            x: color.r as f32 / 255.0,
            y: color.g as f32 / 255.0,
            z: color.b as f32 / 255.0,
            w: color.a as f32 / 255.0,
        }
    }
}

impl From<Color> for Vector3f {
    fn from(color: Color) -> Vector3f {
        Self {
            x: color.r as f32 / 255.0,
            y: color.g as f32 / 255.0,
            z: color.b as f32 / 255.0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_color_to_wgpu() {
        assert_eq!(
            wgpu::Color::from(Color {
                r: 120,
                g: 40,
                b: 10,
                a: 254,
            }),
            wgpu::Color {
                r: 0.47058823529411764,
                g: 0.1568627450980392,
                b: 0.0392156862745098,
                a: 0.996078431372549,
            },
        );
    }

    #[test]
    fn test_color_red() {
        assert_eq!(
            Color::red(103),
            Color {
                r: 103,
                g: 0,
                b: 0,
                a: 255
            },
        );
    }

    #[test]
    fn test_color_green() {
        assert_eq!(
            Color::green(51),
            Color {
                r: 0,
                g: 51,
                b: 0,
                a: 255
            },
        );
    }

    #[test]
    fn test_color_blue() {
        assert_eq!(
            Color::blue(72),
            Color {
                r: 0,
                g: 0,
                b: 72,
                a: 255
            },
        );
    }

    #[test]
    fn test_color_is_opaque() {
        assert_eq!(
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255
            }
            .is_opaque(),
            true
        );
        assert_eq!(
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 254
            }
            .is_opaque(),
            false
        );
        assert_eq!(
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 1
            }
            .is_opaque(),
            false
        );
        assert_eq!(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 0
            }
            .is_opaque(),
            false
        );
    }

    #[test]
    fn test_color_is_translucent() {
        assert_eq!(
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 1,
            }
            .is_translucent(),
            true
        );
        assert_eq!(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 254
            }
            .is_translucent(),
            true
        );
        assert_eq!(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 0,
            }
            .is_translucent(),
            false
        );
        assert_eq!(
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }
            .is_translucent(),
            false
        );
    }

    #[test]
    fn test_color_is_transparent() {
        assert!(Color::TRANSPARENT.is_transparent());
        assert_eq!(
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            }
            .is_transparent(),
            true
        );
        assert_eq!(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 0
            }
            .is_transparent(),
            true
        );
        assert_eq!(
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 1,
            }
            .is_transparent(),
            false
        );
        assert_eq!(
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }
            .is_transparent(),
            false
        );
    }
}
