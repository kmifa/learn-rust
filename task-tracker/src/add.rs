use std::{fs::File, path::Path};

use crate::task::Task;
use std::fs;
use std::io::Write;

const TASKS_FILE: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    // tasks.jsonを読み込む
    if Path::new(TASKS_FILE).exists() {
        let file = File::open(TASKS_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        let des: Result<Vec<Task>, serde_json::Error> = serde_json::from_reader(reader);

        des.unwrap_or_default()
    } else {
        // tasks.jsonが存在しない場合
        Vec::new()
    }
}

pub fn save_tasks(tasks: &Vec<Task>) {
    // tasks.jsonに書き込む
    let file = File::create(TASKS_FILE).unwrap();
    serde_json::to_writer_pretty(file, tasks).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::task::{TaskStatus, Tasks};

    use super::*;

    fn setup_test_file(tasks: &Vec<Task>) {
        save_tasks(tasks);
    }

    fn cleanup_test_file() {
        if Path::new(TASKS_FILE).exists() {
            fs::remove_file(TASKS_FILE).unwrap();
        }
    }

    #[test]
    fn test_load_tasks_with_existing_file() {
        let test_content = Tasks {
            tasks: vec![
                Task {
                    id: 1,
                    description: "test1".to_string(),
                    status: TaskStatus::Todo,
                    created_at: "2021-07-01".to_string(),
                    updated_at: "2021-07-01".to_string(),
                },
                Task {
                    id: 2,
                    description: "test2".to_string(),
                    status: TaskStatus::Todo,
                    created_at: "2021-07-01".to_string(),
                    updated_at: "2021-07-01".to_string(),
                },
                Task {
                    id: 3,
                    description: "test3".to_string(),
                    status: TaskStatus::Todo,
                    created_at: "2021-07-01".to_string(),
                    updated_at: "2021-07-01".to_string(),
                },
            ],
        };

        setup_test_file(&test_content.tasks);

        let tasks = load_tasks();
        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks[0].description, "test1");
        assert_eq!(tasks[0].status, TaskStatus::Todo);
        assert_eq!(tasks[1].description, "test2");
        assert_eq!(tasks[1].status, TaskStatus::Todo);

        cleanup_test_file();
    }

    #[test]
    fn test_load_tasks_with_non_existing_file() {
        cleanup_test_file();

        let tasks = load_tasks();
        assert!(tasks.is_empty());
    }
}
