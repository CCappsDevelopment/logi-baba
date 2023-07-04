use std::collections::BTreeMap;

use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::ttf::{ Font, Sdl2TtfContext };
use lazy_static::lazy_static;

lazy_static! {
    static ref TTF_CONTEXT: Sdl2TtfContext = sdl2::ttf::init().unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebugKey {
    Entity(String),
    Rules(String),
}

pub struct DebugText {
    text: String,
    texture: Option<Texture>,
}

pub struct DebugConsole<'a> {
    font: Font<'a, 'static>,
    pub color: Color,
    pub text_map: BTreeMap<DebugKey, DebugText>,
    pub x_position: i32,
    pub y_position: i32,
    pub show_console: bool,
}

impl<'a> DebugConsole<'a> {
    pub fn new() -> DebugConsole<'a> {
        let font = TTF_CONTEXT.load_font("./assets/fonts/LibreFranklin-Medium.ttf", 9).unwrap();
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

    // TODO: still small memory leak with the textures
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let texture_creator = canvas.texture_creator();
        let mut y_position = self.y_position;

        for (_key, debug_text) in &mut self.text_map {
            if debug_text.texture.is_none() {
                let surface = self.font.render(&debug_text.text).blended(self.color).unwrap();
                let new_texture = Some(texture_creator.create_texture_from_surface(&surface).unwrap());

                debug_text.texture = new_texture;
            }

            if let Some(texture) = &debug_text.texture {
                let texture_query = texture.query();
                let texture_width = texture_query.width;
                let texture_height = texture_query.height;
                let texture_rect = sdl2::rect::Rect::new(self.x_position, y_position, texture_width, texture_height);

                canvas.copy(&texture, None, Some(texture_rect)).unwrap();

                y_position += texture_height as i32;  // Move the position down for the next line
            }
        }
    }

    pub fn out(&mut self, debug_strings: Vec<(DebugKey, String)>) {
        for (key, debug_string) in debug_strings {
            let lines = debug_string.split('\n').map(|s| s.to_owned()).collect::<Vec<String>>();
            for (i, line) in lines.iter().enumerate() {
                let key_clone = match &key {
                    DebugKey::Entity(key_str) => DebugKey::Entity(format!("{}_{:03}", key_str, i)),
                    DebugKey::Rules(key_str) => DebugKey::Rules(format!("{}_{:03}", key_str, i)),
                };

                let debug_text = self.text_map.entry(key_clone).or_insert(DebugText {
                    text: String::new(),
                    texture: None,
                });

                if debug_text.text != *line {
                    debug_text.text = line.clone();
                    debug_text.texture = None;  // Force the texture to be updated in the next draw call
                }
            }
        }
    }
    
    
}
