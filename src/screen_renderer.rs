use std::time::Instant;

use sdl2::image::{ InitFlag, LoadSurface, Sdl2ImageContext };
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::pixels::Color;

use crate::logibaba::{ MovementDirection, EntityState };
use crate::{ sdl_context::SdlContext, logibaba::GameEntity };

pub struct ScreenRenderer {
    pub context: SdlContext,
    pub frame_delay: u32,
    pub frame_ticks: u32,
    pub last_frame_ticks: Instant,
    pub image_context: Sdl2ImageContext,
}

impl ScreenRenderer {
    pub fn new() -> ScreenRenderer {
        let context = SdlContext::new();
        let frame_delay = 10; // Increase for slower animation.
        let frame_ticks = 0;
        let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
        let last_frame_ticks = Instant::now();
        
        ScreenRenderer { context, frame_delay, frame_ticks, image_context, last_frame_ticks }
    }

    // Render the screen
    pub fn draw(&mut self, entities: &mut Vec<GameEntity>) {
        self.context.canvas.set_draw_color(Color::RGB(28, 28, 40));
        self.context.canvas.clear();
        self.context.canvas.set_draw_color(Color::RGB(228, 228, 240));

        let width = self.context.canvas.viewport().width();
        let height = self.context.canvas.viewport().height();

        // Draw vertical lines
        for x in (0..width).step_by(128) {
            self.context.canvas.draw_line((x as i32, 0), (x as i32, height as i32)).unwrap();
        }

        // Draw horizontal lines
        for y in (0..height).step_by(128) {
            self.context.canvas.draw_line((0, y as i32), (width as i32, y as i32)).unwrap();
        }

        for entity in entities {
            let texture_creator = self.context.canvas.texture_creator();
            let mut surface: Surface = LoadSurface::from_file(
                &entity.sprite_data.sprite_sheet
            ).unwrap();

            let color_key = Color::RGB(84, 165, 75);
            surface.set_color_key(true, color_key).expect("Could not set color key");

            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            
            let mut sprite_x = entity.sprite_data.start_frame.x();

            if entity.states.contains_key(&EntityState::You) {

                if self.last_frame_ticks.elapsed().as_millis() > 100 {
                    let speed_pixels = 128;
                    entity.position = match entity.movement_direction {
                        MovementDirection::Up => (entity.position.0, entity.position.1 - speed_pixels),
                        MovementDirection::Right => (entity.position.0 + speed_pixels , entity.position.1),
                        MovementDirection::Down => (entity.position.0, entity.position.1 + speed_pixels),
                        MovementDirection::Left => (entity.position.0 - speed_pixels, entity.position.1),
                        MovementDirection::Idle => entity.position,
                    };
                    self.last_frame_ticks = Instant::now();

                    entity.tile = (
                        entity.position.0 / 128,
                        entity.position.1 / 128,
                    );

                    if entity.tile.0 > 11 {
                        entity.tile.0 = 0;
                        entity.position.0 = 0;
                    }
                    if entity.tile.0 < 0 {
                        entity.tile.0 = 11;
                        entity.position.0 = 128 * 11;
                    }
                    if entity.tile.1 > 7 {
                        entity.tile.1 = 0;
                        entity.position.1 = 0;
                    }
                    if entity.tile.1 < 0 {
                        entity.tile.1 = 7;
                        entity.position.1 = 128 * 7;
                    }
                }

                let frame_multiplier = match entity.movement_direction {
                    MovementDirection::Up => 4,
                    MovementDirection::Right => 0,
                    MovementDirection::Down => 12,
                    MovementDirection::Left => 8,
                    MovementDirection::Idle =>
                        match entity.facing {
                            MovementDirection::Up => 4,
                            MovementDirection::Right => 0,
                            MovementDirection::Down => 12,
                            MovementDirection::Left => 8,
                            _ => 0,
                        }
                };

                let is_idle_factor = match entity.movement_direction {
                    MovementDirection::Idle => 0,
                    _ => 1,
                };

                let frame_width_plus_one = (entity.sprite_data.frame_width as i32) + 1;
                sprite_x =
                    frame_multiplier * frame_width_plus_one +
                    entity.sprite_data.start_frame.x() +
                    frame_width_plus_one *
                        ((entity.sprite_data.current_frame as i32) * is_idle_factor);
            }
            
            let sprite_rect = Rect::new(
                sprite_x,
                entity.sprite_data.start_frame.y(),
                entity.sprite_data.frame_width,
                entity.sprite_data.frame_height
            );

            let world_rect = Rect::new(
                entity.position.0,
                entity.position.1,
                entity.sprite_data.sprite_width,
                entity.sprite_data.sprite_height
            );

            // Get next frame
            if self.frame_ticks >= self.frame_delay {
                entity.sprite_data.current_frame =
                    (entity.sprite_data.current_frame + 1) % entity.sprite_data.num_frames;
                self.frame_ticks = 0;
            }

            self.frame_ticks += 1;

            self.context.canvas.copy(&texture, Some(sprite_rect), Some(world_rect)).unwrap();
        }
        self.context.canvas.present();
    }
}
