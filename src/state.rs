use crate::models::task::Task;
use charting_tools::charted_coordinate::ChartedCoordinate;
use priority_queue::PriorityQueue;
use std::collections::HashSet;
use charting_tools::charted_map::ChartedMap;
use robotics_lib::world::tile::{Content, TileType};
use robotics_lib::world::tile::TileType::Teleport;

/// State required for the AI to take decisions
///
/// Properties:
/// - pq: priority queue that stores tasks to execute
/// - current_task: the current task that must be completed
/// - the coordinates that have already been analyzed for tasks
pub(crate) struct AiState {
    pub(crate) pq: PriorityQueue<Task, usize>,
    pub(crate) current_task: Option<Task>,
    pub(crate) marked_coords: HashSet<ChartedCoordinate>,
    pub(crate) charted_map: ChartedMap<TileType>
}

impl AiState {
    pub(crate) fn new() -> AiState {
        AiState {
            pq: PriorityQueue::new(),
            current_task: None,
            marked_coords: HashSet::new(),
            charted_map: ChartedMap::new()
        }
    }
}

impl Default for AiState {
    fn default() -> Self {
        AiState::new()
    }
}
