pub mod text;
// use text::TextObject;
pub mod container;
// use container::ContainerObject;
pub mod button;

use super::TuiHandler;

#[derive(Debug, Clone)]
pub struct Handle {
    pub indexes: Vec<usize>,
}

pub enum HandleReturn {
    None,
    ObjectHandle(Handle),
    Size((usize, usize)),
    Position((usize, usize)),
}

impl HandleReturn {
    pub fn unwrap_handle(self) -> Handle {
        match self {
            HandleReturn::ObjectHandle(handle) => handle,
            _ => panic!("Called unwrap_handle on a non-handle return value"),
        }
    }

    pub fn unwrap_size(self) -> (usize, usize) {
        match self {
            HandleReturn::Size(size) => size,
            _ => panic!("Called unwrap_size on a non-size return value"),
        }
    }

    pub fn unwrap_position(self) -> (usize, usize) {
        match self {
            HandleReturn::Position(pos) => pos,
            _ => panic!("Called unwrap_position on a non-position return value"),
        }
    }
}

pub trait Object {
    fn display(&self);

    fn handle(&mut self, command: ObjectCommand) -> Result<HandleReturn, ()>;

    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub enum ObjectCommand {
    SetText(String),
    SetSpacing(Vec<usize>),
    SetPosition((usize, usize)),
    GetPosition(),
    SetFlow(container::FlowDirection),
    SetBorder(bool),
    AddObject(Box<dyn Object>),
    SetSize((usize, usize)),
    GetSize(),
    GetObjects(Box<dyn FnOnce(&Vec<Box<dyn Object>>)>),
    SetIndexes(Vec<usize>),
    SetSelected(bool),
}
