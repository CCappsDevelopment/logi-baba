extern crate sdl2;

use std::{ collections::HashMap, time::Duration };
use std::time::Instant;
use sdl2::rect::Rect;

use events::Events;
use screen_renderer::ScreenRenderer;
use sprite_animation::SpriteData;

use crate::{ events, sprite_animation, screen_renderer };

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

#[derive(Debug)]
pub struct Game {
    pub entities: Vec<GameEntity>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            entities: vec![
                GameEntity {
                    name: "Logi".to_string(),
                    states: vec![(EntityState::You, true)].into_iter().collect(),
                    position: (0, 0),
                    tile: (0, 0),
                    sprite_data: SpriteData {
                        sprite_sheet: "./assets/spritesheets/characters.png".to_string(),
                        frame_width: 24,
                        frame_height: 24,
                        sprite_width: 128,
                        sprite_height: 128,
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
                    position: (640, 640),
                    tile: (0, 0),
                    sprite_data: SpriteData {
                        sprite_sheet: "./assets/spritesheets/objects.png".to_string(),
                        frame_width: 24,
                        frame_height: 24,
                        sprite_width: 128,
                        sprite_height: 128,
                        start_frame: Rect::new(101, 226, 24, 24),
                        num_frames: 1,
                        current_frame: 0,
                    },
                    movement_direction: MovementDirection::Idle,
                    facing: MovementDirection::Right,
                    speed: 0.0,
                }
            ],
        }
    }

    pub fn start(&mut self) {
        let mut screen_renderer = ScreenRenderer::new();

        // Game loop
        'running: loop {
            // Handle events
            if
                !Events::process_events(
                    &mut self.entities,
                    &mut screen_renderer.context.event_pump,
                    &mut screen_renderer.context.canvas
                )
            {
                break 'running;
            }

            // Render the screen
            screen_renderer.draw(&mut self.entities);

            // Set the framerate to 60fps
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
