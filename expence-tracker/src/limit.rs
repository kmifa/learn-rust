use serde::{Deserialize, Serialize};

use crate::{io::save_limit, util::println_error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Limit {
    // 月の支出上限
    pub limit: i32,
    // 年月 yyyy-mm
    pub ym: String,
}

impl Limit {
    pub fn new(limit: i32, ym: String) -> Limit {
        Limit { limit, ym }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitList {
    pub limits: Vec<Limit>,
}

impl LimitList {
    pub fn new(limits: Vec<Limit>) -> LimitList {
        LimitList { limits }
    }

    pub fn add(&mut self, limit: i32, ym: String) {
        self.limits.push(Limit::new(limit, ym));
        save_limit(&self.limits);
    }

    pub fn update(&mut self, limit: i32, ym: String) {
        if let Some(i) = self.limits.iter_mut().find(|x| x.ym == ym) {
            i.limit = limit;
        } else {
            println_error("This Year-Month does not exist");
        }
    }

    pub fn delete(&mut self, ym: String) {
        self.limits.retain(|x| x.ym == ym);
        save_limit(&self.limits);
    }

    pub fn get_limit(&self, date: String) -> Option<i32> {
        // dateをyyyy-mmに変換
        let ym = date.split('-').take(2).collect::<Vec<&str>>().join("-");
        let limit = self.limits.iter().find(|x| x.ym == ym);
        limit.map(|x| x.limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_limit() {
        let mut limits = LimitList::new(Vec::new());
        limits.add(100, "2021-08".to_string());
        assert_eq!(limits.limits[0].limit, 100);
        assert_eq!(limits.limits[0].ym, "2021-08");
    }

    #[test]
    fn test_delete_limit() {
        let mut limits = LimitList::new(Vec::new());
        limits.add(100, "2021-08".to_string());
        limits.delete("2021-08".to_string());
        assert_eq!(limits.limits.len(), 0);
    }

    #[test]
    fn test_update_limit() {
        let mut limits = LimitList::new(Vec::new());
        limits.add(100, "2021-08".to_string());
        limits.update(200, "2021-08".to_string());
        assert_eq!(limits.limits[0].limit, 200);
    }
}
