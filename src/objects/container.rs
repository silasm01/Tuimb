use crate::objects::{HandleReturn, Object, ObjectCommand};

//#[derive(Debug, Clone)]
pub struct ContainerObject {
    pub content: Vec<Box<dyn Object>>,
    pub(crate) size: (usize, usize),
    pub position: (usize, usize),
    spacing: Vec<usize>,
    border: bool,
}

impl ContainerObject {
    pub fn new() -> Box<Self> {
        Box::new(ContainerObject {
            content: Vec::new(),
            size: (0, 0),
            position: (0, 0),
            spacing: Vec::new(),
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

        for (i, obj) in self.content.iter_mut().enumerate() {
            let spacing = self.spacing[i] as f32 / spacing_sum as f32;

            let size = ((spacing * self.size.0 as f32) as usize, self.size.1);

            obj.handle(ObjectCommand::SetSize(size));

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
}

impl Object for ContainerObject {
    fn display(&self) {
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
            ObjectCommand::SetSpacing(spacing) => {
                self.spacing = spacing;
                Ok(HandleReturn::None)
            }
            ObjectCommand::SetBorder(border) => {
                self.border = border;
                Ok(HandleReturn::None)
            }
            ObjectCommand::AddObject(obj) => {
                self.add_object(obj);
                Ok(HandleReturn::ObjectHandle(self.content.len() - 1))
            }
            ObjectCommand::GetObjects(callback) => {
                callback(&self.content);
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
