use std::cell::RefCell;
use std::rc::Rc;
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

    pub fn ai_process_tick(&mut self) -> (bool, Vec<Event>, Vec<(Tile, (usize, usize))>) {
        self.runner.game_tick();

        let terminated = self.state.borrow().terminate;
        let events = self.state.borrow().events_of_tick.clone();
        let tiles = self.state.borrow().discovered_tiles.clone();

        self.state.borrow_mut().discovered_tiles = vec![];
        self.state.borrow_mut().events_of_tick = vec![];

        return (terminated, events, tiles);
    }
}