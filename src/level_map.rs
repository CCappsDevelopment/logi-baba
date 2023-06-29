use std::collections::HashMap;
use std::collections::HashSet;

use crate::sdl_context::SdlContext;
use crate::entity::{ Entity, EntityRepository };

pub struct LevelMap {
    pub entities: Vec<Entity>,
    pub entity_map: HashMap<(i32, i32), HashSet<usize>>,
}

impl LevelMap {
    pub fn new(level_to_load: i32, sdl_context: &SdlContext) -> LevelMap {
        let loaded_map_entities = LevelMap::get_level_map(level_to_load, &sdl_context);
        let mut entity_map = HashMap::new();
        for (index, entity) in loaded_map_entities.iter().enumerate() {
            let entity_position = (entity.tile.0, entity.tile.1);
            let mut entity_set = HashSet::new();
            entity_set.insert(index);
            entity_map.insert(entity_position, entity_set);
        }

        LevelMap {
            entities: loaded_map_entities,
            entity_map,
        }
    }

    fn get_level_map(level: i32, sdl_context: &SdlContext) -> Vec<Entity> {
        let tile_width = sdl_context.canvas.viewport().width() / 12;
        let tile_height = sdl_context.canvas.viewport().height() / 8;

        let mut entities = Vec::new();
        let entity_repo = EntityRepository::new(tile_width, tile_height);

        match level {
            1 => {
                entities = vec![
                    entity_repo.create_entity("Logi", 0, 0).unwrap(),
                    entity_repo
                        .create_entity("Goal", 5 * (tile_width as i32), 5 * (tile_height as i32))
                        .unwrap()
                ];
            }
            2 => {
                entities = vec![
                    entity_repo
                        .create_entity("Logi", 6 * (tile_width as i32), 1 * (tile_height as i32))
                        .unwrap(),
                    entity_repo
                        .create_entity("Goal", 0 * (tile_width as i32), 6 * (tile_height as i32))
                        .unwrap()
                ];
            }
            _ => {}
        }
        entities.sort_by_key(|entity| entity.draw_order);
        return entities;
    }
}
