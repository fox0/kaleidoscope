use std::str::Chars;


#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Def,
    Extern,
    /// ';' character
    Delimiter,
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident(String),
    Number(f64),
    Operator(char),
}

pub struct Tokenizer<'a> {
    chars: Chars<'a>,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(text: &'a str) -> Self {
        Self { chars: text.chars() }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.chars.next() {
            return Some(Token::Operator(v));
        }
        None
    }
}
