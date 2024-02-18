use oxag_ai_j::robot::TrashinatorRobot;
use robotics_lib::runner::{Robot, Runner};
use std::thread::sleep;
use std::time::Duration;
use env_logger::init_from_env;
use worldgen_unwrap::public::WorldgeneratorUnwrap;
use env_logger::Builder;
use oxag_ai_j::WrapperTrashinatorRobot;

fn main() {
    // Builder::new().filter_level(LevelFilter::max()).init();
    let mut ai_robot = WrapperTrashinatorRobot::new();

    let _ = loop {
        let _ = ai_robot.ai_process_tick();
        sleep(Duration::from_millis(1000));
        println!("-----------------");
    };
}
