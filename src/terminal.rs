use crate::objects::ObjectCommand;

use super::{
    objects::container::*,
    triggers::{MouseTriggers, Trigger},
    TuiHandler,
};
use crossterm::event::MouseEventKind;
use crossterm::event::{self, Event, KeyCode};

impl TuiHandler {
    pub fn handle_term_events(&mut self) {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            let event = event::read().unwrap();
            match event {
                Event::Resize(width, height) => {
                    println!("Terminal resized to {}x{}", width, height);
                    self.terminal_size = Some((width as usize, height as usize));
                    self.objects
                        .as_any_mut()
                        .downcast_mut::<ContainerObject>()
                        .unwrap()
                        .size = (width as usize, height as usize);
                    self.changed = true;
                }
                Event::Key(key_event) => {
                    let mut triggers = std::mem::take(&mut self.triggers);
                    for (trigger, callback) in &mut triggers {
                        if let Trigger::KeyPress(c) = trigger {
                            if let KeyCode::Char(pressed_char) = key_event.code {
                                if pressed_char == *c {
                                    callback(self);
                                    self.changed = true;
                                }
                            }
                        }
                    }
                    self.triggers = triggers;
                }
                Event::Mouse(mouse_event) => {
                    let mut triggers = std::mem::take(&mut self.triggers);
                    for (trigger, callback) in &mut triggers {
                        if let Trigger::MouseClick(mouse_trigger) = trigger {
                            match mouse_trigger {
                                MouseTriggers::Region { x_range, y_range } => {
                                    if let MouseEventKind::Down(_) = mouse_event.kind {
                                        let x = mouse_event.column as usize;
                                        let y = mouse_event.row as usize;
                                        if x >= x_range.0
                                            && x <= x_range.1
                                            && y >= y_range.0
                                            && y <= y_range.1
                                        {
                                            callback(self);
                                            self.changed = true;
                                        }
                                    }
                                }
                                MouseTriggers::Object { object_handle } => {
                                    if let MouseEventKind::Down(_) = mouse_event.kind {
                                        let x = mouse_event.column as usize;
                                        let y = mouse_event.row as usize;

                                        let (obj_x, obj_y) = self
                                            .with(object_handle)
                                            .handle(ObjectCommand::GetPosition())
                                            .unwrap()
                                            .unwrap_position();
                                        let (obj_width, obj_height) = self
                                            .with(object_handle)
                                            .handle(ObjectCommand::GetSize())
                                            .unwrap()
                                            .unwrap_size();

                                        if x >= obj_x
                                            && x < obj_x + obj_width
                                            && y >= obj_y
                                            && y < obj_y + obj_height
                                        {
                                            callback(self);
                                            self.changed = true;
                                        }
                                    }
                                }
                                MouseTriggers::Radius { x, y, radius } => {
                                    todo!("Handle radius-based mouse trigger");
                                }
                            }
                        }
                    }
                    self.triggers = triggers;
                }
                _ => {}
            }
        }
    }
}
