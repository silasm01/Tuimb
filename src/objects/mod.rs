pub mod text;
use text::TextObject;
pub mod container;
use container::ContainerObject;

// pub enum Objects {
//     TextObject(TextObject),
//     ContainerObject(ContainerObject),
// }

pub enum HandleReturn {
    None,
    ObjectHandle(usize),
}

impl HandleReturn {
    pub fn unwrap_handle(self) -> usize {
        match self {
            HandleReturn::ObjectHandle(handle) => handle,
            _ => panic!("Called unwrap_handle on a non-handle return value"),
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
    SetBorder(bool),
    AddObject(Box<dyn Object>),
    SetSize((usize, usize)),
    GetObjects(Box<dyn FnOnce(&Vec<Box<dyn Object>>)>),
}
