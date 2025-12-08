use crate::objects::{Handle, HandleReturn, Object, ObjectCommand};
use crossterm::{cursor, execute};
use std::io::{self};

#[derive(PartialEq, Clone, Copy)]
pub enum FlowDirection {
    Row,
    Column,
    Toggle,
}

pub struct ContainerObject {
    pub content: Vec<Box<dyn Object>>,
    pub(crate) size: (usize, usize),
    pub position: (usize, usize),
    pub(crate) spacing: Vec<usize>,
    pub(crate) flow: FlowDirection,
    pub(crate) index: Vec<usize>,
    border: bool,
}

impl ContainerObject {
    pub fn new() -> Box<Self> {
        Box::new(ContainerObject {
            content: Vec::new(),
            size: (0, 0),
            position: (0, 0),
            spacing: Vec::new(),
            flow: FlowDirection::Row,
            index: Vec::new(),
            border: true,
        })
    }

    pub fn update_sizes(&mut self) {
        if self.spacing.is_empty() {
            self.spacing = vec![1];
        }

        self.spacing
            .resize(self.content.len(), *self.spacing.last().unwrap());

        let spacing_sum = self.spacing.iter().sum::<usize>();

        let length = self.content.len();

        for (i, obj) in self.content.iter_mut().enumerate() {
            let spacing = self.spacing[i] as f32 / spacing_sum as f32;

            let border_offset = if self.border { 1 } else { 0 };

            let mut size = match self.flow {
                FlowDirection::Row | FlowDirection::Toggle => {
                    ((spacing * self.size.0 as f32) as usize, self.size.1)
                }
                FlowDirection::Column => (self.size.0, ((self.size.1 as f32) * spacing) as usize),
            };

            size = (
                size.0.saturating_sub(border_offset * 2),
                size.1.saturating_sub(border_offset * 2),
            );

            if length > 1 && i != length - 1 && self.border {
                match self.flow {
                    FlowDirection::Row | FlowDirection::Toggle => size.0 = size.0 + 1,
                    FlowDirection::Column => size.1 = size.1 + 1,
                }
            }

            let mut position = match self.flow {
                FlowDirection::Row | FlowDirection::Toggle => {
                    let x_offset: usize = self.spacing.iter().take(i).sum();
                    (
                        self.position.0
                            + (x_offset * (self.size.0 as f32 / spacing_sum as f32) as usize),
                        self.position.1,
                    )
                }
                FlowDirection::Column => {
                    let y_offset: usize = self.spacing.iter().take(i).sum();
                    (
                        self.position.0,
                        self.position.1
                            + (y_offset * (self.size.1 as f32 / spacing_sum as f32) as usize),
                    )
                }
            };

            position = (position.0 + border_offset, position.1 + border_offset);

            if position.1 + size.1 + 1 + border_offset == self.position.1 + self.size.1 {
                size.1 = size.1 + 1;
            }

            obj.handle(ObjectCommand::SetSize(size)).unwrap();
            obj.handle(ObjectCommand::SetPosition(position)).unwrap();

            if obj.as_any().is::<ContainerObject>() {
                obj.as_any_mut()
                    .downcast_mut::<ContainerObject>()
                    .unwrap()
                    .update_sizes();
            }
        }
    }

    pub fn add_object(&mut self, obj: Box<dyn Object>) {
        self.content.push(obj);
    }

    fn display_border(&self) {
        if self.border {
            let (x, y) = self.position;
            let (width, height) = self.size;

            // Draw top border
            execute!(io::stdout(), cursor::MoveTo(x as u16, y as u16),).unwrap();
            print!("╭");
            for _ in 1..width - 1 {
                print!("─");
            }
            print!("╮");

            // Draw side borders
            for row in 1..height - 1 {
                execute!(io::stdout(), cursor::MoveTo(x as u16, (y + row) as u16),).unwrap();
                print!("│");
                execute!(
                    io::stdout(),
                    cursor::MoveTo((x + width - 1) as u16, (y + row) as u16),
                )
                .unwrap();
                print!("│");
            }

            // Draw bottom border
            execute!(
                io::stdout(),
                cursor::MoveTo(x as u16, (y + height - 1) as u16),
            )
            .unwrap();
            print!("╰");
            for _ in 1..width - 1 {
                print!("─");
            }
            print!("╯");
        }
    }
}

impl Object for ContainerObject {
    fn display(&self) {
        self.display_border();

        for obj in &self.content {
            obj.display();
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
            ObjectCommand::SetSpacing(spacing) => {
                self.spacing = spacing;
                Ok(HandleReturn::None)
            }
            ObjectCommand::SetFlow(flow) => {
                if flow == FlowDirection::Toggle {
                    self.flow = match self.flow {
                        FlowDirection::Row => FlowDirection::Column,
                        FlowDirection::Column => FlowDirection::Row,
                        FlowDirection::Toggle => FlowDirection::Row, // Default case
                    };
                } else {
                    self.flow = flow;
                }
                Ok(HandleReturn::None)
            }
            ObjectCommand::SetBorder(border) => {
                self.border = border;
                Ok(HandleReturn::None)
            }
            ObjectCommand::AddObject(mut obj) => {
                let mut indexes = self.index.clone();
                indexes.push(self.content.len());
                let _ = obj.handle(ObjectCommand::SetIndexes(indexes.clone()));
                self.add_object(obj);
                Ok(HandleReturn::ObjectHandle(Handle { indexes }))
            }
            ObjectCommand::GetObjects(callback) => {
                callback(&self.content);
                Ok(HandleReturn::None)
            }
            ObjectCommand::SetIndexes(indexes) => {
                self.index = indexes;
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
