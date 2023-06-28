use std::time::Instant;

use sdl2::image::{ InitFlag, LoadSurface, Sdl2ImageContext };
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use sdl2::surface::Surface;

use crate::entity::{ Entity, EntityState };
use crate::logibaba::MovementDirection;
use crate::sdl_context::SdlContext;

pub struct ScreenRenderer {
    pub context: SdlContext,
    pub frame_delay: u32,
    pub frame_ticks: u32,
    pub last_frame_ticks: Instant,
    pub image_context: Sdl2ImageContext,
    pub window_width: i32,
    pub window_height: i32,
    pub tile_width: i32,
    pub tile_height: i32,
}

impl ScreenRenderer {
    pub fn new() -> ScreenRenderer {
        let context = SdlContext::new();
        let frame_delay = 6; // Increase for slower animation.
        let frame_ticks = 0;
        let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
        let last_frame_ticks = Instant::now();
        let window_width = context.canvas.viewport().width() as i32;
        let window_height = context.canvas.viewport().height() as i32;
        let tile_width = window_width / 12;
        let tile_height = window_height / 8;

        ScreenRenderer {
            context,
            frame_delay,
            frame_ticks,
            image_context,
            last_frame_ticks,
            window_width,
            window_height,
            tile_width,
            tile_height,
        }
    }

    // Render the screen
    pub fn draw(&mut self, entities: &mut Vec<Entity>) {
        // Clear the screen with a background color
        let _ = self.draw_bg();

        // Draw the tile grid
        let _ = self.draw_grid();

        // Draw the entities
        let _ = self.draw_entities(entities);

        self.context.canvas.present();
    }

    fn draw_bg(&mut self) -> Result<(), String> {
        self.context.canvas.set_draw_color(Color::RGB(28, 28, 40));
        self.context.canvas.clear();

        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn draw_grid(&mut self) -> Result<(), String> {
        self.context.canvas.set_draw_color(Color::RGBA(228, 228, 240, 64));

        // Draw vertical lines
        for x in (0..self.window_width).step_by((self.window_width as usize) / 12) {
            self.context.canvas
                .draw_line((x as i32, 0), (x as i32, self.window_height as i32))
                .unwrap();
        }

        // Draw horizontal lines
        for y in (0..self.window_height).step_by((self.window_height as usize) / 8) {
            self.context.canvas
                .draw_line((0, y as i32), (self.window_width as i32, y as i32))
                .unwrap();
        }

        Ok(())
    }

    #[cfg(target_os = "windows, macos")]
    fn draw_grid(&mut self) -> Result<(), String> {
        // Create a texture for drawing
        let texture_creator = self.context.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_target(None, self.window_width as u32, self.window_height as u32)
            .unwrap();

        // Set the blend mode for the texture
        texture.set_blend_mode(BlendMode::Blend);

        // Set the texture as the target for the canvas
        self.context.canvas
            .with_texture_canvas(&mut texture, |canvas| {
                canvas.set_draw_color(Color::RGBA(228, 228, 240, 64));

                // Draw vertical lines
                for x in (0..self.window_width).step_by((self.window_width as usize) / 12) {
                    canvas.draw_line((x as i32, 0), (x as i32, self.window_height as i32)).unwrap();
                }

                // Draw horizontal lines
                for y in (0..self.window_height).step_by((self.window_height as usize) / 8) {
                    canvas.draw_line((0, y as i32), (self.window_width as i32, y as i32)).unwrap();
                }
            })
            .unwrap();

        // Draw the texture to the screen
        self.context.canvas.copy(&texture, None, None).unwrap();

        Ok(())
    }

    fn draw_entities(&mut self, entities: &mut Vec<Entity>) -> Result<(), String> {
        entities.sort_by_key(|entity| entity.draw_order);

        for entity in entities {
            let texture_creator = self.context.canvas.texture_creator();
            let mut surface: Surface = LoadSurface::from_file(&entity.sprite_data.sprite_sheet)?;

            let color_key = Color::RGB(84, 165, 75);
            surface.set_color_key(true, color_key).expect("Could not set color key");

            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let mut sprite_x = entity.sprite_data.start_frame.x();

            if entity.states.contains_key(&EntityState::You) {
                if self.last_frame_ticks.elapsed().as_millis() > 100 {
                    entity.position = match entity.movement_direction {
                        MovementDirection::Up => {
                            (entity.position.0, entity.position.1 - self.tile_height)
                        }
                        MovementDirection::Right => {
                            (entity.position.0 + self.tile_width, entity.position.1)
                        }
                        MovementDirection::Down => {
                            (entity.position.0, entity.position.1 + self.tile_height)
                        }
                        MovementDirection::Left => {
                            (entity.position.0 - self.tile_width, entity.position.1)
                        }
                        MovementDirection::Idle => entity.position,
                    };
                    self.last_frame_ticks = Instant::now();

                    entity.tile = (
                        entity.position.0 / self.tile_width,
                        entity.position.1 / self.tile_height,
                    );

                    if entity.tile.0 > 11 {
                        entity.tile.0 = 0;
                        entity.position.0 = 0;
                    }
                    if entity.tile.0 < 0 {
                        entity.tile.0 = 11;
                        entity.position.0 = self.tile_width * 11;
                    }
                    if entity.tile.1 > 7 {
                        entity.tile.1 = 0;
                        entity.position.1 = 0;
                    }
                    if entity.tile.1 < 0 {
                        entity.tile.1 = 7;
                        entity.position.1 = self.tile_height * 7;
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

            self.context.canvas.copy(&texture, Some(sprite_rect), Some(world_rect))?;
        }

        Ok(())
    }
}
