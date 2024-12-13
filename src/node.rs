use macroquad::math::Vec2;

pub const NODE_RADIUS: f32 = 30.0;
pub const NODE_FONT_SIZE: u16 = 36;
pub const NODE_RADIUS_SQR: f32 = NODE_RADIUS * NODE_RADIUS;

pub struct GNode {
    pub data: u64,
    pub pos: Vec2,
    pub speed: Vec2
}