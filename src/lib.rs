use std::cell::RefCell;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use robotics_lib::event::events::Event;
use robotics_lib::runner::{Robot, Runner};
use robotics_lib::world::tile::Tile;
use robotics_lib::world::world_generator::{Generator, World};
use worldgen_unwrap::public::WorldgeneratorUnwrap;
use crate::robot::TrashinatorRobot;
use crate::state::AiState;

mod models;
pub mod robot;
mod state;
mod runnable;

/// A wrapper for a fully functioning AI driven robot that cleans up garbage and extinguishes fire
///
/// This also exposes useful data for visualizers
pub struct WrapperTrashinatorRobot {
    runner: Runner,
    state: Rc<RefCell<AiState>>,
    world_generator: WorldgeneratorUnwrap
}

impl WrapperTrashinatorRobot {
    pub fn new() -> WrapperTrashinatorRobot {
        let mut world_generator = WorldgeneratorUnwrap::init(false, None);

        let state = Rc::new(RefCell::new(AiState::new()));
        let runner = TrashinatorRobot::new(Robot::new(), state.clone());
        let runner = Runner::new(Box::new(runner), &mut world_generator).unwrap();

        WrapperTrashinatorRobot {
            runner,
            state,
            world_generator
        }
    }

    /// Performs a process tick
    ///
    /// Returns a tuple containing:
    /// - a bool that indicates whether the ai robot has terminated
    /// - a `Vec` of all `Event`s occurred in the process tick
    /// - a `Vec` of `(Tile, (usize, usize))` with all the discovered tiles and relative coordinates for the process tick
    pub fn ai_process_tick(&mut self) -> (bool, Vec<Event>, Vec<(Tile, (usize, usize))>) {
        // Reset the state to prepare for the process tick
        self.state.borrow_mut().discovered_tiles = vec![];
        self.state.borrow_mut().events_of_tick = vec![];

        // Execute the process tick
        self.runner.game_tick();

        sleep(Duration::from_millis(500));

        // Return data usable by the visualizer
        let terminated = self.state.borrow().terminate;
        let events = self.state.borrow().events_of_tick.clone();
        let tiles = self.state.borrow().discovered_tiles.clone();

        return (terminated, events, tiles);
    }
}