#[derive(Clone, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

static mut ID_COUNTER: usize = 0;

impl Task {
    pub fn new(description: String) -> Self {
        unsafe {
            ID_COUNTER += 1;
            Task {
                id: ID_COUNTER,
                description,
                completed: false,
            }
        }
    }
}

struct TaskList {
    v: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { v: Vec::new() }
    }

    pub fn add_task(&mut self, task: Task) {
        self.v.push(task);
    }

    pub fn complete_task(&mut self, id: usize) -> Option<&mut Task> {
        for task in &mut self.v {
            if task.id == id {
                task.completed = true;
                return Some(task);
            }
        }
        None
    }

    pub fn list_tasks(&self) -> Vec<&Task> {
        self.v.iter().collect()
    }
}

struct TaskManager {
    task_list: TaskList,
}

impl TaskManager {
    pub fn new() -> Self {
        let task_list = TaskList::new();
        TaskManager { task_list }
    }

    pub fn add_task(&mut self, description: String) -> &Task {
        let task = Task::new(description);
        self.task_list.add_task(task);
        // Get a reference to the last added task
        let last_task = self.task_list.v.last().unwrap();
        last_task
    }

    pub fn complete_task(&mut self, id: usize) -> Option<&mut Task> {
        self.task_list.complete_task(id)
    }

    pub fn list_tasks(&self) -> Vec<&Task> {
        self.task_list.list_tasks()
    }
}

fn main() {
    let mut task_manager = TaskManager::new();
    let task_return = task_manager.add_task(String::from("task01"));
    println!("{:?}", task_return);
    println!("{:?}", task_manager.list_tasks());
    task_manager.complete_task(1);
    println!("{:?}", task_manager.list_tasks());
    let task_return1 = task_manager.add_task(String::from("task02"));
    println!("{:?}", task_return1);
    println!("{:?}", task_manager.list_tasks());
}
