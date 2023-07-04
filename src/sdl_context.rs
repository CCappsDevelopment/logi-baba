use std::collections::HashMap;

use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, BlendMode};
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct SdlContext {
    pub canvas: Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pub texture_map: HashMap<String, Texture>,
}

impl SdlContext {
    pub fn new() -> SdlContext {
        let (event_pump, canvas) = Self::init_sdl2().unwrap();

        let window_width = canvas.viewport().width();
        let window_height = canvas.viewport().height();

        let texture_creator = canvas.texture_creator();
        
        let color_key = Color::RGB(84, 165, 75);
        
        let char_spritesheet = String::from("./assets/spritesheets/characters.png");
        let obj_spritesheet = String::from("./assets/spritesheets/objects.png");
        let text_spritesheet = String::from("./assets/spritesheets/text-entities.png");

        let mut char_surface: Surface = LoadSurface::from_file(char_spritesheet.clone()).unwrap();
        let mut obj_surface: Surface = LoadSurface::from_file(obj_spritesheet.clone()).unwrap();
        let mut text_surface: Surface = LoadSurface::from_file(text_spritesheet.clone()).unwrap();

        char_surface.set_color_key(true, color_key).expect("Could not set color key");
        obj_surface.set_color_key(true, color_key).expect("Could not set color key");
        text_surface.set_color_key(true, color_key).expect("Could not set color key");

        let char_texture = texture_creator
            .create_texture_from_surface(&char_surface)
            .map_err(|e| e.to_string()).unwrap();
        let obj_texture = texture_creator
            .create_texture_from_surface(&obj_surface)
            .map_err(|e| e.to_string()).unwrap();
        let text_texture = texture_creator
            .create_texture_from_surface(&text_surface)
            .map_err(|e| e.to_string()).unwrap();

        let mut texture_map = HashMap::new();
        texture_map.insert(char_spritesheet.clone(), char_texture);
        texture_map.insert(obj_spritesheet.clone(), obj_texture);
        texture_map.insert(text_spritesheet.clone(), text_texture);

        SdlContext {
            event_pump,
            canvas,
            texture_map
        }
    }

    fn init_sdl2() -> Result<(EventPump, Canvas<sdl2::video::Window>), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        // Get the current display mode so we can determine screen dimensions
        let display_mode = video_subsystem.current_display_mode(0)?;

        // Calculate window dimensions as percentages of screen dimensions
        let window_width: u32 = ((display_mode.w as f32) * 0.6) as u32;
        let window_height: u32 = ((display_mode.h as f32) * 0.64) as u32;

        let window: Window = video_subsystem
            .window("Logibaba", window_width, window_height)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            //.software() // turn off hardware acceleration
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;

        Ok((event_pump, canvas))
    }
}
