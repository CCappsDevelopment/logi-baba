use logibaba::MovementDirection;

use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, EventPump};

use crate::entity::{Entity, EntityState};
use crate::logibaba;

pub struct Events;

impl Events {
    pub fn process_events(
        entities: &mut Vec<Entity>,
        event_pump: &mut EventPump,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
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
                    Keycode::Space => {
                        for entity in &mut *entities {
                            if !entity.states.contains_key(&EntityState::You) {
                                if !entity.states.contains_key(&EntityState::Stop) {
                                    entity.states.insert(EntityState::Stop, true);
                                } else {
                                    entity.states.remove(&EntityState::Stop);
                                }
                                println!("{:?}: {:?}", entity.name, entity.states)
                            }
                        }
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
