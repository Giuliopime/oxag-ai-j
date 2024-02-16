use std::collections::HashSet;

use charting_tools::charted_coordinate::{self, ChartedCoordinate};
use priority_queue::PriorityQueue;
use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{destroy, go, put, robot_view, Direction},
    runner::{backpack::BackPack, Robot, Runnable},
    world::{
        coordinates::Coordinate,
        tile::Content::{self, Bin, Fire, Garbage},
        World,
    },
};

/// Represents the action of a task stored in the priority queue
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum TaskAction {
    DestroyFire,
    DestroyGarbage,
    PutGarbage,
}

/// All infos needed to perform a task
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Task {
    action: TaskAction,
    coordinates: (usize, usize),
}

fn get_priority_for_task(action: &TaskAction) -> usize {
    match action {
        TaskAction::DestroyFire => 100,
        TaskAction::DestroyGarbage => 50,
        TaskAction::PutGarbage => 1,
    }
}

pub struct AiState {
    pub pq: PriorityQueue<Task, usize>,
    pub current_task: Option<Task>,
    pub marked_coords: HashSet<ChartedCoordinate>,
}

pub struct AiBot {
    pub robot: Robot,
    pub state: AiState,
}

impl Runnable for AiBot {
    fn process_tick(&mut self, world: &mut World) {
        // DETECT NEAR TILES AND POPULATE PRIORITY QUEUE
        let view = robot_view(self, world);
        for (row_coord, row) in view.iter().enumerate() {
            for (col_coord, col) in row.iter().enumerate() {
                match col {
                    None => print!("found an unknown tile"),
                    Some(tile) => {
                        let action = match tile.content {
                            Garbage(_) => Some(TaskAction::DestroyGarbage),
                            Fire => Some(TaskAction::DestroyFire),
                            Bin(_) => {
                                if let Some(garbage) =
                                    self.get_backpack().get_contents().get(&Content::Garbage(0))
                                {
                                    if garbage.to_owned() > 5 {
                                        Some(TaskAction::PutGarbage)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        };

                        if action.is_some() {
                            let final_row = self.get_coordinate().get_row() + row_coord - 1;
                            let final_col = self.get_coordinate().get_col() + col_coord - 1;
                            let charted_coordinate = ChartedCoordinate::new(final_row, final_col);

                            if self.state.marked_coords.contains(&charted_coordinate) {
                                // Skip
                            } else {
                                self.state.marked_coords.insert(charted_coordinate);

                                let priority = get_priority_for_task(action.as_ref().unwrap());

                                println!("Adding {:?} with priority {priority} to queue!", action);

                                self.state.pq.push(
                                    Task {
                                        action: action.unwrap(),
                                        coordinates: (final_row, final_col),
                                    },
                                    priority,
                                );
                            }
                        }
                    }
                }
            }
        }

        // DETERMINE CURRENT TASK

        if self.state.current_task.is_none() {
            self.state.current_task = self.state.pq.pop().map(|(task, _)| task);
        }

        println!("Current task: {:?}", self.state.current_task);
        println!("Current coords: {:?}", self.get_coordinate());

        // NAVIGATE TO COORDS AND EXECUTE TASK
        match &self.state.current_task {
            None => {
                // move around
                let _ = go(self, world, Direction::Right);
            }
            Some(task) => {
                let current_coordinates = self.get_coordinate();
                let current_row = current_coordinates.get_row();
                let current_col = current_coordinates.get_col();

                let row_diff = current_row as i32 - task.coordinates.0 as i32;
                let col_diff = task.coordinates.1 as i32 - current_col as i32;

                let mut direction_for_action = None;

                if row_diff.abs() > 1 {
                    if row_diff.is_positive() {
                        direction_for_action = Some(Direction::Up)
                    } else {
                        direction_for_action = Some(Direction::Down)
                    }
                } else if col_diff.abs() > 1 {
                    if col_diff.is_positive() {
                        direction_for_action = Some(Direction::Right)
                    } else {
                        direction_for_action = Some(Direction::Left)
                    }
                } else if row_diff.abs() == 1 && col_diff.abs() == 1 {
                    if col_diff.is_positive() {
                        direction_for_action = Some(Direction::Right)
                    } else {
                        direction_for_action = Some(Direction::Left)
                    }
                }

                if direction_for_action.is_some() {
                    // Simply move towards task coordinates
                    // TODO: Handle result
                    println!(
                        "Need to move, task not close enough, dir: {:?}",
                        direction_for_action
                    );
                    let _ = go(self, world, direction_for_action.unwrap());
                } else {
                    direction_for_action = if row_diff.abs() == 1 {
                        if row_diff.is_positive() {
                            Some(Direction::Up)
                        } else {
                            Some(Direction::Down)
                        }
                    } else if col_diff.abs() == 1 {
                        if col_diff.is_positive() {
                            Some(Direction::Right)
                        } else {
                            Some(Direction::Left)
                        }
                    } else {
                        None
                    };

                    match direction_for_action {
                        Some(direction) => {
                            match task.action {
                                TaskAction::PutGarbage => {
                                    // TODO: Check if we have garbage and put in bin
                                    if let Some(garbage) =
                                        self.get_backpack().get_contents().get(&Content::Garbage(0))
                                    {
                                        if garbage.to_owned() > 0 {
                                            // TODO: Handle res
                                            put(
                                                self,
                                                world,
                                                Content::Garbage(0),
                                                garbage.to_owned(),
                                                direction,
                                            );

                                            println!("put garbage in bin");
                                        }
                                    }
                                }
                                _ => {
                                    // TODO: Check result
                                    let res = destroy(self, world, direction.clone());

                                    match res {
                                        Ok(_) => {}
                                        Err(e) => println!("{:?}", e),
                                    }
                                    println!("Destroyed stuff at direction {:?}", direction);
                                }
                            }

                            self.state.current_task = None;
                        }
                        None => print!("Could not determine direction for performing tsak!"),
                    }
                }
            }
        }
    }

    fn handle_event(&mut self, event: Event) {
        // println!("{:?}", event);
    }
    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }
    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }
}
