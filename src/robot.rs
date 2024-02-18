use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::ptr::copy_nonoverlapping;
use std::rc::Rc;
use crate::models::task::{Task, TaskAction};
use crate::state::AiState;
use charting_tools::charted_coordinate::ChartedCoordinate;
use charting_tools::charted_map::ChartedMap;
use charting_tools::ChartingTools;
use log::{debug, error};
use priority_queue::PriorityQueue;
use rand::Rng;
use robotics_lib::interface::{
    destroy, go, one_direction_view, put, robot_view, teleport, Direction,
};
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::tile::Content::{Bin, Fire, Garbage};
use robotics_lib::world::tile::{Tile, TileType};
use robotics_lib::world::World;

/// A fully functioning AI driven robot that cleans up garbage and extinguishes fire
/// Properties:
/// - robot: the actual `Robot`
/// - state: the state for storing useful data for a visualizer
/// - pq: priority queue that stores tasks to execute
/// - current_task: the current task that must be completed
/// - marked_coords: the coordinates that have already been analyzed for tasks
/// - charted_map: tool used to store location of teleporters
/// - previous_move_direction: direction to which the robot moved in the last process tick
/// - previous_one_directional_view_direction: direction in which the robot looked using the one directional view in the last process tick
pub struct TrashinatorRobot {
    pub robot: Robot,
    pub state: Rc<RefCell<AiState>>,
    pub(crate) pq: PriorityQueue<Task, usize>,
    pub(crate) current_task: Option<Task>,
    pub(crate) marked_coords: HashSet<ChartedCoordinate>,
    pub(crate) charted_map: ChartedMap<TileType>,
    pub(crate) previous_move_direction: Option<Direction>,
    pub(crate) previous_one_directional_view_direction: Option<Direction>,
}

impl TrashinatorRobot {
    pub fn new(robot: Robot, state: Rc<RefCell<AiState>>) -> TrashinatorRobot {
        TrashinatorRobot {
            robot,
            state,
            pq: PriorityQueue::new(),
            current_task: None,
            marked_coords: HashSet::new(),
            charted_map: ChartingTools::tool::<ChartedMap<TileType>>().unwrap(),
            previous_move_direction: None,
            previous_one_directional_view_direction: None,
        }
    }
}

impl TrashinatorRobot {
    /// Discovers new tiles and populates the pq
    pub(crate) fn discover_tiles_and_populate_pq(&mut self, world: &mut World) {
        let view = robot_view(self, world);

        for (row, row_tiles) in view.iter().enumerate() {
            for (col, col_tile) in row_tiles.iter().enumerate() {
                match col_tile {
                    None => {}
                    Some(tile) => {
                        let action_row = self.get_coordinate().get_row() + row - 1;
                        let action_col = self.get_coordinate().get_col() + col - 1;

                        self.state.borrow_mut().discovered_tiles.push((tile.clone(), (action_row, action_col)));

                        self.populate_pq(tile, (action_row, action_col));
                    }
                }
            }
        }
    }

    /// Discovers new tiles using the one directional view and populates the pq
    pub(crate) fn discover_tiles_one_direction_and_populate_pq(&mut self, world: &mut World) {
        let direction = Self::calculate_random_direction_with_weighted_previous_direction(
            &self.previous_one_directional_view_direction,
        );

        let view = one_direction_view(self, world, direction.clone(), 4);

        match view {
            Ok(view) => {
                for (x, row_tiles) in view.iter().enumerate() {
                    for (y, tile) in row_tiles.iter().enumerate() {
                        let (row, col) = match direction {
                            Direction::Up => {
                                let row = self.get_coordinate().get_row() - y - 1;
                                let col = self.get_coordinate().get_col() + x - 1;

                                (row, col)
                            }
                            Direction::Down => {
                                let row = self.get_coordinate().get_row() + y + 1;
                                let col = self.get_coordinate().get_col() + x - 1;

                                (row, col)
                            }
                            Direction::Left => {
                                let row = self.get_coordinate().get_row() + x - 1;
                                let col = self.get_coordinate().get_col() - y - 1;

                                (row, col)
                            }
                            Direction::Right => {
                                let row = self.get_coordinate().get_row() + x - 1;
                                let col = self.get_coordinate().get_col() + y + 1;

                                (row, col)
                            }
                        };

                        self.state.borrow_mut().discovered_tiles.push((tile.clone(), (row, col)));
                        self.populate_pq(tile, (row, col));
                    }
                }
            }
            Err(e) => println!("Failed to look in one direction: {:?}", e),
        };
    }

