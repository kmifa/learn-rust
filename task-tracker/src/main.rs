mod add;
mod task;
pub mod util;
use add::load_tasks;
use clap::{Parser, Subcommand};
use task::{TaskStatus, Tasks};

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

#[derive(Subcommand, PartialEq)]
enum CommandTaskStatus {
    /// Show completed tasks
    Done,
    /// Show pending tasks
    Todo,
    /// Show in-progress tasks
    InProgress,
}

fn main() {
    // コマンドライン引数を解析
    let args = Cli::parse();

    // オプション（verboseフラグ）の確認
    if args.verbose {
        println!("Verbose mode enabled");
    }

    let mut a = Tasks::new(load_tasks());

    // サブコマンドの解析
    match &args.command {
        Commands::Add { task } => {
            println!("Adding task: {}", task);
            // タスクを追加する処理
            a.add(task);
        }
        Commands::Update { id, task } => {
            println!("Updating task {}: {}", id, task);
            // タスクを更新する処理
            a.update(*id as i32, task);
        }
        Commands::Delete { id } => {
            println!("Deleting task {}", id);
            // タスクを削除する処理
            a.delete(*id as i32);
        }
        Commands::MarkInProgress { id } => {
            println!("Marking task {} as in progress", id);
            // タスクを進行中にする処理
            a.mark_in_progress(*id as i32);
        }
        Commands::MarkDone { id } => {
            println!("Marking task {} as done", id);
            // タスクを完了にする処理
            a.mark_done(*id as i32);
        }
        Commands::List { status } => {
            match status {
                Some(CommandTaskStatus::Done) => {
                    println!("Listing completed tasks");
                    // 完了したタスクを表示する処理
                    a.list_status_task(TaskStatus::Done);
                }
                Some(CommandTaskStatus::Todo) => {
                    println!("Listing pending tasks");
                    // 未完了のタスクを表示する処理
                    a.list_status_task(TaskStatus::Todo);
                }
                Some(CommandTaskStatus::InProgress) => {
                    println!("Listing in-progress tasks");
                    // 進行中のタスクを表示する処理
                    a.list_status_task(TaskStatus::InProgress);
                }
                None => {
                    println!("Listing all tasks");
                    // すべてのタスクを表示する処理
                    a.list_all_tasks();
                }
            }
        }
    }
}
