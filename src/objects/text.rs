use crate::objects::{HandleReturn, Object, ObjectCommand};
use crossterm::{cursor, execute};
use std::io::{self};

#[derive(Debug, Clone)]
pub struct TextObject {
    pub content: String,
    pub size: (usize, usize),
    pub position: (usize, usize),
}

impl TextObject {
    pub fn new(content: &str) -> Box<Self> {
        Box::new(TextObject {
            content: content.to_string(),
            size: (0, 0),
            position: (0, 0),
        })
    }
}

impl Object for TextObject {
    fn display(&self) {
        let text_bits: Vec<&str> = self
            .content
            .as_bytes()
            .chunks(self.size.0)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect();

        for i in 0..self.size.1 {
            execute!(
                io::stdout(),
                cursor::MoveTo(self.position.0 as u16, self.position.1 as u16 + i as u16),
            )
            .unwrap();

            if i < text_bits.len() {
                print!("{}", text_bits[i]);
            } else {
                print!(" ");
            }
        }
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
                self.content = new_text;
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
}
