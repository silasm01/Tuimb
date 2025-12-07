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
use std::ffi::IntoStringError;
use std::io;

#[derive(Debug)]
pub enum MouseTriggers {
    Region {
        x_range: (usize, usize),
        y_range: (usize, usize),
    },
    Object {
        object_handle: usize,
    },
    Radius {
        x: usize,
        y: usize,
        radius: usize,
    },
}

pub enum Trigger {
    KeyPress(char),
    MouseClick(MouseTriggers),
}

pub struct TuiHandler {
    pub objects: Box<dyn Object>,
    pub triggers: Vec<(Trigger, Box<dyn FnMut(&mut TuiHandler)>)>,
    pub terminal_size: Option<(usize, usize)>,
    pub changed: bool,
}

impl TuiHandler {
    pub fn new() -> Self {
        let size = dimensions();
        let tui = TuiHandler {
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

    pub fn update_container_sizes(&mut self) {
        for obj in &mut self.objects {
            if obj.as_any().is::<ContainerObject>() {
                obj.as_any_mut()
                    .downcast_mut::<ContainerObject>()
                    .unwrap()
                    .update_sizes();

                self.changed = true;
            };
        }
    }

    pub fn run(&mut self) {
        let _ = execute!(std::io::stdout(), crossterm::event::EnableMouseCapture);
        execute!(io::stdout(), EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
        loop {
            self.handle_term_events();
            if self.changed {
                execute!(io::stdout(), Clear(crossterm::terminal::ClearType::All)).unwrap();
                // self.update_container_sizes();
                for obj in &self.objects {
                    obj.display();
                }
                self.changed = false;
            }
        }
    }

    pub fn add_object(&mut self, mut object: Box<dyn Object>) -> usize {
        if object.as_any().is::<ContainerObject>() {
            object
                .as_any_mut()
                .downcast_mut::<ContainerObject>()
                .unwrap()
                .size = match self.terminal_size {
                Some((w, h)) => (w, h),
                None => (80, 24),
            };
        }

        self.objects.push(object);
        return self.objects.len() - 1;
    }

    pub fn add_trigger(&mut self, trigger: Trigger, callback: Box<dyn FnMut(&mut TuiHandler)>) {
        self.triggers.push((trigger, callback));
    }

    pub fn exit(&mut self) {
        disable_raw_mode().unwrap();
        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    }
}

fn main() {
    let mut handler = TuiHandler::new();

    println!(
        "Terminal size: {:?}",
        handler
            .objects
            .as_any()
            .downcast_ref::<ContainerObject>()
            .unwrap()
            .size
    );
    // let cont = handler.add_object(ContainerObject::new());

    // handler.objects[cont].handle(ObjectCommand::AddObject(TextObject::new(
    //     "This is inside of an container!!!",
    // )));

    // handler.objects[cont].handle(ObjectCommand::AddObject(TextObject::new(
    //     "Another text object inside the container.",
    // )));

    // handler.objects[cont].handle(ObjectCommand::SetSpacing(vec![1, 2]));

    handler.add_trigger(
        Trigger::KeyPress('q'),
        Box::new(|handler| {
            handler.exit();
            std::process::exit(0);
        }),
    );

    handler.run();
}
