use logibaba::MovementDirection;

use sdl2::{ EventPump, event::Event, mouse::MouseButton, keyboard::Keycode };

use crate::{logibaba, entity};
use crate::entity::{ Entity, EntityState };

pub struct Events;

impl Events {
    pub fn process_events(
        entities: &mut Vec<Entity>,
        event_pump: &mut EventPump,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>
    ) -> bool {
        let mut movement_direction = None;
        let mut facing = None;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false;
                }
                Event::Window { win_event: sdl2::event::WindowEvent::Resized(w, h), .. } => {
                    canvas
                        .window_mut()
                        .set_size(w as u32, h as u32)
                        .unwrap();
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, /*x, y*/ .. } => {}
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {}
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::W => {
                            movement_direction = Some(MovementDirection::Up);
                            facing = Some(MovementDirection::Up);
                        }
                        Keycode::A => {
                            movement_direction = Some(MovementDirection::Left);
                            facing = Some(MovementDirection::Left);
                        }
                        Keycode::S => {
                            movement_direction = Some(MovementDirection::Down);
                            facing = Some(MovementDirection::Down);
                        }
                        Keycode::D => {
                            movement_direction = Some(MovementDirection::Right);
                            facing = Some(MovementDirection::Right);
                        }
                        Keycode::Space => {
                            println!("Space");
                            for entity in &mut *entities {
                                if !entity.states.contains_key(&EntityState::Push) {
                                    entity.states.insert(EntityState::Push, true);
                                }
                                else {
                                    entity.states.remove(&EntityState::Push);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::W | Keycode::A | Keycode::S | Keycode::D => {
                            movement_direction = Some(MovementDirection::Idle);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }

            if !movement_direction.is_none() {
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
