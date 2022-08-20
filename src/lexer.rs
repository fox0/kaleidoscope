use lazy_static::lazy_static;
use regex::{Regex, RegexSet};


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
    Operator(String),
}

pub struct Tokenizer {
    text: String,
}

impl Tokenizer {
    pub fn new<T: Into<String>>(text: T) -> Self {
        let text = text.into();
        let text = Self::preprocessing(text);
        Self { text }
    }

    fn preprocessing(text: String) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?m)#.*\n").unwrap();
        }
        let text = RE.replace_all(text.as_str(), "\n");
        text.into()
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let set = RegexSet::new(&[
            r"\w+",
            r"\d+",
            r"\pL+",
            r"foo",
            r"bar",
            r"barfoo",
            r"foobar",
        ]).unwrap();

        todo!()
    }
}
