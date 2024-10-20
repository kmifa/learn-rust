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
