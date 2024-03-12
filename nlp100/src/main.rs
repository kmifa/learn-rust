fn main() {
    nlp_03();
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

fn nlp_02() {
    let str1 = "パトカー";
    let str2 = "タクシー";
    let mut re = String::new(); // Initialize the `str` variable
    for (c1, c2) in str1.chars().zip(str2.chars()) {
        re.push(c1);
        re.push(c2);
    }

    println!("{}", re);
    assert_eq!(re, "パタトクカシーー");
}

fn nlp_03() {
    let str = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let re = str
        .split_whitespace()
        .map(|s| s.chars().nth(0).unwrap())
        .collect::<String>();
    // let s = re.concat::<Chars>();
    println!("{:?}", re);
    assert_eq!(re, "NInadaocathliqm")
}
