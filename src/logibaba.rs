extern crate sdl2;

use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use events::Events;
use screen_renderer::ScreenRenderer;

use crate::{entity::Entity, events, level_map::LevelMap, screen_renderer, debug_console::{DebugConsole, DebugKey}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MovementDirection {
    Up,
    Right,
    Down,
    Left,
    Idle,
}

pub struct Game<'a> {
    pub screen_renderer: ScreenRenderer,
    pub entities: Vec<Entity>,
    pub entity_map: HashMap<(i32, i32), HashSet<usize>>,
    pub debug_console: DebugConsole<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let screen_renderer = ScreenRenderer::new();
        let entities = Vec::new();
        let entity_map = HashMap::new();
        let debug_console = DebugConsole::new();

        Game {
            screen_renderer,
            entities,
            entity_map,
            debug_console,
        }
    }

    pub fn start(&mut self) {
        self.load_level(2);

        // Game loop
        'running: loop {
            // Handle events
            if !Events::process_events(
                &mut self.entities,
                &mut self.screen_renderer.context.event_pump,
                &mut self.screen_renderer.context.canvas,
                &mut self.debug_console
            ) {
                break 'running;
            }

            // Render the screen
            self.screen_renderer
                .draw(&mut self.entities, &mut self.entity_map, &mut self.debug_console);

            // Set the framerate to 60fps
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn load_level(&mut self, level_to_load: i32) {
        let level_map = LevelMap::new(level_to_load, &self.screen_renderer.context.canvas);
        self.entities = level_map.entities;
        self.entity_map = level_map.entity_map;

        let mut debug_strings = Vec::new();
        debug_strings.push((DebugKey::Render("Level".to_string()), format!("Level: {:?}", level_to_load)));
        debug_strings.push((DebugKey::Render("Entity Map".to_string()), format!("Entity Map: {:?}", self.entity_map)));

        self.debug_console.out(debug_strings);

    }
}
