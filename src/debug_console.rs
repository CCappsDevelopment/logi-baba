use std::collections::BTreeMap;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::{ Font, Sdl2TtfContext };
use lazy_static::lazy_static;

lazy_static! {
    static ref TTF_CONTEXT: Sdl2TtfContext = sdl2::ttf::init().unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebugKey {
    Render(String),
    Rules(String),
}

pub struct DebugConsole<'a> {
    font: Font<'a, 'static>,
    pub color: Color,
    pub text_map: BTreeMap<DebugKey, String>,
    pub x_position: i32,
    pub y_position: i32,
    pub show_console: bool,
}

impl<'a> DebugConsole<'a> {
    pub fn new() -> DebugConsole<'a> {
        let font = TTF_CONTEXT.load_font("./assets/fonts/LibreFranklin-Medium.ttf", 12).unwrap();
        let color = Color::WHITE;
        let text_map = BTreeMap::new();
        let x_position = 10;
        let y_position = 10;
        let show_console = false;

        DebugConsole {
            font,
            color,
            text_map,
            x_position,
            y_position,
            show_console,
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let texture_creator = canvas.texture_creator();
        let mut y_position = self.y_position;
    
        for (_key, text) in &self.text_map {
            let surface = self.font.render(&text.clone()).blended(self.color).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            let texture_query = texture.query();
            let texture_width = texture_query.width;
            let texture_height = texture_query.height;
            let texture_rect = sdl2::rect::Rect::new(self.x_position, y_position, texture_width, texture_height);
            
            canvas.copy(&texture, None, Some(texture_rect)).unwrap();
    
            y_position += texture_height as i32;  // Move the position down for the next line
        }
    }

    pub fn out(&mut self, debug_strings: Vec<(DebugKey, String)>) {
        for (key, debug_string) in debug_strings {
            self.text_map.insert(key, debug_string);
        }
        //println!("Debug Strings: {:?}", self.text_map);
    }
}
