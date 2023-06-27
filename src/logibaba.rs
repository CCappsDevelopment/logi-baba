extern crate sdl2;

use std::{collections::HashMap, time::Duration};

use events::Events;
use screen_renderer::ScreenRenderer;
use sprite_animation::SpriteData;

use crate::{events, level_map::LevelMap, screen_renderer, sprite_animation};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MovementDirection {
    Up,
    Right,
    Down,
    Left,
    Idle,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum EntityState {
    You,
    Win,
}

#[derive(Debug)]
pub struct GameEntity {
    pub name: String,
    pub states: HashMap<EntityState, bool>,
    pub position: (i32, i32),
    pub tile: (i32, i32),
    pub sprite_data: SpriteData,
    pub movement_direction: MovementDirection,
    pub facing: MovementDirection,
    pub speed: f32,
}

pub struct Game {
    pub screen_renderer: ScreenRenderer,
    pub entities: Vec<GameEntity>,
}

impl Game {
    pub fn new() -> Game {
        let screen_renderer = ScreenRenderer::new();
        let entities = Vec::new();
        Game {
            screen_renderer,
            entities,
        }
    }

    pub fn start(&mut self) {
        self.load_level(1);

        // Game loop
        'running: loop {
            // Handle events
            if !Events::process_events(
                &mut self.entities,
                &mut self.screen_renderer.context.event_pump,
                &mut self.screen_renderer.context.canvas,
            ) {
                break 'running;
            }

            // Render the screen
            self.screen_renderer.draw(&mut self.entities);

            // Set the framerate to 60fps
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn load_level(&mut self, level_to_load: i32) {
        let level_map = LevelMap::new(level_to_load, &self.screen_renderer.context);
        self.entities = level_map.entities;
    }
}
