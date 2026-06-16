#[derive(Clone)]
pub struct FooterController {
    pub visible: bool,
}

impl FooterController {
    pub fn new() -> Self {
        Self { visible: false }
    }

    pub fn toggle(&mut self, visibility: bool) {
        self.visible = visibility;
    }
}
