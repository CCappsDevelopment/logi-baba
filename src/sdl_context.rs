use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct SdlContext {
    pub canvas: Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
}

impl SdlContext {
    pub fn new() -> SdlContext {
        let (event_pump, canvas) = Self::init_sdl2().unwrap();
        SdlContext {
            event_pump,
            canvas,
        }
    }

    fn init_sdl2() -> Result<(EventPump, Canvas<sdl2::video::Window>), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
    
        // Get the current display mode so we can determine screen dimensions
        let display_mode = video_subsystem.current_display_mode(0)?;
    
        // Calculate window dimensions as percentages of screen dimensions
        let window_width: u32 = ((display_mode.w as f32) * 0.60) as u32;
        let window_height: u32 = ((display_mode.h as f32) * 0.64) as u32;
    
        let window: Window = video_subsystem
            .window("Logibaba", window_width, window_height)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;
    
        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;
    
        let event_pump = sdl_context.event_pump()?;
    
        Ok((event_pump, canvas))
    }
}