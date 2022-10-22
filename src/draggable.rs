use egui::Vec2;

pub struct Draggable {
    pub position: Vec2,
    offset: Vec2,
    is_dragging: bool,
}

impl Draggable {
    pub fn new(pos: Vec2) -> Self {
        Self {
            position: pos,
            offset: Vec2::ZERO,
            is_dragging: false,
        }
    }
    pub fn start_dragging(&mut self, mouse_pos: impl Into<Vec2>) {
        self.is_dragging = true;
        self.offset = self.position - mouse_pos.into();
    }
    pub fn update(&mut self, mouse_pos: impl Into<Vec2>) {
        if self.is_dragging {
            self.position = mouse_pos.into() + self.offset;
        }
    }
    pub fn end_dragging(&mut self) {
        self.is_dragging = false;
    }
}
