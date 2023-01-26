struct Task {
    description: String,
    complete: bool,
}

impl Task {
    fn new(description: &str) -> Self {
        Self {
            description: String::from(description),
            complete: false,
        }
    }
}

fn main() {
    let tasks = vec![Task::new("Todo: Write a todo app!"),
                     Task::new("Accept user input"),
                     Task::new("Allow tasks to be marked complete"),
                     Task::new("Save DB"),
                     Task::new("Add Gui"),
    ];
    for task in tasks { if !task.complete { println!("⬛{}", task.description) } }
    ;
}
