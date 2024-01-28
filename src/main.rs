pub mod app;
pub mod color;
pub mod defaults;
pub mod dimensions;
pub mod fragments;
pub mod gpu;
pub mod options;
pub mod pixels;
pub mod scene;
pub mod shader;
pub mod variables;
pub mod vertices;
pub mod window;

use app::*;
use window::*;

fn main() {
    let app = Box::<DefaultApp>::default();
    pollster::block_on(run(app));
}
