use oxag_ai_j::WrapperTrashinatorRobot;
use env_logger::Env;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .format_timestamp(None)
        .format_target(false)
        .init();

    let mut ai_robot = WrapperTrashinatorRobot::new(10);

    let mut done = false;

    while !done {
        println!("-----------------");
        let res = ai_robot.ai_process_tick();
        done = res.0;
    };
}
