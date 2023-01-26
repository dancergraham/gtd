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
    let tasks = vec![Task::new("Todo: Write a todo app!")];
    for task in tasks { if !task.complete { println!("⬛{}", task.description) } }
    ;
}
