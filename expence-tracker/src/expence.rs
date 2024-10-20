use crate::{
    io::save_expence,
    util::{println_error, println_error_with_id, println_success},
};

use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpencesList {
    pub expences: Vec<Expence>,
}

impl ExpencesList {
    pub fn new(expences: Vec<Expence>) -> ExpencesList {
        ExpencesList { expences }
    }

    pub fn len(&self) -> usize {
        self.expences.len()
    }

    pub fn add(&mut self, date: String, description: String, amount: i32) {
        let id = self.len() as i32 + 1;
        let new_expence = Expence::new(id, date, description, amount);
        self.expences.push(new_expence);
        save_expence(&self.expences);
        println_success("Expense added successfully!", id);
    }

    pub fn delete(&mut self, id: i32) {
        let task = self.expences.iter().position(|t| t.id == id);
        match task {
            Some(i) => {
                self.expences.remove(i);
                save_expence(&self.expences);
                println_success("Expense deleted successfully!", id);
            }
            None => {
                println_error_with_id("Expense not found", id);
            }
        }
    }

    pub fn list(&self) {
        println!("ID | Date       | Description | Amount");
        for t in self.expences.iter() {
            println!(
                "{}    {}   {}          ${}",
                t.id, t.date, t.description, t.amount
            );
        }
    }

    pub fn summary(&self, month: &Option<i32>) {
        if let Some(m) = month {
            let current_year = chrono::Local::now().format("%Y").to_string();
            let total: i32 = self
                .expences
                .iter()
                .filter(|t| t.date.starts_with(&format!("{}-{:02}-", current_year, m)))
                .map(|t| t.amount)
                .sum();
            println!("Total expenses for month {}: ${}", m, total);
        } else {
            let total: i32 = self.expences.iter().map(|t| t.amount).sum();
            println!("Total expenses: ${}", total);
        }
    }

    pub fn update(&mut self, id: i32, description: String, amount: i32) {
        let task = self.expences.iter_mut().find(|t| t.id == id);
        match task {
            Some(t) => {
                t.description = description;
                t.amount = amount;
                save_expence(&self.expences);
                println_success("Expense updated successfully!", id);
            }
            None => {
                println_error_with_id("Expense not found", id);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expence {
    pub id: i32,
    pub date: String,
    pub description: String,
    pub amount: i32,
}

impl Expence {
    pub fn new(id: i32, date: String, description: String, amount: i32) -> Expence {
        Expence {
            id,
            date,
            description,
            amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_expence() {
        let mut expences = ExpencesList::new(Vec::new());
        expences.add("2021-08-01".to_string(), "Lunch".to_string(), 20);
        assert_eq!(expences.len(), 1);
    }

    #[test]
    fn test_delete_expence() {
        let mut expences = ExpencesList::new(Vec::new());
        expences.add("2021-08-01".to_string(), "Lunch".to_string(), 20);
        expences.delete(1);
        assert_eq!(expences.len(), 0);
    }

    #[test]
    fn test_list() {
        let mut expences = ExpencesList::new(Vec::new());
        expences.add("2021-08-01".to_string(), "Lunch".to_string(), 20);
        expences.add("2021-08-02".to_string(), "Dinner".to_string(), 30);
        expences.list();
    }

    #[test]
    fn test_summary() {
        let mut expences = ExpencesList::new(Vec::new());
        expences.add("2021-08-01".to_string(), "Lunch".to_string(), 20);
        expences.add("2021-08-02".to_string(), "Dinner".to_string(), 30);
        expences.summary(&None);
        expences.summary(&Some(8));
    }

    #[test]
    fn test_update_expence() {
        let mut expences = ExpencesList::new(Vec::new());
        expences.add("2021-08-01".to_string(), "Lunch".to_string(), 20);
        expences.update(1, "Dinner".to_string(), 30);
    }
}
