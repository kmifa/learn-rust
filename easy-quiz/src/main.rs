use std::io;

fn main() {
    println!("英単語をランダムに表示します。対応する日本語をひらがなで入力してね！");
    println!("問題は5問あるよ！");

    let quiz = [
        ("apple", "りんご"),
        ("banana", "ばなな"),
        ("orange", "おれんじ"),
        ("peach", "もも"),
        ("grape", "ぶどう"),
    ];

    // 答え合わせ用の変数
    let mut count = 0;

    // ループ用の変数
    let mut i = 0;
    loop {
        println!("{}問目! {} ", i + 1, quiz[i].0);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 不正入力検知
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        // read_line メソッドを使用すると、入力の終わりに改行文字（\n）が含まれる点にあります
        let d = guess.trim();

        // 答え合わせ
        if d == quiz[i].1 {
            println!("正解！");
            count += 1;
        } else {
            println!("不正解！");
            println!("{}", quiz[i].1);
        }

        // 5問終わったら終了
        if i == 4 {
            println!("終了！あなたの正解数は{}問です！", count);
            break;
        }

        i += 1;
    }

    // 入力文字を保持するための変数
}
