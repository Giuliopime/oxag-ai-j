# Oxidizing Agents AI - j
This is a simple yet functional AI for the oxidizing agents robot.

### Tools & Libraries
- `charting tools` by `do not panic!()`
- `worldgen_unwrap` by `.unwrap().unwrap().unwrap()` (bin only)
- `priority-queue`
- `log` for better logging
- `env_logger` (bin only)
- `rand`

### Logic
These ai uses a priority queue (PQ) for storing tasks to perform.  
A task is formed by an identifier and contains coordinates for the target tile of the task.  

Each task obviously has a priority indicator since it's stored in a priority queue.  
Priorities are:
- destroy_fire: P1
- destroy_garbage: P2
- put_garbage_in_bin: P3  

The ai also stores a `current_task` in its state to know what task should be performed at each tick, expetially when energy is missing to perform all calculations and the task in the same process tick.  

On each process tick the ai performs the following actions:
1. Detect: scan for near content using either `robot_view` or `one_direction_view` (this has some randomness and some logic related to the energy level of the robot)   
    |--> found fire: insert in PQ with P1  
    |--> found garbage: insert in PQ with P2  
    |--> found bin: check whether the robot has garbage to put, if so insert in PQ with P3   
    |--> found teleport: store in charted_map tool
2. Determine `current_task`: checks whether the `current_task` is set, if not pops the queue to get the task with most priority and sets it to the `current_task`
3. Execute: navigate to coordinates and execute the task. If the current_task is missing (meaning the queue was empty) then the robot will call the `go` function to move trough the map in a random way, still avoiding to go back to where it came from most of the times (it might also use teleports when elegible).   
4. Completion: if task is completed, then set the `current_task` to None, otherwise it will be continued in the following process tick.

### Example
A fully functional project is present in the `/bin` folder.

### Real-time monitoring
Considering making a web server with HTMX for the frontend and websockets to allow viewing the performed tasks in real-time.
