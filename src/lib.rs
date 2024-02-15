use std::usize;

use priority_queue::PriorityQueue;
use robotics_lib::{
    interface::{destroy, go, put, robot_view, Direction},
    runner::Runnable,
    world::{
        tile::Content::{self, Bin, Fire, Garbage},
        World,
    },
};

/// Represents the action of a task stored in the priority queue
#[derive(PartialEq, Eq, Hash)]
enum TaskAction {
    DestroyFire,
    DestroyGarbage,
    PutGarbage,
}

/// All infos needed to perform a task
#[derive(PartialEq, Eq, Hash)]
struct Task {
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

struct AiState {
    pq: PriorityQueue<Task, usize>,
    current_task: Option<Task>,
}

fn process_ai_tick(runnable: &mut impl Runnable, world: &mut World, state: &mut AiState) {
    // DETECT NEAR TILES AND POPULATE PRIORITY QUEUE
    let view = robot_view(runnable, world);
    for (row_coord, row) in view.iter().enumerate() {
        for (col_coord, col) in row.iter().enumerate() {
            match col {
                None => print!("found an unknown tile"),
                Some(tile) => {
                    let action = match tile.content {
                        Garbage(_) => Some(TaskAction::DestroyGarbage),
                        Fire => Some(TaskAction::DestroyGarbage),
                        Bin(_) => Some(TaskAction::PutGarbage),
                        _ => None,
                    };

                    if action.is_some() {
                        let priority = get_priority_for_task(action.as_ref().unwrap());
                        state.pq.push(
                            Task {
                                action: action.unwrap(),
                                coordinates: (row_coord, col_coord),
                            },
                            priority,
                        );
                    }
                }
            }
        }
    }

    // DETERMINE CURRENT TASK

    if state.current_task.is_none() {
        state.current_task = state.pq.pop().map(|(task, _)| task);
    }

    // NAVIGATE TO COORDS AND EXECUTE TASK
    match &state.current_task {
        None => {
            // move around
        }
        Some(task) => {
            let current_coordinates = runnable.get_coordinate();
            let current_row = current_coordinates.get_row();
            let current_col = current_coordinates.get_col();

            let row_diff = task.coordinates.0 as i32 - current_row as i32;
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
                go(runnable, world, direction_for_action.unwrap());
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
                                if let Some(garbage) = runnable
                                    .get_backpack()
                                    .get_contents()
                                    .get(&Content::Garbage(0))
                                {
                                    if garbage.to_owned() > 0 {
                                        // TODO: Handle res
                                        put(
                                            runnable,
                                            world,
                                            Content::Garbage(0),
                                            garbage.to_owned(),
                                            direction,
                                        );
                                    }
                                }
                            }
                            _ => {
                                // TODO: Check result
                                destroy(runnable, world, direction);
                            }
                        }
                    }
                    None => print!("Could not determine direction!"),
                }
            }
        }
    }
}
