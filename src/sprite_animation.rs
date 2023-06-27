use sdl2::rect::Rect;

#[derive(Debug)]
pub struct SpriteData {
    pub sprite_sheet: String,
    pub frame_width: u32,
    pub frame_height: u32,
    pub sprite_width: u32,
    pub sprite_height: u32,
    pub start_frame: Rect,
    pub num_frames: u32,
    pub current_frame: u32,
}

pub struct SpriteAnimation {}

impl SpriteAnimation {
    pub fn new() -> SpriteAnimation {
        SpriteAnimation {}
    }

    pub fn animation_move_up(&mut self) {}

    pub fn animation_move_right(&mut self) {}

    pub fn animation_move_down(&mut self) {}

    pub fn animation_move_left(&mut self) {}
}
