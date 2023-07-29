use std::str::Chars;

use crate::entities::{Token, self};


#[derive(Debug)]
pub struct LineParser<'a> {
    pub source: Box<String>,
    chars: Box<Chars<'a>>,
    state: State,
    stacked_tokens: Vec<Token>,
}

#[derive(Debug)]
enum State {
    Normal,
    Brace,
}

impl<'a> LineParser<'a> {
    pub fn new(line: &'a str) -> Self {
        Self {
            source: Box::new(line.to_string()),
            chars: Box::new(line.chars()),
            state: State::Normal,
            stacked_tokens: vec![],
        }
    }
}

impl Iterator for LineParser<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.stacked_tokens.pop() {
            return Some(token);
        }
        let mut token: Option<Token> = None;
        match &self.state {
            State::Normal => {
                let mut texts = vec![];
                for char in self.chars.by_ref() {
                    match char {
                        '。' | '！' | '？' => {
                            self.stacked_tokens.push(Token::EndOfSentence(char.to_string()));
                            token = Some(entities::Token::Text(texts.concat()));
                            break;
                        }
                        '、' | ',' => {
                            self.stacked_tokens.push(Token::Comma(char.to_string()));
                            token = Some(entities::Token::Text(texts.concat()));
                            break;
                        }
                        '{' => {
                            self.state = State::Brace;
                            token = self.next();
                            break;
                        }
                        _ => {
                            texts.push(char.to_string());
                        }
                    }
                }
            }
            State::Brace => {
                let mut texts = vec![];
                for char in self.chars.by_ref() {
                    match char {
                        '}' => {
                            self.state = State::Normal;
                            let rich_text: String = texts.concat();
                            let mut richtext_parser = crate::parser::richtext_parser::RichTextParser::new(rich_text.as_str());
                            token = Some(richtext_parser.parse());
                            break;
                        }
                        _ => {
                            texts.push(char.to_string());
                        }
                    }
                }
            }
        };
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod next {
        use super::*;

        #[test]
        fn it_returns_text_token() {
            let expected = vec![
                entities::Token::Text("我が輩は".to_string()),
                entities::Token::Comma("、".to_string()),
                entities::Token::RichText("猫".to_string(), entities::Attribute::Ruby("ねこ".to_string())),
                entities::Token::Text("である".to_string()),
                entities::Token::EndOfSentence("。".to_string()),
                entities::Token::Text("名前は".to_string()),
                entities::Token::Comma("、".to_string()),
                entities::Token::Text("まだ無い".to_string()),
                entities::Token::EndOfSentence("。".to_string()),
            ];
            let parser = LineParser::new("我が輩は、{猫|ねこ}である。名前は、まだ無い。");
            let actual = parser.collect::<Vec<Token>>();
            assert_eq!(actual, expected);
        }
    }
}