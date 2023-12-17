pub type Color = (f64, f64, f64);

pub struct DrawState {
    background_color: Color,
}

impl DrawState {
    pub fn new(background_color: Color) -> DrawState {
        DrawState{
            background_color,
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }
}