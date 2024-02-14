# Oxidizing Agents AI - j
This is a simple yet functional AI for the oxidizing agents robot

### Logic
These ai uses a priority queue (PQ) for storing tasks to perform.
A task is formed by an identifier and can contain coordinates for the target tile of the task.

Each task obviously has a priority indicator since it's stored in a priority queue.
Priorities are:
- destroy_fire: P1
- destroy_garbage: P2

The ai also stores a `current_task` in its state to know what task should be performed at each tick, expetially when energy is missing to perform all calculations and the task in the same process tick.

On each process tick the ai performs the following actions:
1. Detect: scan for near content using either `robot_view` or `one_direction_view`
    |--> found fire: insert in PQ with P1
    |--> found garbage: insert in PQ with P2
    |--> found bin: set current task to drop_garbage
2. Determine `current_task`: checks whether the `current_task` is set, if not pops the queue to get the task with most priority and sets it to the `current_task`
3. Execute: navigate to coordinates and execute the task
4. Completion: if task is completed, then set the `current_task` to None, otherwise it will be continued in the following process tick.

### Real-time monitoring
Considering making a web server with HTMX for the frontend and websockets to allow viewing the performed tasks in real-time
