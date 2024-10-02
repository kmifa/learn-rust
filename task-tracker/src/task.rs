use serde::{Deserialize, Serialize};

use crate::util::println_error;

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

    pub fn add(&mut self, task_name: &String, created_at: String) -> (&Vec<Task>, i32) {
        let id = self.len() as i32 + 1;
        let new_task = Task::new(
            id,
            task_name.to_string(),
            TaskStatus::Todo,
            created_at.clone(),
            created_at.clone(),
        );
        self.tasks.push(new_task);
        (&self.tasks, id)
    }

    pub fn update(
        &mut self,
        id: i32,
        task_name: &String,
        updated_at: String,
    ) -> Result<&Vec<Task>, String> {
        let task = self.tasks.iter_mut().find(|t| t.id == id);
        match task {
            Some(t) => {
                t.description = task_name.to_string();
                t.updated_at = updated_at;
                Ok(&self.tasks)
            }
            None => Err(format!("Task with ID {} not found", id)),
        }
    }

    pub fn delete(&mut self, id: i32) -> Result<&Vec<Task>, String> {
        let task = self.tasks.iter().position(|t| t.id == id);
        match task {
            Some(i) => {
                self.tasks.remove(i);
                Ok(&self.tasks)
            }
            None => Err(format!("Task with ID {} not found", id)),
        }
    }

    pub fn mark_todo(&mut self, id: i32, updated_at: String) -> Result<&Vec<Task>, String> {
        let task = self.tasks.iter_mut().find(|t| t.id == id);
        match task {
            Some(t) => {
                t.status = TaskStatus::Todo;
                t.updated_at = updated_at;
                Ok(&self.tasks)
            }
            None => Err(format!("Task with ID {} not found", id)),
        }
    }

    pub fn mark_in_progress(&mut self, id: i32, updated_at: String) -> Result<&Vec<Task>, String> {
        let task = self.tasks.iter_mut().find(|t| t.id == id);
        match task {
            Some(t) => {
                t.status = TaskStatus::InProgress;
                t.updated_at = updated_at;
                Ok(&self.tasks)
            }
            None => Err(format!("Task with ID {} not found", id)),
        }
    }

    pub fn mark_done(&mut self, id: i32, updated_at: String) -> Result<&Vec<Task>, String> {
        let task = self.tasks.iter_mut().find(|t| t.id == id);
        match task {
            Some(t) => {
                t.status = TaskStatus::Done;
                t.updated_at = updated_at;
                Ok(&self.tasks)
            }
            None => Err(format!("Task with ID {} not found", id)),
        }
    }

    pub fn list_status_task(&self, status: TaskStatus) -> Result<Vec<&Task>, &str> {
        match status {
            TaskStatus::Done => {
                let done_tasks: Vec<&Task> = self
                    .tasks
                    .iter()
                    .filter(|t| t.status == TaskStatus::Done)
                    .collect();
                if done_tasks.is_empty() {
                    Err("No completed tasks found")
                } else {
                    Ok(done_tasks)
                }
            }
            TaskStatus::Todo => {
                let todo_tasks: Vec<&Task> = self
                    .tasks
                    .iter()
                    .filter(|t| t.status == TaskStatus::Todo)
                    .collect();
                if todo_tasks.is_empty() {
                    Err("No todo tasks found")
                } else {
                    Ok(todo_tasks)
                }
            }
            TaskStatus::InProgress => {
                let in_progress_tasks: Vec<&Task> = self
                    .tasks
                    .iter()
                    .filter(|t| t.status == TaskStatus::InProgress)
                    .collect();
                if in_progress_tasks.is_empty() {
                    Err("No in-progress tasks found")
                } else {
                    Ok(in_progress_tasks)
                }
            }
        }
    }

    pub fn list_all_tasks(&self) {
        if self.tasks.is_empty() {
            println_error("No tasks found");
        } else {
            for task in &self.tasks {
                println!("{:?}", task);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut tasks = Tasks::new(Vec::new());
        let t = tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        assert_eq!(t.0[0].id, 1);
        assert_eq!(t.0[0].description, "Test task");
        assert_eq!(t.0[0].status, TaskStatus::Todo);
        assert_eq!(t.0[0].created_at, "2021-07-01");
        assert_eq!(t.0[0].updated_at, "2021-07-01");
        assert!(t.0.len() == 1);
        assert_eq!(t.1, 1);
    }

    #[test]
    fn test_update_task_success() {
        let mut tasks = Tasks::new(Vec::new());
        tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        let t = tasks.update(1, &"Updated task".to_string(), "2021-07-02".to_string());
        let result = t.unwrap();
        assert_eq!(result[0].description, "Updated task");
        assert_eq!(result[0].updated_at, "2021-07-02");
    }

    #[test]
    fn test_update_non_existing_task() {
        let mut tasks = Tasks::new(Vec::new());
        tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        let result = tasks.update(2, &"Updated task".to_string(), "2021-07-02".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task with ID 2 not found");
    }

    #[test]
    fn test_delete_task() {
        let mut tasks = Tasks::new(Vec::new());
        tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        let result = tasks.delete(1);
        assert_eq!(result.unwrap().len(), 0)
    }

    #[test]
    fn test_delete_non_existing_task() {
        let mut tasks = Tasks::new(Vec::new());
        tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        let result = tasks.delete(2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task with ID 2 not found");
    }

    #[test]
    fn test_mark_todo() {
        let mut tasks = Tasks::new(Vec::new());
        tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        let result = tasks.mark_todo(1, "2021-07-01".to_string());
        assert_eq!(result.unwrap()[0].status, TaskStatus::Todo);
    }

    #[test]
    fn test_mark_todo_non_existing_task() {
        let mut tasks = Tasks::new(Vec::new());
        tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        let result = tasks.mark_todo(2, "2021-07-01".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task with ID 2 not found");
    }

    #[test]
    fn test_mark_in_progress() {
        let mut tasks = Tasks::new(Vec::new());
        tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        let result = tasks.mark_in_progress(1, "2021-07-01".to_string());
        assert_eq!(result.unwrap()[0].status, TaskStatus::InProgress);
    }

    #[test]
    fn test_mark_in_progress_non_existing_task() {
        let mut tasks = Tasks::new(Vec::new());
        tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        let result = tasks.mark_in_progress(2, "2021-07-01".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task with ID 2 not found");
    }

    #[test]
    fn test_mark_done() {
        let mut tasks = Tasks::new(Vec::new());
        tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        let result = tasks.mark_done(1, "2021-07-01".to_string());
        assert_eq!(result.unwrap()[0].status, TaskStatus::Done);
    }

    #[test]
    fn test_mark_done_non_existing_task() {
        let mut tasks = Tasks::new(Vec::new());
        tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        let result = tasks.mark_done(2, "2021-07-01".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task with ID 2 not found");
    }

    #[test]
    fn test_status_list_done_tasks() {
        let mut tasks = Tasks::new(Vec::new());
        tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        tasks.add(&"Test task 2".to_string(), "2021-07-01".to_string());
        tasks.add(&"Test task 3".to_string(), "2021-07-01".to_string());
        tasks.mark_done(1, "2021-07-01".to_string()).unwrap();
        tasks.mark_done(2, "2021-07-01".to_string()).unwrap();
        let result = tasks.list_status_task(TaskStatus::Done).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_list_all_tasks() {
        let mut tasks = Tasks::new(Vec::new());
        tasks.add(&"Test task".to_string(), "2021-07-01".to_string());
        tasks.add(&"Test task 2".to_string(), "2021-07-01".to_string());
        tasks.add(&"Test task 3".to_string(), "2021-07-01".to_string());
        tasks.list_all_tasks();
        assert_eq!(tasks.len(), 3);
    }
}
