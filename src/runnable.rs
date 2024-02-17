use log::info;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::Runnable;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use crate::robot::TrashinatorRobot;

impl Runnable for TrashinatorRobot {
    fn process_tick(&mut self, world: &mut World) {
        let mut state = *self.state.borrow_mut();
        let energy = self.get_energy().get_energy_level();

        if energy > 50 && energy % 2 == 0 {
            self.discover_tiles_one_direction_and_populate_pq(world, state);
        } else {
            self.discover_tiles_and_populate_pq(world, state);
        }

        self.determine_current_task(state);
        self.execute_task(world, state);
    }

    fn handle_event(&mut self, event: Event) {
        info!("Event - {}", event);
        self.state.borrow_mut().events_of_tick.push(event);
    }
    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }
    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}
