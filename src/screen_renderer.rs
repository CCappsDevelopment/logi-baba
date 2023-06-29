use std::collections::{HashMap, HashSet};
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
    pub fn draw(&mut self, entities: &mut Vec<Entity>, entity_map: &mut HashMap<(i32, i32), HashSet<usize>>) {
        if self.last_frame_ticks.elapsed().as_millis() >= 60 {
            self.update(entities, entity_map);

            let _ = self.draw_bg();
            let _ = self.draw_grid();
            let _ = self.draw_entities(entities);

            self.last_frame_ticks = Instant::now();
        }
    
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

    // ad a cfg to run the draw_grid function on windows and macos
    #[cfg(not(target_os = "linux"))]
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
        for entity in entities.iter_mut() {
            let texture_creator = self.context.canvas.texture_creator();
            let mut surface: Surface = LoadSurface::from_file(&entity.sprite_data.sprite_sheet)?;

            let color_key = Color::RGB(84, 165, 75);
            surface.set_color_key(true, color_key).expect("Could not set color key");

            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let sprite_rect = Rect::new(
                entity.sprite_data.frame_x,
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

            self.context.canvas.copy(&texture, Some(sprite_rect), Some(world_rect))?;
        }

        Ok(())
    }

    pub fn update(&mut self, entities: &mut Vec<Entity>, entity_map: &mut HashMap<(i32, i32), HashSet<usize>>) {
        for entity in entities.iter_mut() {
                // compute the tile positions of the four adjacent tiles
                let up_tile = (entity.tile.0, entity.tile.1 - 1);
                let right_tile = (entity.tile.0 + 1, entity.tile.1);
                let down_tile = (entity.tile.0, entity.tile.1 + 1);
                let left_tile = (entity.tile.0 - 1, entity.tile.1);

                // use the tile positions to query the entity_map
                let _ = entity.neighbors.up.insert(entity_map.get(&up_tile).unwrap_or(&HashSet::new()).clone());
                let _ = entity.neighbors.right.insert(entity_map.get(&right_tile).unwrap_or(&HashSet::new()).clone());
                let _ = entity.neighbors.down.insert(entity_map.get(&down_tile).unwrap_or(&HashSet::new()).clone());
                let _ = entity.neighbors.left.insert(entity_map.get(&left_tile).unwrap_or(&HashSet::new()).clone());
                
                if entity.states.contains_key(&EntityState::You) {
                    println!("up: {:?}\n right: {:?}\n down: {:?}\n left: {:?}\n", entity.neighbors.up, entity.neighbors.right, entity.neighbors.down, entity.neighbors.left);
                    entity.tile = match entity.movement_direction {
                        MovementDirection::Up => up_tile,
                        MovementDirection::Right => right_tile,
                        MovementDirection::Down => down_tile,
                        MovementDirection::Left => left_tile,
                        MovementDirection::Idle => entity.tile,
                    };
                    
                    entity.tile_to_position(self.tile_width, self.tile_height);

                    entity.sprite_data.current_frame =
                        (entity.sprite_data.current_frame + 1) % entity.sprite_data.num_frames;
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
                entity.sprite_data.frame_x =
                    frame_multiplier * frame_width_plus_one +
                    entity.sprite_data.start_frame.x() +
                    frame_width_plus_one *
                        ((entity.sprite_data.current_frame as i32) * is_idle_factor);

        }

        entity_map.clear();

        // then, repopulate it based on the new entity positions
        for (i, entity) in entities.iter().enumerate() {
            entity_map.entry(entity.tile).or_insert_with(HashSet::new).insert(i);
        }
    }
}
