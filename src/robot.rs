use charting_tools::charted_coordinate::ChartedCoordinate;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{Direction, go, put, robot_view, teleport};
use crate::state::AiState;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Content::{Bin, Fire, Garbage};
use robotics_lib::world::tile::TileType;
use robotics_lib::world::World;
use crate::models::task::{Task, TaskAction};

/// A fully functioning AI driven robot that cleans up garbage and extinguishes fire
pub struct TrashinatorRobot {
    pub robot: Robot,
    pub(crate) state: AiState,
}

impl TrashinatorRobot {
    pub fn new(robot: Robot) -> TrashinatorRobot {
        TrashinatorRobot {
            robot,
            state: AiState::new()
        }
    }
}

impl Default for TrashinatorRobot {
    fn default() -> Self {
        TrashinatorRobot::new(Robot::new())
    }
}

impl TrashinatorRobot {
    /// Discovers new tiles and populates the priority queue based on the discovered tiles
    fn discover_tiles_and_populate_pq(&mut self, world: &mut World) {
        /// TODO: Use one directional view if energy is a lot
        let view = robot_view(self, world);

        for (row, row_tiles) in view.iter().enumerate() {
            for (col, col_tile) in row_tiles.iter().enumerate() {
                match col_tile {
                    None => {},
                    Some(tile) => {
                        let action_row = self.get_coordinate().get_row() + row - 1;
                        let action_col = self.get_coordinate().get_col() + col - 1;
                        let charted_coordinate = ChartedCoordinate::new(action_row, action_col);

                        if tile.tile_type == TileType::Teleport(false) || tile.tile_type == TileType::Teleport(true) {
                            &self.state.charted_map.save(&tile.tile_type, &charted_coordinate)
                        }

                        let action = match tile.content {
                            Garbage(_) => Some(TaskAction::DestroyGarbage),
                            Fire => Some(TaskAction::DestroyFire),
                            Bin(_) => {
                                self.get_backpack().get_contents().get(&Garbage(0)).map(|garbage| {
                                    if garbage.to_owned() > 5 {
                                        Some(TaskAction::PutGarbageInBin)
                                    } else {
                                        None
                                    }
                                });
                            }
                            _ => None,
                        };

                        if let Some(action) = action {
                            if !self.state.marked_coords.contains(&charted_coordinate) {
                                self.state.marked_coords.insert(charted_coordinate);

                                self.state.pq.push(
                                    Task::new(action, (action_row, action_col)),
                                    action.get_priority_for_task(),
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    fn determine_current_task(&mut self) {
        if self.state.current_task.is_none() {
            self.state.current_task = self.state.pq.pop().map(|(task, _)| task);
        }
    }

    /// Executes the current task
    fn execute_task(&mut self, world: &mut World) {
        match &self.state.current_task {
            None => {
                let current_coordinates = self.get_coordinate();
                let current_row = current_coordinates.get_row();
                let current_col = current_coordinates.get_col();

                let teleports = &self.state.charted_map.get(&TileType::Teleport(true));

                if let Some(teleports) = teleports {
                    if teleports.iter().any(|t| t.0.0 == current_row && t.0.1 == current_col) {
                        if let Some(target_teleport) = teleports.iter().find(|t| t.0.0 != current_row || t.0.1 != current_col) {
                            let teleport_res = teleport(self, world, (target_teleport.0.0, target_teleport.0.1));

                            match teleport_res {
                                Ok(_) => return,
                                Err(e) => println!("Failed to teleport: {:?}", e)
                            }
                        }
                    }
                }

                GetToTile()
                let _ = go(self, world, Direction::Right);
            }
            Some(task) => {
                match self.determine_action_to_perform_task(task) {
                    Ok((execute, direction)) => {

                    },
                    Err(e) => println!("{}", e)
                }
            }
        }
    }

    /// Determines the action that the robot needs to perform in order to get closer to the
    /// completion of the task
    ///
    /// Returns a Result containing:
    /// - a `bool`: true if the bot should perform the task action, false if it should just move
    /// - a `Direction` in which the robot should perform the action or move (depending on the bool)
    fn determine_action_to_perform_task(&mut self, task: &Task) -> Result<(bool, Direction), Err> {
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
                Some(direction) => {
                    Ok((true, direction))
                }
                None => Err("Couldn't determine action and direction!"),
            }
        }
    }
}

impl Runnable for TrashinatorRobot {
    fn process_tick(&mut self, world: &mut World) {
        self.discover_tiles_and_populate_pq(world);
        self.determine_current_task();
        self.execute_task(world);
    }

    fn handle_event(&mut self, _: Event) {

    }
    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }
    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}
