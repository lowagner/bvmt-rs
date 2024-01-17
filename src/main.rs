pub mod color;
pub mod dimensions;
pub mod gpu;
pub mod pixels;
mod synced;
pub mod window;

use window::*;

fn main() {
    pollster::block_on(run());
}
