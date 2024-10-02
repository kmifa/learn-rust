use std::{fs::File, path::Path};

use crate::task::Task;

pub fn load_tasks(file_name: &str) -> Vec<Task> {
    if Path::new(file_name).exists() {
        let file = File::open(file_name).unwrap();
        let reader = std::io::BufReader::new(file);
        let des: Result<Vec<Task>, serde_json::Error> = serde_json::from_reader(reader);

        des.unwrap_or_default()
    } else {
        Vec::new()
    }
}

pub fn save_tasks(file_name: &str, tasks: &Vec<Task>) {
    let file = File::create(file_name).unwrap();
    serde_json::to_writer_pretty(file, tasks).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::task::{TaskStatus, Tasks};
    use std::fs;

    use super::*;

    const TEST_TASKS_FILE: &str = "test_tasks.json";

    fn setup_test_file(file_name: &str, tasks: &Vec<Task>) {
        save_tasks(file_name, tasks);
    }

    fn cleanup_test_file() {
        if Path::new(TEST_TASKS_FILE).exists() {
            fs::remove_file(TEST_TASKS_FILE).unwrap();
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

        setup_test_file(TEST_TASKS_FILE, &test_content.tasks);

        let tasks = load_tasks(TEST_TASKS_FILE);
        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks[0].description, "test1");
        assert_eq!(tasks[0].status, TaskStatus::Todo);
        assert_eq!(tasks[1].description, "test2");
        assert_eq!(tasks[1].status, TaskStatus::Todo);

        cleanup_test_file();
    }

    #[test]
    fn test_load_tasks_with_non_existing_file() {
        let voidfile = "voidfile.json";
        setup_test_file(voidfile, &Vec::new());

        let tasks = load_tasks(voidfile);
        assert_eq!(tasks.len(), 0);

        if Path::new(voidfile).exists() {
            fs::remove_file(voidfile).unwrap();
        }
    }
}