    /// Calculates the current task to execute
    pub(crate) fn determine_current_task(&mut self) {
        if self.current_task.is_none() {
            let new_task = self.pq.pop().map(|(task, _)| task);
            self.current_task = new_task;
        }

        if let Some(task) = &self.current_task {
            println!("Determined current task: {}", task);
        }
    }

    /// Executes the current task
    pub(crate) fn execute_task(&mut self, world: &mut World) {
        let current_task = &self.current_task;

        match current_task {
            None => {
                let current_coordinates = self.get_coordinate();
                let current_row = current_coordinates.get_row();
                let current_col = current_coordinates.get_col();

                // TODO: Check
                // let cloned_state = self.state.clone();
                // let state = cloned_state.borrow_mut();
                let teleports = self.charted_map.get(&TileType::Teleport(true));
                let mut target_telepor_coordinates = None;

                if let Some(teleports) = teleports {
                    if teleports
                        .iter()
                        .any(|t| t.0 .0 == current_row && t.0 .1 == current_col)
                    {
                        if let Some(target_teleport) = teleports
                            .iter()
                            .find(|t| t.0 .0 != current_row || t.0 .1 != current_col)
                        {
                            target_telepor_coordinates =
                                Some((target_teleport.0 .1, target_teleport.0 .1));
                        }
                    }
                }

                if let Some(coordinates) = target_telepor_coordinates {
                    let teleport_res = teleport(self, world, coordinates);

                    match teleport_res {
                        Ok(_) => {
                            println!(
                                "Teleported to coordinates {}, {}",
                                coordinates.0, coordinates.1
                            );
                            return;
                        }
                        Err(e) => println!("Failed to teleport: {:?}", e),
                    }
                }

                let direction = Self::calculate_random_direction_with_weighted_previous_direction(
                    &self.previous_move_direction,
                );
                let go_res = go(self, world, direction.clone());

                match go_res {
                    Ok(_) => println!("Moved {:?}", direction),
                    Err(e) => {
                        println!("Failed go to direction {:?}: {:?}", direction, e);
                    }
                };
            }
            Some(task) => match self.determine_action_to_perform_task(task) {
                Ok((execute, direction)) => {
                    println!(
                        "Determined action to perform, execute: {}, direction: {:?}",
                        execute, direction
                    );

                    if execute {
                        match task.action {
                            TaskAction::PutGarbageInBin => {
                                if let Some(garbage) =
                                    self.get_backpack().get_contents().get(&Garbage(0))
                                {
                                    if *garbage > 0 {
                                        let res = put(
                                            self,
                                            world,
                                            Garbage(0),
                                            *garbage,
                                            direction.clone(),
                                        );

                                        match res {
                                            Ok(_) => {
                                                println!("Put garbage in bin at {:?}", direction);
                                            }
                                            Err(e) => println!(
                                                "Failed putting garbage in bin at {:?}: {:?}",
                                                direction, e
                                            ),
                                        }
                                    }
                                }
                            }
                            _ => {
                                let res = destroy(self, world, direction.clone());

                                match res {
                                    Ok(_) => {
                                        println!("Destroyed {:?}", direction);
                                    }
                                    Err(e) => println!("Failed destroy at {:?}: {:?}", direction, e),
                                }
                            }
                        };

                        self.current_task = None;
                    } else {
                        let res = go(self, world, direction.clone());

                        match res {
                            Ok(_) => {
                                println!("Moved {:?}", direction);
                            }
                            Err(e) => println!("Failed go to {:?}: {:?}", direction, e),
                        }
                    };
                }
                Err(_) => println!("Failed determining task to perform"),
            },
        }
    }

