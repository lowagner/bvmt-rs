#![allow(dead_code)]

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Pod, Zeroable)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
}
