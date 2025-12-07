pub mod objects;
use crate::objects::container::*;
use crate::objects::text::*;
use crate::objects::*;

use term_size::dimensions;

pub mod terminal;

use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;

pub mod triggers;
use crate::triggers::Trigger;

pub struct TuiHandler {
    pub objects: Box<dyn Object>,
    triggers: Vec<(Trigger, Box<dyn FnMut(&mut TuiHandler)>)>,
    terminal_size: Option<(usize, usize)>,
    pub changed: bool,
}

impl TuiHandler {
    pub fn new() -> Self {
        let size = dimensions();
        let mut tui = TuiHandler {
            objects: ContainerObject::new(),
            triggers: Vec::new(),
            terminal_size: dimensions(),
            changed: true,
        };
        tui.objects
            .as_any_mut()
            .downcast_mut::<ContainerObject>()
            .unwrap()
            .size = match size {
            Some((w, h)) => (w, h),
            None => (80, 24),
        };
        tui
    }

    pub fn run(&mut self) {
        execute!(std::io::stdout(), crossterm::event::EnableMouseCapture).unwrap();
        execute!(io::stdout(), EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();

        loop {
            self.handle_term_events();
            if self.changed {
                execute!(io::stdout(), Clear(crossterm::terminal::ClearType::All)).unwrap();
                let container = self
                    .objects
                    .as_any_mut()
                    .downcast_mut::<ContainerObject>()
                    .unwrap();
                container.update_sizes();
                for obj in &container.content {
                    obj.display();
                }
                self.changed = false;
            }
        }
    }

    pub fn with(&mut self, handle: &Handle) -> &mut dyn Object {
        let mut current: &mut dyn Object = &mut *self.objects;
        for index in handle.indexes.clone() {
            let container = current
                .as_any_mut()
                .downcast_mut::<ContainerObject>()
                .unwrap();
            current = &mut *container.content[index];
        }
        current
    }

    pub fn add_trigger(&mut self, trigger: Trigger, callback: Box<dyn FnMut(&mut TuiHandler)>) {
        self.triggers.push((trigger, callback));
    }

    pub fn exit(&mut self) {
        disable_raw_mode().unwrap();
        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    }
}
