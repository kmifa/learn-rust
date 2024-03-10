fn main() {
    let str = "stressed";
    let re = str.chars().rev().collect::<String>();
    println!("{}", re);
    assert_eq!(re, "desserts");
}
