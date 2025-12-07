use super::{MouseTriggers, Trigger, TuiHandler};
use crossterm::event::{self, Event, KeyCode};
use crossterm::{execute, terminal};
impl TuiHandler {
    pub fn handle_term_events(&mut self) {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            let event = event::read().unwrap();
            match event {
                Event::Resize(width, height) => {
                    self.terminal_size = Some((width as usize, height as usize));
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
                                    use crossterm::event::MouseEventKind;
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
                                    // Handle object-specific mouse trigger
                                }
                                MouseTriggers::Radius { x, y, radius } => {
                                    // Handle radius-specific mouse trigger
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
