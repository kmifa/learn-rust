fn main() {
    nlp_01();
}

fn nlp_00() {
    let str = "stressed";
    let re = str.chars().rev().collect::<String>();
    println!("{}", re);
    assert_eq!(re, "desserts");
}

fn nlp_01() {
    let str = "パタトクカシーー";
    let re = str.chars().step_by(2).collect::<String>();
    println!("{}", re);
    assert_eq!(re, "パトカー");
}
