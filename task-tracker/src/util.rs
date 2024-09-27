pub fn println_success(s: &str, id: i32) {
    println!("\x1B[34m{} (ID: {})\x1B[0m", s, id);
}

pub fn println_error_with_id(s: &str, id: i32) {
    println!("\x1B[31m{} (ID: {})\x1B[0m", s, id);
}

pub fn println_error(s: &str) {
    println!("\x1B[31m{}\x1B[0m", s);
}
