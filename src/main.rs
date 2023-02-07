#![warn(missing_docs)]
//! Todo app
//!
//!  Simple todo opp to help me learn Rust 🦀

use std::error::Error;
use std::fs::create_dir_all;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

/// Represents a task to be done
#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    complete: bool,
}


impl Task {
    /// Constructor. Tasks are incomplete on initialisation
    fn new(description: &str) -> Self {
        Self {
            description: String::from(description),
            complete: false,
        }
    }
}

/// Read tasks from the csv file database
fn read_db() -> Result<Vec<Task>, Box<dyn Error>> {
    use csv::Reader;
    let data_path = Path::new("./data/tasks.csv");
    let mut data = vec![];
    let mut reader = Reader::from_path(data_path)?;
    for result in reader.deserialize() {
        data.push(result?)
    }
    Ok(data)
}

/// Write all tasks back to the csv file database
///
/// Creates the db file if necessary
fn write_db(data: &[Task]) -> Result<(), Box<dyn Error>> {
    use csv::Writer;
    let data_dir = Path::new("./data");
    let data_path = Path::new("./data/tasks.csv");
    create_dir_all(data_dir)?;
    let mut wtr = Writer::from_path(data_path)?;
    for datum in data {
        wtr.serialize(datum)?;
    }
    wtr.flush()?;
    Ok(())
}

fn main() {
    use std::io;
    let tasks = read_db();
    match tasks {
        Ok(mut tasks) => loop {
            display_tasks(&tasks);
            println!("\nadd task, finish or (save and) exit");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_n) => {
                    input = String::from(input.trim_end());
                    if input == "exit" {
                        save_db(&tasks);
                        break;
                    } else if input == "finish" {
                        input.clear();
                        match io::stdin().read_line(&mut input) {
                            Ok(_n) => {
                                input = String::from(input.trim());
                                let i = input.parse::<usize>().expect("give me an integer");
                                if let Some(t) = tasks.get_mut(i) {
                                    t.complete = true
                                }
                            }
                            Err(e) => println!("{e}"),
                        }
                    } else {
                        tasks.push(Task::new(&input));
                        save_db(&tasks)
                    };
                }
                Err(e) => println!("{e}"),
            }
        },
        Err(e) => println!("Error reading db {e}"),
    }
}

/// Print any incomplete tasks to the console
fn display_tasks(tasks: &[Task]) {
    print!("{}[2J", 27 as char);
    for (i, task) in tasks.iter().enumerate() {
        if !task.complete {
            println!("{}⬛{}", i, task.description)
        }
    }
}

/// Attempt to save the database and report on success
fn save_db(tasks: &[Task]) {
    let saved = write_db(tasks);
    match saved {
        Ok(_) => println!("Tasks saved"),
        Err(e) => println!("Error saving tasks {e}"),
    }
}
