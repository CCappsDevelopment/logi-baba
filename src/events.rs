use logibaba::MovementDirection;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, EventPump};

use crate::debug_console::{DebugConsole, DebugKey};
use crate::entity::{Entity, EntityState};
use crate::logibaba;

pub struct Events;

impl Events {
    pub fn process_events(
        entities: &mut Vec<Entity>,
        event_pump: &mut EventPump,
        canvas: &mut Canvas<Window>,
        debug_console: &mut DebugConsole,
    ) -> bool {
        let mut movement_direction = None;
        let mut facing = None;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return false;
                }
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    canvas.window_mut().set_size(w as u32, h as u32).unwrap();
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left, /*x, y*/
                    ..
                } => {}
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W | Keycode::Up => {
                        movement_direction = Some(MovementDirection::Up);
                        facing = Some(MovementDirection::Up);
                    }
                    Keycode::A | Keycode::Left => {
                        movement_direction = Some(MovementDirection::Left);
                        facing = Some(MovementDirection::Left);
                    }
                    Keycode::S | Keycode::Down => {
                        movement_direction = Some(MovementDirection::Down);
                        facing = Some(MovementDirection::Down);
                    }
                    Keycode::D | Keycode::Right => {
                        movement_direction = Some(MovementDirection::Right);
                        facing = Some(MovementDirection::Right);
                    }
                    Keycode::P => {
                        let mut debug_strings = Vec::new();
                        for entity in &mut *entities {
                            if !entity.states.contains_key(&EntityState::Push) {
                                entity.states.insert(EntityState::Push, true);
                            } else {
                                entity.states.remove(&EntityState::Push);
                            }
                    
                           debug_strings.push((DebugKey::Rules(entity.name.to_string()), format!("{:?}: {:?}", entity.name, entity.states)));
                        }
                    
                        debug_console.out(debug_strings);
                    }
                    Keycode::Space => {
                        let mut debug_strings = Vec::new();
                        for entity in &mut *entities {
                            if !entity.states.contains_key(&EntityState::Stop) {
                                entity.states.insert(EntityState::Stop, true);
                            } else {
                                entity.states.remove(&EntityState::Stop);
                            }
                    
                           debug_strings.push((DebugKey::Rules(entity.name.to_string()), format!("{:?}: {:?}", entity.name, entity.states)));
                        }
                    
                        debug_console.out(debug_strings);
                    }                    
                    Keycode::Backquote => {
                        debug_console.show_console = !debug_console.show_console;
                    }
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W
                    | Keycode::A
                    | Keycode::S
                    | Keycode::D
                    | Keycode::Up
                    | Keycode::Left
                    | Keycode::Down
                    | Keycode::Right => {
                        movement_direction = Some(MovementDirection::Idle);
                    }
                    _ => {}
                },
                _ => {}
            }

            if movement_direction.is_some() {
                for entity in &mut *entities {
                    if entity.states.contains_key(&EntityState::You) {
                        entity.movement_direction = movement_direction.unwrap();
                        if facing.is_some() {
                            entity.facing = facing.unwrap();
                        }
                    }
                }
            }
        }

        return true;
    }
}
