use crate::models::task::Task;
use charting_tools::charted_coordinate::ChartedCoordinate;
use charting_tools::charted_map::ChartedMap;
use charting_tools::ChartingTools;
use priority_queue::PriorityQueue;
use robotics_lib::interface::Direction;
use robotics_lib::world::tile::{Tile, TileType};
use std::collections::HashSet;
use robotics_lib::event::events::Event;

/// State required for the AI to take decisions
///
/// Properties:
/// - pq: priority queue that stores tasks to execute
/// - current_task: the current task that must be completed
/// - marked_coords: the coordinates that have already been analyzed for tasks
/// - previous_move_direction: direction to which the robot moved in the last process tick
/// - previous_one_directional_view_direction: direction in which the robot looked using the one directional view in the last process tick
pub(crate) struct AiState {
    pub events_of_tick: Vec<Event>,
    pub discovered_tiles: Vec<(Tile, (usize, usize))>,
    pub terminate: bool
}

impl AiState {
    pub(crate) fn new() -> AiState {
        AiState {
            events_of_tick: vec![],
            discovered_tiles: vec![],
            terminate: false
        }
    }
}

impl Default for AiState {
    fn default() -> Self {
        AiState::new()
    }
}
