use std::{thread::sleep, collections::HashSet};
use std::time::Duration;
use oxag_ai_j::{AiBot, AiState};
use priority_queue::PriorityQueue;
use robotics_lib::runner::{Runner, Robot};
use worldgen_unwrap::public::WorldgeneratorUnwrap;


fn main() {
    let ai_robot = AiBot {
        robot: Robot::new(),
        state: AiState {
            pq: PriorityQueue::new(),
            current_task: None,
            marked_coords: HashSet::new(),
        }
    };

    let mut gen = WorldgeneratorUnwrap::init(false, None);

    println!("Running!");

    let run = Runner::new(Box::new(ai_robot), &mut gen);

    match run {
        | Ok(mut r) => {
            let _ = loop {
                let _ = r.game_tick();
                sleep(Duration::from_millis(1000));
                println!("-----------------");
            };
        }
        | Err(e) => println!("{:?}", e),
    }
}
