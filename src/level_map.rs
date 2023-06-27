use sdl2::rect::Rect;

use crate::{
    logibaba::{EntityState, GameEntity, MovementDirection},
    sdl_context::SdlContext,
    sprite_animation::SpriteData,
};

pub struct LevelMap {
    pub entities: Vec<GameEntity>,
}

impl LevelMap {
    pub fn new(level_to_load: i32, sdl_context: &SdlContext) -> LevelMap {
        let loaded_map_entities = LevelMap::get_level_map(level_to_load, &sdl_context);
        LevelMap {
            entities: loaded_map_entities,
        }
    }

    fn get_level_map(level: i32, sdl_context: &SdlContext) -> Vec<GameEntity> {
        let tile_width = sdl_context.canvas.viewport().width() / 12;
        let tile_height = sdl_context.canvas.viewport().height() / 8;
        match level {
            1 => {
                return vec![
                    GameEntity {
                        name: "Logi".to_string(),
                        states: vec![(EntityState::You, true)].into_iter().collect(),
                        position: (0, 0),
                        tile: (0, 0),
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
                    },
                    GameEntity {
                        name: "Goal".to_string(),
                        states: vec![(EntityState::Win, true)].into_iter().collect(),
                        position: (5 * tile_width as i32, 5 * tile_height as i32),
                        tile: (5, 5),
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
                    },
                ]
            }
            _ => Vec::new(),
        }
    }
}
