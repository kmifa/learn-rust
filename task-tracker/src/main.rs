mod io;
mod task;
pub mod util;
use clap::{Parser, Subcommand};
use io::{load_tasks, save_tasks};
use task::{TaskStatus, Tasks};
use util::{println_error, println_success};

#[derive(Parser)]
#[clap(
    name = "task-cli",
    version = "1.0",
    about = "A simple task tracker CLI"
)]
struct Cli {
    /// Optional verbose flag
    #[clap(short, long, action)]
    verbose: bool,

    /// Subcommands (add, done, list, remove)
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a new task
    Add {
        /// The task description
        task: String,
    },
    /// Updating a task
    Update {
        /// The task ID
        id: u32,
        /// The new task description
        task: String,
    },
    /// Deleting a task
    Delete {
        /// The task ID
        id: u32,
    },
    /// Marking a task as to do
    MarkTodo {
        /// The task ID
        id: u32,
    },
    /// Marking a task as in progress
    MarkInProgress {
        /// The task ID
        id: u32,
    },
    /// Marking a task as done
    MarkDone {
        /// The task ID
        id: u32,
    },
    /// Lists all tasks
    List {
        /// Optional status filter
        #[clap(subcommand)]
        status: Option<CommandTaskStatus>, // サブサブコマンドをオプションで定義
    },
}

#[derive(Subcommand)]
enum CommandTaskStatus {
    /// Show completed tasks
    Done,
    /// Show pending tasks
    Todo,
    /// Show in-progress tasks
    InProgress,
}

const TASKS_FILE: &str = "tasks.json";

fn main() {
    // コマンドライン引数を解析
    let args = Cli::parse();

    // オプション（verboseフラグ）の確認
    if args.verbose {
        println!("Verbose mode enabled");
    }

    let mut a = Tasks::new(load_tasks(TASKS_FILE));
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    // サブコマンドの解析
    match &args.command {
        Commands::Add { task } => {
            let t = a.add(task, today);
            save_tasks(TASKS_FILE, t.0);
            println_success("Task added successfully!", t.1);
        }
        Commands::Update { id, task } => {
            let t = a.update(*id as i32, task, today);
            match t {
                Ok(v) => {
                    save_tasks(TASKS_FILE, v);
                    println_success("Task updated successfully!", *id as i32);
                }
                Err(e) => {
                    println_error(&e);
                }
            }
        }
        Commands::Delete { id } => {
            let t = a.delete(*id as i32);
            match t {
                Ok(v) => {
                    save_tasks(TASKS_FILE, v);
                    println_success("Task deleted successfully!", *id as i32);
                }
                Err(e) => {
                    println_error(&e);
                }
            }
        }
        Commands::MarkTodo { id } => {
            let t = a.mark_todo(*id as i32, today);
            match t {
                Ok(v) => {
                    save_tasks(TASKS_FILE, v);
                    println_success("Task successfully marked as todo!", *id as i32);
                }
                Err(e) => {
                    println_error(&e);
                }
            }
        }
        Commands::MarkInProgress { id } => {
            let t = a.mark_in_progress(*id as i32, today);
            match t {
                Ok(v) => {
                    save_tasks(TASKS_FILE, v);
                    println_success("Task successfully marked as in progress!", *id as i32);
                }
                Err(e) => {
                    println_error(&e);
                }
            }
        }
        Commands::MarkDone { id } => {
            let t = a.mark_done(*id as i32, today);
            match t {
                Ok(v) => {
                    save_tasks(TASKS_FILE, v);
                    println_success("Task successfully marked as done!", *id as i32);
                }
                Err(e) => {
                    println_error(&e);
                }
            }
        }
        Commands::List { status } => match status {
            Some(CommandTaskStatus::Done) => {
                let t = a.list_status_task(TaskStatus::Done);
                match t {
                    Ok(v) => {
                        for task in v {
                            println!("{:?}", task);
                        }
                    }
                    Err(e) => {
                        println_error(e);
                    }
                }
            }
            Some(CommandTaskStatus::Todo) => {
                let t = a.list_status_task(TaskStatus::Todo);
                match t {
                    Ok(v) => {
                        for task in v {
                            println!("{:?}", task);
                        }
                    }
                    Err(e) => {
                        println_error(e);
                    }
                }
            }
            Some(CommandTaskStatus::InProgress) => {
                let t = a.list_status_task(TaskStatus::InProgress);
                match t {
                    Ok(v) => {
                        for task in v {
                            println!("{:?}", task);
                        }
                    }
                    Err(e) => {
                        println_error(e);
                    }
                }
            }
            None => {
                a.list_all_tasks();
            }
        },
    }
}
