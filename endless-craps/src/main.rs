use regex::Regex;
use std::io;

#[derive(Debug, PartialEq)]
struct WordCheck {
    quote_last_word: String, // 前の言葉から引用した文字
    add_word: String,        // 自分で追加した文字
}

fn main() {
    println!("エンドレスクラップス始まるよ");

    // ループ用の変数
    let mut i = 0;

    let mut last_word = String::from("みかん");

    let mut score = 0;

    loop {
        println!("前の言葉は「{}」", last_word);
        println!("サイコロを振るよ！");

        // サイコロを振る
        let dice = rand::random::<u8>() % 6 + 1;
        println!("出た目は{}だよ！", dice);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        let is_ok = answer_check(&last_word, &guess, dice as u8);

        if is_ok {
            println!("{}点追加", guess.chars().count());
            score += guess.chars().count();
            last_word = guess.to_string();
        } else {
            println!("間違えたよ！{}点減点", dice);
            score -= dice as usize;
        }

        if i == 3 {
            println!("終了！");
            println!("今回のscoreは{}点だよ！", score);
            break;
        }

        i += 1;
    }
}

fn answer_check(last_word: &str, answer: &str, dice_value: u8) -> bool {
    // 1
    if answer.chars().next().unwrap() == 'ん' {
        return false;
    }

    if answer.chars().count() < dice_value as usize {
        println!("出目より文字が少ないよ");
        return false;
    }

    // 2
    let substring_answer = substring(answer, dice_value as usize);

    // 出目と追加した文字の数が一致していなかったらfalseを返す
    if dice_value as usize != substring_answer.add_word.chars().count() {
        return false;
    }

    // answerの最初の文字とlast_wordの最後の文字が一致しているか
    if last_word.ends_with(substring_answer.quote_last_word.as_str()) {
        return true;
    } else {
        println!("最後の文字が一致していません");
        return false;
    }
}

fn substring(word: &str, dice_value: usize) -> WordCheck {
    let clone = word.clone().to_string();
    // 文字列の長さを取得
    let count = clone.chars().count();
    WordCheck {
        quote_last_word: clone.chars().skip(0).take(count - dice_value).collect(),
        add_word: clone.chars().skip(count - dice_value).take(count).collect(),
    }
}

#[cfg(test)]

// [x] answerが「ん」からはじまったらfalseを返す
// [x] 前の単語が「みかん」、出目が2の場合、「かんさい」と答えるとtrueを返す
// [x] 出目より単語が短い場合はfalseを返す
mod tests {
    use super::*;

    #[test]
    fn _答えがんから始まったらfalseを返す() {
        assert_eq!(answer_check("みかん", "んかんだん", 0), false);
    }

    #[test]
    fn _出目が4で答えが3文字の場合はfalseを返す() {
        assert_eq!(answer_check("みかん", "かんじ", 4), false);
    }

    #[test]
    fn _さいころの出目が2の場合はみかんの後にかんさいと返すとtrueを返す() {
        assert_eq!(answer_check("みかん", "かんさい", 2), true);
    }

    #[test]
    fn _さいころの出目が2の場合はみかんの後にかんさいと返すとfalseを返す() {
        assert_eq!(answer_check("みかん", "かんさいくうこう", 2), false);
    }

    #[test]
    fn _かんさいくうこうといれてかんとさいくうこうで分割されること() {
        assert_eq!(
            substring("かんさいくうこう", 6),
            WordCheck {
                quote_last_word: "かん".to_string(),
                add_word: "さいくうこう".to_string(),
            }
        );
    }
}
