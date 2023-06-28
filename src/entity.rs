use std::collections::HashMap;

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
    pub num_frames: u32,
    pub current_frame: u32,
}

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub states: HashMap<EntityState, bool>,
    pub position: (i32, i32),
    pub tile: (i32, i32),
    pub draw_order: i32,
    pub sprite_data: SpriteData,
    pub movement_direction: MovementDirection,
    pub facing: MovementDirection,
    pub speed: f32,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum EntityState {
    You,
    Win,
}

pub struct EntityRepository {
    tile_width: u32,
    tile_height: u32,
    predefined_entities: HashMap<String, Entity>,
}

impl EntityRepository {
    pub fn new(tile_width: u32, tile_height: u32) -> Self {
        let mut predefined_entities = HashMap::new();

        predefined_entities.insert("Logi".to_string(), Entity {
            name: "Logi".to_string(),
            states: vec![(EntityState::You, true)].into_iter().collect(),
            position: (0, 0),
            tile: (0, 0),
            draw_order: 3,
            sprite_data: SpriteData {
                sprite_sheet: "./assets/spritesheets/characters.png".to_string(),
                frame_width: 24,
                frame_height: 24,
                sprite_width: tile_width,
                sprite_height: tile_height,
                start_frame: Rect::new(576, 1, 24, 24),
                num_frames: 4,
                current_frame: 0,
            },
            movement_direction: MovementDirection::Idle,
            facing: MovementDirection::Right,
            speed: 1.0,
        });

        predefined_entities.insert("Goal".to_string(), Entity {
            name: "Goal".to_string(),
            states: vec![(EntityState::Win, true)].into_iter().collect(),
            position: (5 * (tile_width as i32), 5 * (tile_height as i32)),
            tile: (5, 5),
            draw_order: 1,
            sprite_data: SpriteData {
                sprite_sheet: "./assets/spritesheets/objects.png".to_string(),
                frame_width: 24,
                frame_height: 24,
                sprite_width: tile_width,
                sprite_height: tile_height,
                start_frame: Rect::new(101, 226, 24, 24),
                num_frames: 1,
                current_frame: 0,
            },
            movement_direction: MovementDirection::Idle,
            facing: MovementDirection::Right,
            speed: 0.0,
        });

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
            tile: (x_pos / (self.tile_width as i32), y_pos / (self.tile_height as i32)),
            draw_order: entity.draw_order,
            sprite_data: entity.sprite_data.clone(),
            movement_direction: entity.movement_direction.clone(),
            facing: entity.facing.clone(),
            speed: entity.speed,
        })
    }
}
