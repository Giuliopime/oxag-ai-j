use oxag_ai_j::robot::TrashinatorRobot;
use robotics_lib::runner::{Robot, Runner};
use std::thread::sleep;
use std::time::Duration;
use env_logger::init_from_env;
use worldgen_unwrap::public::WorldgeneratorUnwrap;
use env_logger::Builder;

fn main() {
    // Builder::new().filter_level(LevelFilter::max()).init();
    let ai_robot = TrashinatorRobot::new(Robot::new());

    let mut gen = WorldgeneratorUnwrap::init(false, None);

    println!("Running!");

    let mut run = Runner::new(Box::new(ai_robot), &mut gen).unwrap();

    let _ = loop {
        let _ = run.game_tick();
        sleep(Duration::from_millis(1000));
        println!("-----------------");
    };
}
