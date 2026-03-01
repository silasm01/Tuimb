pub mod objects;
use crate::objects::button::ButtonObject;
use crate::objects::container::*;
use crate::objects::*;

use term_size::dimensions;

pub mod terminal;

use crossterm::cursor::{Hide, Show};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;

pub mod triggers;
use crate::triggers::Trigger;

use std::collections::HashMap;

pub type Coord = (usize, usize);

pub struct Grid {
    pub selected: Handle,
    cells: HashMap<Coord, Handle>,
}

impl Grid {
    pub fn move_down(&mut self) {
        println!(
            "Moving down from {:?} with map: {:?}",
            self.selected, self.cells
        );
        let current_pos = self
            .cells
            .iter()
            .find_map(|(pos, handle)| {
                if *handle == self.selected {
                    Some(pos)
                } else {
                    None
                }
            })
            .unwrap();

        println!("Current position: {:?}", current_pos);

        println!(
            "Valid: {:?}",
            self.cells
                .keys()
                .filter(|&&(cx, cy)| cx > current_pos.0)
                .collect::<Vec<&Coord>>()
        );

        let next = self
            .cells
            .keys()
            // .filter(|&&(cx, cy)| cx == current_pos.0 && cy < current_pos.1)
            .filter(|&&(cx, cy)| cx == current_pos.0 && cy < current_pos.1)
            .max_by_key(|&&(_, cy)| cy)
            .copied();

        println!("Next position: {:?}", next);

        let next_handle = next.and_then(|pos| self.cells.get(&pos)).cloned().unwrap();
        self.selected = next_handle;
    }
}

pub struct TuiHandler {
    pub objects: Box<dyn Object>,
    triggers: Vec<(Trigger, Box<dyn FnMut(&mut TuiHandler)>)>,
    terminal_size: Option<(usize, usize)>,
    pub changed: bool,
    pub selectables_map: Grid,
}

