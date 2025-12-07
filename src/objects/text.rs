use crate::objects::{HandleReturn, Object, ObjectCommand};

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
        println!("{}: {:?}", self.content, self.size);
    }

    fn handle(&mut self, command: ObjectCommand) -> Result<HandleReturn, ()> {
        match command {
            ObjectCommand::SetSize((width, height)) => {
                self.size = (width, height);
                Ok(HandleReturn::None)
            }
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
