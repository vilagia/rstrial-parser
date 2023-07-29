pub mod entities;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn parse_line(line: &str) -> Vec<entities::Token> {
    let mut tokens = vec![];
    let mut plainstrings = vec![];
    for char in line.chars() {
        match char {
            '。' | '！' | '？' => {
                tokens.push(entities::Token::Text(plainstrings.concat()));
                tokens.push(entities::Token::EndOfSentence(char.to_string()));
                plainstrings.clear();
            }
            '、' | ',' => {
                tokens.push(entities::Token::Text(plainstrings.concat()));
                tokens.push(entities::Token::Comma(char.to_string()));
                plainstrings.clear();
            }
            _ => {
                plainstrings.push(char.to_string());
            }
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    mod parse_line {
        use super::*;

        #[test]
        fn it_returns_text_token() {
            let result = parse_line("我が輩は、猫である。名前は、まだ無い。");
            let expected = vec![
                entities::Token::Text("我が輩は".to_string()),
                entities::Token::Comma("、".to_string()),
                entities::Token::Text("猫である".to_string()),
                entities::Token::EndOfSentence("。".to_string()),
                entities::Token::Text("名前は".to_string()),
                entities::Token::Comma("、".to_string()),
                entities::Token::Text("まだ無い".to_string()),
                entities::Token::EndOfSentence("。".to_string()),
            ];
            assert_eq!(result, expected);
        }
    }
}
