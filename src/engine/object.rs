#[derive(Clone)]
pub enum Mesh {
    Rectangle,
    Circle,
    RoundedRectangle,
}

pub struct Object {
    pub position: cgmath::Point2<f32>,
    pub size: cgmath::Point2<f32>,
    pub angle: f32,
    pub color: [f32; 4],
    pub mesh: Mesh,
}