    /// Calculates a direction in mix of deterministic and random logic based on the previously used `Direction`
    fn calculate_random_direction_with_weighted_previous_direction(
        previous: &Option<Direction>,
    ) -> Direction {
        let left = if *previous == Some(Direction::Right) {
            50
        } else {
            100
        };
        let right = if *previous == Some(Direction::Left) {
            50
        } else {
            100
        };
        let up = if *previous == Some(Direction::Down) {
            50
        } else {
            100
        };
        let down = if *previous == Some(Direction::Up) {
            50
        } else {
            100
        };

        let left_random = rand::thread_rng().gen_range(0..left);
        let right_random = rand::thread_rng().gen_range(0..right);
        let up_random = rand::thread_rng().gen_range(0..up);
        let down_random = rand::thread_rng().gen_range(0..down);

        let vec_of_randoms = vec![
            (left_random, Direction::Left),
            (right_random, Direction::Right),
            (up_random, Direction::Up),
            (down_random, Direction::Down),
        ];

        println!("Calculating randoms with vec: {:?}", vec_of_randoms);

        let mut max = -1;
        let mut direction = Direction::Left;

        for rand in vec_of_randoms {
            if rand.0 > max {
                max = rand.0;
                direction = rand.1;
            }
        }

        return direction;
    }

    /// Populates the pq given a tile with its coordinates
    fn populate_pq(&mut self, tile: &Tile, coordinate: (usize, usize)) {
        let charted_coordinates = &ChartedCoordinate::new(coordinate.0, coordinate.1);

        if tile.tile_type == TileType::Teleport(false) || tile.tile_type == TileType::Teleport(true) {
            self
                .charted_map
                .save(&tile.tile_type, charted_coordinates);
            println!("Saved teleport tile at coordinates {}", charted_coordinates)
        }

        let action = match tile.content {
            Garbage(_) => Some(TaskAction::DestroyGarbage),
            Fire => Some(TaskAction::DestroyFire),
            Bin(_) => self
                .get_backpack()
                .get_contents()
                .get(&Garbage(0))
                .map(|garbage| {
                    if *garbage > 5 {
                        Some(TaskAction::PutGarbageInBin)
                    } else {
                        None
                    }
                })
                .unwrap_or(None),
            _ => None,
        };

        if let Some(action) = action {
            if !self.marked_coords.contains(charted_coordinates) {
                self.marked_coords.insert(charted_coordinates.clone());

                let priority = action.get_priority_for_task();
                let task = Task::new(action, (coordinate.0, coordinate.1));

                println!("Added task to pq: {:?}", task);

                self.pq.push(task, priority);
            }
        } else {
            // println!("Didn't detect any task")
        }
    }

    /// Determines the action that the robot needs to perform in order to get closer to the
    /// completion of the task
    ///
    /// Returns a Result containing:
    /// - a `bool`: true if the bot should perform the task action, false if it should just move
    /// - a `Direction` in which the robot should perform the action or move (depending on the bool)
    fn determine_action_to_perform_task(&self, task: &Task) -> Result<(bool, Direction), ()> {
        let current_coordinates = self.get_coordinate();
        let current_row = current_coordinates.get_row();
        let current_col = current_coordinates.get_col();

        let row_diff = current_row as i32 - task.coordinates.0 as i32;
        let col_diff = task.coordinates.1 as i32 - current_col as i32;

        let mut direction_for_action = if row_diff.abs() > 1 {
            if row_diff.is_positive() {
                Some(Direction::Up)
            } else {
                Some(Direction::Down)
            }
        } else if col_diff.abs() > 1 {
            if col_diff.is_positive() {
                Some(Direction::Right)
            } else {
                Some(Direction::Left)
            }
        } else if row_diff.abs() == 1 && col_diff.abs() == 1 {
            if col_diff.is_positive() {
                Some(Direction::Right)
            } else {
                Some(Direction::Left)
            }
        } else {
            None
        };

        return if direction_for_action.is_some() {
            Ok((false, direction_for_action.unwrap()))
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
                Some(direction) => Ok((true, direction)),
                None => Err(()),
            }
        };
    }
}
