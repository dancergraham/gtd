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
    use std::io;
    let mut tasks = vec![];
    tasks.push(Task::new("Todo: Write a todo app!"));
    tasks.push(Task::new("Write tests"));
    tasks.push(Task::new("Accept user input"));
    tasks.push(Task::new("Allow tasks to be marked complete"));
    tasks.push(Task::new("Save DB"));
    tasks.push(Task::new("Add Gui"));
    loop {
        for (i, task) in tasks.iter().enumerate() { if !task.complete { println!("{}⬛{}", i, task.description) } }
        println!("\nadd, finish or exit");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                input = String::from(input.trim_end());
                if input == "exit" { break; } else if input == "finish" {
                    input.clear();
                    match io::stdin().read_line(&mut input) {
                        Ok(_n) => {
                            input = String::from(input.trim());
                            let i = input.parse::<usize>().expect("give me an integer");
                            if let Some(t) = tasks.get_mut(i) {
                                t.complete = true
                            }
                        }
                        Err(e) => println!("{e}")
                    }
                } else {
                    tasks.push(Task::new(&input))
                }
                ;
            }
            Err(e) => println!("{e}")
        }
    }
}
