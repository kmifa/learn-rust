mod expence;
pub mod util;
use expence::ExpencesList;
use io::{load_expence_list, load_limit_list};
mod io;
mod limit;

use chrono::prelude::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "expence-tracker-cli",
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
        #[clap(
            short,
            long,
            help = "A brief description of the expense (e.g., 'Lunch')"
        )]
        description: String,
        /// The task amount
        #[clap(short, long, help = "The amount spent (e.g., 20.50)")]
        amount: i32,
        #[clap(short, long, help = "The date of the expense (e.g., 2021-08-01)")]
        category: String,
    },
    /// Deleting a task
    Delete {
        /// The task ID
        #[clap(short, long, help = "Delete an expense by ID (e.g., --delete 1)")]
        id: i32,
    },
    /// List all Expences
    List {},
    /// All Expences Summary
    Summary {
        /// Optional month filter (e.g., --month 8)
        #[clap(short, long, help = "Specify the month to filter expenses (1-12)")]
        month: Option<i32>,
    },
    Update {
        /// The task ID
        #[clap(short, long, help = "Update an expense by ID (e.g., --update 1)")]
        id: i32,
        /// The task description
        #[clap(
            short,
            long,
            help = "A brief description of the expense (e.g., 'Lunch')"
        )]
        description: String,
        /// The task amount
        #[clap(short, long, help = "The amount spent (e.g., 20.50)")]
        amount: i32,
    },
    AddLimit {
        /// The limit amount
        #[clap(short, long, help = "The limit amount (e.g., 100)")]
        limit: i32,
        /// The year-month
        #[clap(short, long, help = "The year-month (e.g., 2021-08)")]
        ym: String,
    },
    UpdateLimit {
        /// The limit amount
        #[clap(short, long, help = "The limit amount (e.g., 100)")]
        limit: i32,
        /// The year-month
        #[clap(short, long, help = "The year-month (e.g., 2021-08)")]
        ym: String,
    },
    DeleteLimit {
        /// The limit index
        #[clap(short, long, help = "Delete a limit by year-month (e.g., 2021-08)")]
        ym: String,
    },
    Download {},
}

fn main() {
    // コマンドライン引数を解析
    let args = Cli::parse();

    // オプション（verboseフラグ）の確認
    if args.verbose {
        println!("Verbose mode enabled");
    }

    // タスクリストの初期化
    let mut a = ExpencesList::new(load_expence_list());
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    // サブコマンドの解析
    match &args.command {
        Commands::Add {
            description,
            amount,
            category,
        } => {
            // タスクを追加する処理
            a.add(
                today,
                description.to_string(),
                *amount,
                category.to_string(),
            );
        }
        Commands::Delete { id } => {
            // タスクを削除する処理
            a.delete(*id);
        }
        Commands::List {} => {
            a.list();
        }
        Commands::Summary { month } => {
            a.summary(month);
        }
        Commands::Update {
            id,
            description,
            amount,
        } => {
            a.update(*id, description.to_string(), *amount);
        }
        Commands::AddLimit { limit, ym } => {
            let mut l = limit::LimitList::new(load_limit_list());
            l.add(*limit, ym.to_string());
        }
        Commands::UpdateLimit { limit, ym } => {
            let mut l = limit::LimitList::new(load_limit_list());
            l.update(*limit, ym.to_string());
        }
        Commands::DeleteLimit { ym } => {
            let mut l = limit::LimitList::new(load_limit_list());
            l.delete(ym.to_string());
        }
        Commands::Download {} => {
            io::download_csv();
        }
    }
}
