use robotics_lib::runner::{Robot, Runner};
use std::time::Duration;
use std::thread::sleep;
use worldgen_unwrap::public::WorldgeneratorUnwrap;
use oxag_ai_j::robot::TrashinatorRobot;

fn main() {
    let ai_robot = TrashinatorRobot::new(Robot::new());

    let mut gen = WorldgeneratorUnwrap::init(false, None);

    println!("Running!");

    let run = Runner::new(Box::new(ai_robot), &mut gen);

    match run {
        Ok(mut r) => {
            let _ = loop {
                let _ = r.game_tick();
                sleep(Duration::from_millis(1000));
                println!("-----------------");
            };
        }
        Err(e) => println!("{:?}", e),
    }
}
