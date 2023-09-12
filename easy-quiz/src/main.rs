use std::io;

// 定数の場合は、型を明示する必要がある
const QUIZ: [(&str, &str); 5] = [
    ("apple", "りんご"),
    ("banana", "ばなな"),
    ("orange", "おれんじ"),
    ("peach", "もも"),
    ("grape", "ぶどう"),
];

fn main() {
    println!("英単語をランダムに表示します。対応する日本語をひらがなで入力してね！");
    println!("問題は5問あるよ！");

    // 答え合わせ用の変数
    let mut count = 0;

    // ループ用の変数
    let mut i = 0;
    loop {
        println!("{}問目! {} ", i + 1, QUIZ[i].0);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // read_line メソッドを使用すると、入力の終わりに改行文字（\n）が含まれるのtirm()で削除
        let d = guess.trim();

        // 答え合わせ
        let check = answer_check(d, QUIZ[i].1);

        match check {
            true => {
                println!("正解！");
                count += 1;
            }
            false => {
                println!("不正解！");
                println!("{}", QUIZ[i].1);
            }
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

fn answer_check(input_word: &str, quiz_answer: &str) -> bool {
    if input_word == quiz_answer {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _りんごと入力するとtrueを返す() {
        assert_eq!(true, answer_check("りんご", QUIZ[0].1));
    }

    #[test]
    fn _改行コードを入力するとfalseを返す() {
        assert_eq!(false, answer_check("りんご\n", QUIZ[0].1));
    }
}
