use crate::window::Window;

pub struct TextArea {}

impl TextArea {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self {
        TextArea {}
    }
}
