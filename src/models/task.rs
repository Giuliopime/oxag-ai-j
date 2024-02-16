/// Stores the action and coordinates needed to execute the task
#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) struct Task {
    pub(crate) action: TaskAction,
    /// Coordinates in the form of row, col
    pub(crate) coordinates: (usize, usize),
}

impl Task {
    pub(crate) fn new(action: TaskAction, coordinates: (usize, usize)) -> Task {
        Task {
            action,
            coordinates,
        }
    }
}

/// Represents the action of a task stored in the priority queue
#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) enum TaskAction {
    DestroyFire,
    DestroyGarbage,
    PutGarbageInBin,
}

impl TaskAction {
    pub(crate) fn get_priority_for_task(&self) -> usize {
        match self {
            TaskAction::DestroyFire => 100,
            TaskAction::DestroyGarbage => 50,
            TaskAction::PutGarbageInBin => 1,
        }
    }
}
