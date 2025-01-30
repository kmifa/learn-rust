use crate::{
    io::{load_limit_list, save_expence},
    limit,
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

    pub fn add(&mut self, date: String, description: String, amount: i32, category: String) {
        let id = self.len() as i32 + 1;
        let new_expence = Expence::new(id, date.clone(), description, amount, category);
        self.expences.push(new_expence);
        // dateのymを取得して、その月の合計金額を計算する
        let amount = self.get_expence(date.clone());
        // dateのlimitを取得する
        let l = limit::LimitList::new(load_limit_list());
        let limit = l.get_limit(date.clone());

        // その月の合計金額がlimitを超えていたら警告を出す
        if amount.unwrap() > limit.unwrap() {
            println!(
                "You have exceeded the monthly limit of ${}, your current expenses are ${}",
                limit.unwrap(),
                amount.unwrap()
            );
        }

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
        // フォーマット文字列で使える特殊な記法
        println!(
            "{:<4} | {:<10} | {:<20} | {:>6} | {:<10}",
            "ID", "Date", "Description", "Amount", "Category"
        );
        println!("---------------------------------------------------------------");
        for t in self.expences.iter() {
            println!(
                "{:<4} | {:<10} | {:<20} | {:>6} | {:<10}",
                t.id, t.date, t.description, t.amount, t.category
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
        // 更新する項目の合計金額と限度額を先に計算しておく
        let task_date = self
            .expences
            .iter()
            .find(|t| t.id == id)
            .map(|t| t.date.clone());
        let amo = task_date
            .as_ref()
            .and_then(|date| self.get_expence(date.clone()));
        let l = limit::LimitList::new(load_limit_list());
        let limit = task_date
            .as_ref()
            .and_then(|date| l.get_limit(date.clone()));

        // 可変借用が衝突しないように self.expences の可変参照はここで行う
        let task = self.expences.iter_mut().find(|t| t.id == id);

        // その月の合計金額が limit を超えていたら警告を出す
        if let (Some(amo), Some(limit)) = (amo, limit) {
            if amo > limit {
                println!(
                    "You have exceeded the monthly limit of ${}, your current expenses are ${}",
                    limit, amo
                );
            }
        }

        // タスクの更新処理
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

    fn get_expence(&self, date: String) -> Option<i32> {
        let ym = date.split('-').take(2).collect::<Vec<&str>>().join("-");
        let a: i32 = self
            .expences
            .iter()
            .filter(|x| x.date.split('-').take(2).collect::<Vec<&str>>().join("-") == ym)
            .map(|t| t.amount)
            .sum();

        Some(a)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expence {
    pub id: i32,
    pub date: String,
    pub description: String,
    pub amount: i32,
    pub category: String,
}

impl Expence {
    pub fn new(
        id: i32,
        date: String,
        description: String,
        amount: i32,
        category: String,
    ) -> Expence {
        Expence {
            id,
            date,
            description,
            amount,
            category,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_expence() {
        let mut expences = ExpencesList::new(Vec::new());
        expences.add(
            "2021-08-01".to_string(),
            "Lunch".to_string(),
            20,
            "Food".to_string(),
        );
        assert_eq!(expences.len(), 1);
    }

    #[test]
    fn test_delete_expence() {
        let mut expences = ExpencesList::new(Vec::new());
        expences.add(
            "2021-08-01".to_string(),
            "Lunch".to_string(),
            20,
            "Food".to_string(),
        );
        expences.delete(1);
        assert_eq!(expences.len(), 0);
    }

    #[test]
    fn test_get_expence() {
        let mut expences = ExpencesList::new(Vec::new());
        expences.add(
            "2021-08-01".to_string(),
            "Lunch".to_string(),
            20,
            "Food".to_string(),
        );
        expences.add(
            "2021-08-02".to_string(),
            "Dinner".to_string(),
            30,
            "Food".to_string(),
        );
        assert_eq!(expences.get_expence("2021-08-01".to_string()), Some(50));
    }
}
