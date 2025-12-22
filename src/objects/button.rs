use std::io;

use crossterm::{cursor, execute};

use crate::objects::*;

pub struct ButtonObject {
    pub text: String,
    pub is_selected: bool,
    pub callback: Box<dyn FnMut()>,
    pub size: (usize, usize),
    pub position: (usize, usize),
}

impl ButtonObject {
    pub fn new(text: &str, callback: Box<dyn FnMut()>) -> Box<Self> {
        Box::new(ButtonObject {
            text: text.to_string(),
            is_selected: false,
            callback,
            size: (0, 0),
            position: (0, 0),
        })
    }
}

impl Object for ButtonObject {
    fn display(&self) {
        let text_bits: Vec<&str> = self
            .text
            .as_bytes()
            .chunks(self.size.0)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect();

        let _ = execute!(
            io::stdout(),
            cursor::MoveTo(self.position.0 as u16, self.position.1 as u16 as u16),
        );

        if self.is_selected {
            let _ = execute!(
                io::stdout(),
                crossterm::style::SetAttribute(crossterm::style::Attribute::Reverse)
            );
        }

        println!("{}", text_bits[0]);
        execute!(
            io::stdout(),
            crossterm::style::SetAttribute(crossterm::style::Attribute::NoReverse)
        )
        .unwrap();
    }

    fn handle(&mut self, command: ObjectCommand) -> Result<HandleReturn, ()> {
        match command {
            ObjectCommand::SetSize((width, height)) => {
                self.size = (width, height);
                Ok(HandleReturn::None)
            }
            ObjectCommand::GetSize() => Ok(HandleReturn::Size(self.size)),
            ObjectCommand::SetPosition((x, y)) => {
                self.position = (x, y);
                Ok(HandleReturn::None)
            }
            ObjectCommand::GetPosition() => Ok(HandleReturn::Position(self.position)),
            ObjectCommand::SetText(new_text) => {
                self.text = new_text;
                Ok(HandleReturn::None)
            }
            ObjectCommand::GetSelected() => Ok(HandleReturn::Selected(self.is_selected)),
            ObjectCommand::SetSelected(selected) => {
                self.is_selected = selected;
                Ok(HandleReturn::None)
            }
            _ => Err(()),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_selectable(&self) -> bool {
        true
    }
}

impl Selectable for ButtonObject {}
