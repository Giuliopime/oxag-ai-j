use crate::models::task::Task;
use charting_tools::charted_coordinate::ChartedCoordinate;
use charting_tools::charted_map::ChartedMap;
use charting_tools::ChartingTools;
use priority_queue::PriorityQueue;
use robotics_lib::interface::Direction;
use robotics_lib::world::tile::{Tile, TileType};
use std::collections::HashSet;
use robotics_lib::event::events::Event;

/// State that should be consumed by a visualizer
///
/// Properties:
/// - events_of_tick: the events occurred in a process tick
/// - discovered_tiles: all discovered tiles during the process tick
/// - terminate: whether the robot has completed its goal
pub struct AiState {
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
