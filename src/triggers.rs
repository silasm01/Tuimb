use crate::objects::Handle;

#[derive(Debug)]
pub enum MouseTriggers {
    Region {
        x_range: (usize, usize),
        y_range: (usize, usize),
    },
    Object {
        object_handle: Handle,
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
