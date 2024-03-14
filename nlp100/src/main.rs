fn main() {
    nlp_04();
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

fn nlp_04() {
    let str = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";

    let re = str
        .split_whitespace()
        .enumerate()
        .map(|(i, s)| {
            let mut n = 1;
            if [1, 5, 6, 7, 8, 9, 15, 16, 19].contains(&i) {
                n = 2;
            }
            (s.chars().take(n).collect::<String>(), i)
        })
        .collect::<Vec<_>>();

    println!("{:?}", re);
}
