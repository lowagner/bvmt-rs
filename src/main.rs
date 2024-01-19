pub mod app;
pub mod color;
pub mod dimensions;
pub mod gpu;
pub mod pixels;
mod synced;
pub mod window;

use app::*;
use window::*;

fn main() {
    let app = Box::<DefaultApp>::default();
    pollster::block_on(run(app));
}
