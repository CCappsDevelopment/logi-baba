use std::collections::{ HashMap, HashSet };
use std::time::Instant;

use sdl2::image::{ InitFlag, Sdl2ImageContext };
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use crate::debug_console::{DebugConsole, DebugKey};
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
    pub grid_size: (i32, i32),
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
        let grid_size = (12, 8);

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
            grid_size,
        }
    }

    // Render the screen
    pub fn draw(
        &mut self,
        entities: &mut Vec<Entity>,
        entity_map: &mut HashMap<(i32, i32), HashSet<usize>>,
        debug_console: &mut DebugConsole,
    ) {
        if self.last_frame_ticks.elapsed().as_millis() >= 80 {
            self.update(entities, entity_map, debug_console);
            self.last_frame_ticks = Instant::now();
        }

        let _ = self.draw_bg();
        let _ = self.draw_grid();
        let _ = self.draw_entities(entities);

        if debug_console.show_console {
            self.debug_console_out(debug_console, entities, entity_map);
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
            let texture = &self.context.texture_map.get(&entity.sprite_data.sprite_sheet).unwrap();

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

    pub fn update(
        &mut self,
        entities: &mut Vec<Entity>,
        entity_map: &mut HashMap<(i32, i32), HashSet<usize>>,
        debug_console: &mut DebugConsole,
    ) {
        // Update entity positions
        let new_positions = self.update_positions(entities, debug_console);

        // Update entity map
        self.update_entity_map(entities, entity_map, new_positions);
        
        // Update entity neighbors
        self.update_neighbors(entities, entity_map);

        // Update entity sprite frames
        self.update_sprite_frames(entities);
    }

    fn check_can_move(&self, directional_neighbor: &Option<HashSet<usize>>, entities: &Vec<Entity>) -> bool {
        return directional_neighbor.as_ref().map_or(false, |neighbor| {
            neighbor.iter().any(|idx| {
                entities
                    .get(*idx)
                    .map(|e| e.states.contains_key(&EntityState::Stop))
                    .unwrap_or(false)
            })
        })
    }
    
    fn check_is_pushed(&self, entity: &Entity, entities: &Vec<Entity>) -> (bool, Option<MovementDirection>) {
        if self.check_direction(entity.neighbors.up.as_ref(), MovementDirection::Down, entities) {
            (true, Some(MovementDirection::Up))
        } else if self.check_direction(entity.neighbors.right.as_ref(), MovementDirection::Left, entities) {
            (true, Some(MovementDirection::Right))
        } else if self.check_direction(entity.neighbors.left.as_ref(), MovementDirection::Right, entities) {
            (true, Some(MovementDirection::Left))
        } else if self.check_direction(entity.neighbors.down.as_ref(), MovementDirection::Up, entities) {
            (true, Some(MovementDirection::Down))
        } else {
            (false, None)
        }
    }

    fn check_direction(&self, neighbors: Option<&HashSet<usize>>, direction: MovementDirection, entities: &Vec<Entity>) -> bool {
        neighbors.as_ref().map_or(false, |neighbor| {
            neighbor.iter().any(|idx| {
                entities
                    .get(*idx)
                    .map(|e| e.movement_direction == direction)
                    .unwrap_or(false)
            })
        })
    }
    
    fn update_positions(&self, entities: &mut Vec<Entity>, debug_console: &mut DebugConsole) -> HashMap<usize, (i32, i32)> {
        let mut new_positions: HashMap<usize, (i32, i32)> = HashMap::new();

        for (i, entity) in entities.iter().enumerate() {
            // Only move if there's no STOP entity in the direction of movement
            let mut can_move = false;
            if entity.states.contains_key(&EntityState::You) || entity.states.contains_key(&EntityState::Move) {
                can_move = match entity.movement_direction {
                    MovementDirection::Up => {
                        !self.check_can_move(&entity.neighbors.up, &entities)
                    }
                    MovementDirection::Right => {
                        !self.check_can_move(&entity.neighbors.right, &entities)
                    }
                    MovementDirection::Down => {
                        !self.check_can_move(&entity.neighbors.down, &entities)
                    }
                    MovementDirection::Left => {
                        !self.check_can_move(&entity.neighbors.left, &entities)
                    }
                    _ => true,
                };
            }

            let mut is_pushed = false;
            let mut push_direction = None;
            if entity.states.contains_key(&EntityState::Push) && !entity.states.contains_key(&EntityState::Stop){
                let push_check = self.check_is_pushed(&entity, &entities);
                is_pushed = push_check.0;
                if is_pushed {
                    push_direction = push_check.1;
                }
            }
            
            let new_tile = 
                if can_move {
                    match entity.movement_direction {
                        MovementDirection::Up => (entity.tile.0, entity.tile.1 - 1),
                        MovementDirection::Right => (entity.tile.0 + 1, entity.tile.1),
                        MovementDirection::Down => (entity.tile.0, entity.tile.1 + 1),
                        MovementDirection::Left => (entity.tile.0 - 1, entity.tile.1),
                        MovementDirection::Idle => entity.tile,
                    }
                } else if is_pushed {
                    match push_direction {
                        Some(MovementDirection::Up) => (entity.tile.0, entity.tile.1 + 1),
                        Some(MovementDirection::Right) => (entity.tile.0 - 1, entity.tile.1),
                        Some(MovementDirection::Down) => (entity.tile.0, entity.tile.1 - 1),
                        Some(MovementDirection::Left) => (entity.tile.0 + 1, entity.tile.1),
                        _ => entity.tile,
                    }
                } else {
                    entity.tile
                };


            new_positions.insert(i, new_tile);

            let debug_strings = vec![
                (DebugKey::Render(format!("CheckStop:({:?})", entity.name).to_owned()), format!("{:?} - Is Stopped: {:?}", entity.name, !can_move)),
                (DebugKey::Render(format!("CheckPush:({:?})", entity.name).to_owned()), format!("{:?} - Is Pushed: {:?}", entity.name, is_pushed)),
                (DebugKey::Render(format!("CheckPushDir:({:?})", entity.name).to_owned()), format!("{:?} - Push Dir: {:?}", entity.name, push_direction)),

            ];
            debug_console.out(debug_strings);
        }       
        return new_positions;
    }

    fn update_entity_map(&self, entities: &mut Vec<Entity>, entity_map: &mut HashMap<(i32, i32), HashSet<usize>>, new_positions: HashMap<usize, (i32, i32)>) {
        entity_map.clear();
        for (i, entity) in entities.iter_mut().enumerate() {
            if let Some(new_tile) = new_positions.get(&i) {
                entity.tile = *new_tile;
                entity.tile_to_position(self.tile_width, self.tile_height);
                entity_map.entry(entity.tile).or_insert_with(HashSet::new).insert(i);
            }
        }
    }

    fn update_neighbors(&self, entities: &mut Vec<Entity>, entity_map: &HashMap<(i32, i32), HashSet<usize>>) {
        for (_i, entity) in entities.iter_mut().enumerate() {
            // compute the tile positions of the four adjacent tiles
            let up_tile = (entity.tile.0, (entity.tile.1 - 1 + self.grid_size.1) % self.grid_size.1);
            let right_tile = ((entity.tile.0 + 1 + self.grid_size.0) % self.grid_size.0, entity.tile.1);
            let down_tile = (entity.tile.0, (entity.tile.1 + 1 + self.grid_size.1) % self.grid_size.1);
            let left_tile = ((entity.tile.0 - 1 + self.grid_size.0) % self.grid_size.0, entity.tile.1);

            // use the tile positions to query the entity_map
            entity.neighbors.up = entity_map.get(&up_tile).cloned();
            entity.neighbors.right = entity_map.get(&right_tile).cloned();
            entity.neighbors.down = entity_map.get(&down_tile).cloned();
            entity.neighbors.left = entity_map.get(&left_tile).cloned();
        }
    }


    fn update_sprite_frames(&self, entities: &mut Vec<Entity>) {
        for (_i, entity) in entities.iter_mut().enumerate() {
            if entity.movement_direction == MovementDirection::Idle {
                continue;
            }

            // Determine frame_multiplier based on entity direction
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

            // Update entity animation frame
            entity.sprite_data.current_frame =
                (entity.sprite_data.current_frame + 1) % entity.sprite_data.num_frames;

            // TODO: further abstract this
            let frame_width_plus_one = (entity.sprite_data.frame_width as i32) + 1;
            let frame_height_plus_one = (entity.sprite_data.frame_height as i32) + 1;
            let num_x_frames = 4;
            let num_y_frames = entity.sprite_data.num_frames / num_x_frames;
                
            // Calculate frame position based on entity state
            entity.sprite_data.frame_x =
                frame_multiplier * frame_width_plus_one +
                entity.sprite_data.start_frame.x() +
                frame_width_plus_one * ((entity.sprite_data.current_frame % num_x_frames) as i32);

            if entity.sprite_data.current_frame % num_x_frames == 0 {
                entity.sprite_data.frame_y =
                    entity.sprite_data.start_frame.y() +
                    frame_height_plus_one *
                        ((entity.sprite_data.current_frame / num_y_frames) as i32);
            }
        }
    }

    fn debug_console_out(&mut self, debug_console: &mut DebugConsole, entities: &Vec<Entity>, entity_map: &HashMap<(i32, i32), HashSet<usize>>) {
        for entity in entities {
            let tile_str = format!("{:?} Tile: x: {}, y: {}", entity.name, entity.tile.0, entity.tile.1);
            let neighbors_str = format!("{:?}, {:?}", entity.name, entity.neighbors);
            let entity_map_str = format!("Entity Map: {:?}", entity_map);
            let debug_strings = vec![
                (DebugKey::Render(format!("Tile({:?})", entity.name).to_owned()), tile_str),
                (DebugKey::Render(format!("Neighbors({:?})", entity.name).to_owned()), neighbors_str),
                (DebugKey::Render("Entity Map".to_string()), entity_map_str)
            ];
            debug_console.out(debug_strings);
        }
        debug_console.draw(&mut self.context.canvas);
    }
}
