use serde::{Deserialize, Serialize};

use crate::add::save_tasks;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl Task {
    pub fn new(
        id: i32,
        description: String,
        status: TaskStatus,
        created_at: String,
        updated_at: String,
    ) -> Task {
        Task {
            id,
            description,
            status,
            created_at,
            updated_at,
        }
    }
}

impl Tasks {
    pub fn new(tasks: Vec<Task>) -> Tasks {
        Tasks { tasks }
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn add(&mut self, task_name: &String) {
        let id = self.len() as i32 + 1;
        let new_task = Task::new(
            id,
            task_name.to_string(),
            TaskStatus::Todo,
            "2021-07-01".to_string(),
            "2021-07-01".to_string(),
        );
        self.tasks.push(new_task);
        save_tasks(&self.tasks);
        println!("Task added successfully (ID: {})", id);
    }

    pub fn update(&mut self, id: i32, task_name: &String) {
        let task = self.tasks.iter_mut().find(|t| t.id == id);
        match task {
            Some(t) => {
                t.description = task_name.to_string();
                t.updated_at = "2021-07-01".to_string();
                save_tasks(&self.tasks);
                println!("Task updated successfully (ID: {})", id);
            }
            None => {
                println!("Task not found (ID: {})", id);
            }
        }
    }

    pub fn delete(&mut self, id: &i32) {
        let task = self.tasks.iter().position(|t| &t.id == id);
        match task {
            Some(i) => {
                self.tasks.remove(i);
                save_tasks(&self.tasks);
                println!("Task deleted successfully (ID: {})", id);
            }
            None => {
                println!("Task not found (ID: {})", id);
            }
        }
    }

    pub fn mark_in_progress(&mut self, id: &i32) {
        let task = self.tasks.iter_mut().find(|t| &t.id == id);
        match task {
            Some(t) => {
                t.status = TaskStatus::InProgress;
                t.updated_at = "2021-07-01".to_string();
                save_tasks(&self.tasks);
                println!("Task marked as in progress (ID: {})", id);
            }
            None => {
                println!("Task not found (ID: {})", id);
            }
        }
    }

    pub fn mark_done(&mut self, id: &i32) {
        let task = self.tasks.iter_mut().find(|t| &t.id == id);
        match task {
            Some(t) => {
                t.status = TaskStatus::Done;
                t.updated_at = "2021-07-01".to_string();
                save_tasks(&self.tasks);
                println!("Task marked as done (ID: {})", id);
            }
            None => {
                println!("Task not found (ID: {})", id);
            }
        }
    }

    pub fn list_status_task(&self, status: TaskStatus) {
        match status {
            TaskStatus::Done => {
                let done_tasks: Vec<&Task> = self
                    .tasks
                    .iter()
                    .filter(|t| t.status == TaskStatus::Done)
                    .collect();
                if done_tasks.is_empty() {
                    println!("No completed tasks found");
                } else {
                    for task in done_tasks {
                        println!("{:?}", task);
                    }
                }
            }
            TaskStatus::Todo => {
                let todo_tasks: Vec<&Task> = self
                    .tasks
                    .iter()
                    .filter(|t| t.status == TaskStatus::Todo)
                    .collect();
                if todo_tasks.is_empty() {
                    println!("No pending tasks found");
                } else {
                    for task in todo_tasks {
                        println!("{:?}", task);
                    }
                }
            }
            TaskStatus::InProgress => {
                let in_progress_tasks: Vec<&Task> = self
                    .tasks
                    .iter()
                    .filter(|t| t.status == TaskStatus::InProgress)
                    .collect();
                if in_progress_tasks.is_empty() {
                    println!("No in-progress tasks found");
                } else {
                    for task in in_progress_tasks {
                        println!("{:?}", task);
                    }
                }
            }
        }
    }

    pub fn list_all_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found");
        } else {
            for task in &self.tasks {
                println!("{:?}", task);
            }
        }
    }
}
