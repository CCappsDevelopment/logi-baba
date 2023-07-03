use std::collections::{HashMap, HashSet};

use sdl2::rect::Rect;

use crate::logibaba::MovementDirection;

#[derive(Debug, Clone)]
pub struct SpriteData {
    pub sprite_sheet: String,
    pub frame_width: u32,
    pub frame_height: u32,
    pub sprite_width: u32,
    pub sprite_height: u32,
    pub start_frame: Rect,
    pub frame_x: i32,
    pub frame_y: i32,
    pub num_frames: u32,
    pub current_frame: u32,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum EntityState {
    You,
    Win,
    Push,
    Move,
    Stop,
}

#[derive(Debug, Clone)]
pub struct Neighbors {
    pub up: Option<HashSet<usize>>,
    pub right: Option<HashSet<usize>>,
    pub down: Option<HashSet<usize>>,
    pub left: Option<HashSet<usize>>,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub states: HashMap<EntityState, bool>,
    pub position: (i32, i32),
    pub tile: (i32, i32),
    pub neighbors: Neighbors,
    pub draw_order: i32,
    pub sprite_data: SpriteData,
    pub movement_direction: MovementDirection,
    pub intended_movement: MovementDirection,
    pub facing: MovementDirection,
    pub speed: f32,
}

impl Entity {
    pub fn tile_to_position(&mut self, tile_width: i32, tile_height: i32) {
        if self.tile.0 > 11 {
            self.tile.0 = 0;
        }
        if self.tile.0 < 0 {
            self.tile.0 = 11;
        }
        if self.tile.1 > 7 {
            self.tile.1 = 0;
        }
        if self.tile.1 < 0 {
            self.tile.1 = 7;
        }
        self.position = (self.tile.0 * (tile_width), self.tile.1 * (tile_height));
    }
}

pub struct EntityRepository {
    tile_width: u32,
    tile_height: u32,
    predefined_entities: HashMap<String, Entity>,
}

impl EntityRepository {
    pub fn new(tile_width: u32, tile_height: u32) -> Self {
        let mut predefined_entities = HashMap::new();

        predefined_entities.insert(
            "Logi".to_string(),
            Entity {
                name: "Logi".to_string(),
                states: vec![(EntityState::You, true)].into_iter().collect(),
                position: (0, 0),
                tile: (0, 0),
                neighbors: Neighbors {
                    up: None,
                    right: None,
                    down: None,
                    left: None,
                },
                draw_order: 3,
                sprite_data: SpriteData {
                    sprite_sheet: "./assets/spritesheets/characters.png".to_string(),
                    frame_width: 24,
                    frame_height: 24,
                    sprite_width: tile_width,
                    sprite_height: tile_height,
                    start_frame: Rect::new(576, 1, 24, 24),
                    frame_x: 576,
                    frame_y: 1,
                    num_frames: 12,
                    current_frame: 0,
                },
                movement_direction: MovementDirection::Idle,
                intended_movement: MovementDirection::Idle,
                facing: MovementDirection::Right,
                speed: 1.0,
            },
        );

        predefined_entities.insert(
            "Goal".to_string(),
            Entity {
                name: "Goal".to_string(),
                states: vec![(EntityState::Win, true)].into_iter().collect(),
                position: (5 * (tile_width as i32), 5 * (tile_height as i32)),
                tile: (5, 5),
                neighbors: Neighbors {
                    up: None,
                    right: None,
                    down: None,
                    left: None,
                },
                draw_order: 1,
                sprite_data: SpriteData {
                    sprite_sheet: "./assets/spritesheets/objects.png".to_string(),
                    frame_width: 24,
                    frame_height: 24,
                    sprite_width: tile_width,
                    sprite_height: tile_height,
                    start_frame: Rect::new(101, 226, 24, 24),
                    frame_x: 101,
                    frame_y: 226,
                    num_frames: 1,
                    current_frame: 0,
                },
                movement_direction: MovementDirection::Idle,
                intended_movement: MovementDirection::Idle,
                facing: MovementDirection::Right,
                speed: 0.0,
            },
        );

        Self {
            tile_width,
            tile_height,
            predefined_entities,
        }
    }

    pub fn create_entity(&self, name: &str, x_pos: i32, y_pos: i32) -> Option<Entity> {
        self.predefined_entities.get(name).map(|entity| Entity {
            name: entity.name.clone(),
            states: entity.states.clone(),
            position: (x_pos, y_pos),
            tile: (
                x_pos / (self.tile_width as i32),
                y_pos / (self.tile_height as i32),
            ),
            neighbors: entity.neighbors.clone(),
            draw_order: entity.draw_order,
            sprite_data: entity.sprite_data.clone(),
            movement_direction: entity.movement_direction.clone(),
            intended_movement: entity.intended_movement.clone(),
            facing: entity.facing.clone(),
            speed: entity.speed,
        })
    }
}