impl TuiHandler {
    pub fn new() -> Self {
        let size = dimensions();
        let mut tui = TuiHandler {
            objects: ContainerObject::new(),
            triggers: Vec::new(),
            terminal_size: dimensions(),
            changed: true,
            selectables_map: Grid {
                selected: Handle {
                    indexes: Vec::new(),
                },
                cells: HashMap::new(),
            },
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
        execute!(io::stdout(), Hide).unwrap();
        execute!(io::stdout(), EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();

        loop {
            self.handle_term_events();
            if self.changed {
                // execute!(io::stdout(), Clear(crossterm::terminal::ClearType::All)).unwrap();

                let container = self
                    .objects
                    .as_any_mut()
                    .downcast_mut::<ContainerObject>()
                    .unwrap();
                container.update_sizes(&mut self.selectables_map);
                // println!("Selectables Map: {:?}", self.selectables_map);
                for obj in &container.content {
                    // obj.display();
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

    pub fn set_selected(&mut self, handle: &Handle) {
        if !self.selectables_map.selected.indexes.is_empty() {
            self.with(&self.selectables_map.selected.clone())
                .handle(ObjectCommand::SetSelected(false))
                .unwrap();
        }

        self.selectables_map.selected = handle.clone();
        self.with(&handle)
            .handle(ObjectCommand::SetSelected(true))
            .unwrap();
    }

    pub fn add_trigger(&mut self, trigger: Trigger, callback: Box<dyn FnMut(&mut TuiHandler)>) {
        self.triggers.push((trigger, callback));
    }

    pub fn exit(&mut self) {
        disable_raw_mode().unwrap();
        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        execute!(io::stdout(), Show).unwrap();
        execute!(std::io::stdout(), crossterm::event::DisableMouseCapture).unwrap();
    }

    pub fn selectable_movement(&mut self, direction: SelectionDirection) {
        let mut current_handle = self.selectables_map.selected.clone();
        self.selectable_movement_specific(direction, current_handle);
    }

    fn container_bound(
        &mut self,
        direction: SelectionDirection,
        current_handle: Handle,
        container_handle: &mut Handle,
    ) -> bool {
        let container_count = self
            .with(&container_handle)
            .handle(ObjectCommand::GetObjectCount())
            .unwrap()
            .unwrap_count();

        match direction {
            SelectionDirection::Down => {
                if container_count > *current_handle.indexes.last().unwrap() + 1 {
                    true
                } else {
                    false
                }
            }
            SelectionDirection::Up => {
                if *current_handle.indexes.last().unwrap() > 0 {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn selectable_movement_specific(
        &mut self,
        direction: SelectionDirection,
        mut current_handle: Handle,
    ) {
        let mut container_handle = {
            let mut ch = current_handle.clone();
            ch.indexes.pop();
            ch
        };

        let container_count = self
            .with(&container_handle)
            .handle(ObjectCommand::GetObjectCount())
            .unwrap()
            .unwrap_count();

        let current_index = *current_handle.indexes.last().unwrap();

        match direction {
            SelectionDirection::Down => {
                if container_count <= current_index {
                    return;
                }

                if self
                    .with(&container_handle)
                    .handle(ObjectCommand::GetFlow())
                    .unwrap()
                    .unwrap_flow()
                    == FlowDirection::Row
                    && current_handle.indexes.last().unwrap() != &0
                {
                    container_handle.indexes.pop();
                    *container_handle.indexes.last_mut().unwrap() += 1;
                    current_handle = container_handle.clone();
                    current_handle.indexes.push(0);
                    if self.with(&current_handle).is_selectable() {
                        self.set_selected(&current_handle);
                        return;
                    } else {
                        self.selectable_movement_specific(
                            direction.clone(),
                            current_handle.clone(),
                        );
                    }
                }

                if self.with(&current_handle).as_any().is::<ContainerObject>() {
                    if self
                        .with(&current_handle)
                        .handle(ObjectCommand::GetObjectCount())
                        .unwrap()
                        .unwrap_count()
                        != 0
                    {
                        container_handle = current_handle.clone();
                        container_handle.indexes.push(0);
                        current_handle.indexes.push(0);
                        if self.with(&current_handle).is_selectable() {
                            self.set_selected(&current_handle);
                            return;
                        } else {
                            self.selectable_movement_specific(direction, current_handle);
                            return;
                        }
                    }
                }
                if self.container_bound(
                    direction.clone(),
                    current_handle.clone(),
                    &mut container_handle,
                ) {
                    *current_handle.indexes.last_mut().unwrap() += 1;

                    if self.with(&current_handle).is_selectable() {
                        self.set_selected(&current_handle);
                        return;
                    } else {
                        self.selectable_movement_specific(direction, current_handle);
                    }
                } else {
                    if container_handle.indexes.is_empty() {
                        // Reached the top-level container and can't go further down
                        return;
                    }

                    current_handle = container_handle.clone();
                    *current_handle.indexes.last_mut().unwrap() += 1;
                    container_handle.indexes.pop();

                    self.selectable_movement_specific(direction, current_handle);
                }
            }
            SelectionDirection::Up => {
                if self
                    .with(&container_handle)
                    .handle(ObjectCommand::GetFlow())
                    .unwrap()
                    .unwrap_flow()
                    == FlowDirection::Row
                    && current_handle.indexes.last().unwrap() != &0
                {
                    if !self.with(&current_handle).as_any().is::<ContainerObject>() {
                        container_handle.indexes.pop();
                        *container_handle.indexes.last_mut().unwrap() -= 1;
                        current_handle = container_handle.clone();
                        let count = self
                            .with(&current_handle)
                            .handle(ObjectCommand::GetObjectCount())
                            .unwrap()
                            .unwrap_count();
                        current_handle.indexes.push(count - 1);
                        if self.with(&current_handle).is_selectable() {
                            self.set_selected(&current_handle);
                            return;
                        } else {
                            self.selectable_movement_specific(
                                direction.clone(),
                                current_handle.clone(),
                            );
                        }
                    } else {
                        *current_handle.indexes.last_mut().unwrap() =
                            current_handle.indexes.last().unwrap().saturating_sub(1);
                        self.selectable_movement_specific(
                            direction.clone(),
                            current_handle.clone(),
                        );
                    }
                }

                if self.with(&current_handle).as_any().is::<ContainerObject>() {
                    if self
                        .with(&current_handle)
                        .handle(ObjectCommand::GetObjectCount())
                        .unwrap()
                        .unwrap_count()
                        != 0
                    {
                        container_handle = current_handle.clone();
                        // container_handle.indexes.push(current_index);
                        if self
                            .with(&container_handle)
                            .handle(ObjectCommand::GetFlow())
                            .unwrap()
                            .unwrap_flow()
                            == FlowDirection::Row
                        {
                            current_handle.indexes.push(0);
                        } else {
                            current_handle.indexes.push(
                                self.with(&container_handle)
                                    .handle(ObjectCommand::GetObjectCount())
                                    .unwrap()
                                    .unwrap_count()
                                    - 1,
                            );
                        }

                        if self.with(&current_handle).is_selectable() {
                            self.set_selected(&current_handle);
                            return;
                        } else {
                            self.selectable_movement_specific(direction, current_handle);
                            return;
                        }
                    }
                }

                if self.container_bound(
                    direction.clone(),
                    current_handle.clone(),
                    &mut container_handle,
                ) {
                    *current_handle.indexes.last_mut().unwrap() -= 1;

                    if self.with(&current_handle).is_selectable() {
                        self.set_selected(&current_handle);
                        return;
                    } else {
                        self.selectable_movement_specific(direction, current_handle);
                    }
                } else {
                    if current_handle.indexes.first() == Some(&0) {
                        // Reached the top-level container and can't go further up
                        return;
                    }
                    while current_handle.indexes.last() == Some(&0) {
                        current_handle.indexes.pop();
                    }
                    container_handle = current_handle.clone();
                    container_handle.indexes.pop();
                    *current_handle.indexes.last_mut().unwrap() -= 1;

                    self.selectable_movement_specific(direction, current_handle);
                }
            }
            SelectionDirection::Right => {
                if self.with(&current_handle).as_any().is::<ContainerObject>() {
                    if self
                        .with(&current_handle)
                        .handle(ObjectCommand::GetObjectCount())
                        .unwrap()
                        .unwrap_count()
                        != 0
                    {
                        container_handle = current_handle.clone();
                        container_handle.indexes.push(0);
                        current_handle.indexes.push(0);
                        if self.with(&current_handle).is_selectable() {
                            self.set_selected(&current_handle);
                            return;
                        } else {
                            self.selectable_movement_specific(direction, current_handle);
                            return;
                        }
                    }
                }

                if current_handle.indexes.is_empty() {
                    // Reached the top-level container and can't go further right
                    return;
                }

                container_handle = current_handle.clone();
                container_handle.indexes.pop();
                if *current_handle.indexes.last().unwrap() + 1
                    < self
                        .with(&container_handle)
                        .handle(ObjectCommand::GetObjectCount())
                        .unwrap()
                        .unwrap_count()
                {
                    *current_handle.indexes.last_mut().unwrap() += 1;
                    if self.with(&current_handle).is_selectable() {
                        self.set_selected(&current_handle);
                        return;
                    } else {
                        self.selectable_movement_specific(direction, current_handle);
                    }
                } else {
                    // Look for containers to the right
                    if container_handle.indexes.is_empty() {
                        // Reached the top-level container and can't go further right
                        return;
                    }
                    current_handle = container_handle.clone();
                    *current_handle.indexes.last_mut().unwrap() += 1;
                    container_handle.indexes.pop();
                    self.selectable_movement_specific(direction, current_handle);
                }
            }
            _ => {}
        }
    }
}
