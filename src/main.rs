mod app;
mod text_area;
mod window;

use app::Application;

fn main() {
    let app = Application::new();
    pollster::block_on(app.run());
}
