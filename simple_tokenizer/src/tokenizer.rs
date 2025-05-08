use std::str::Chars;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LParen,
    RParen,
    Error(String),
    //EOL,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Tokenizer<'a> {
    input: &'a str,
    chars_iter: std::iter::Peekable<Chars<'a>>,
}

#[allow(dead_code)]
impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input,
            chars_iter: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        /*
            主要包含两段处理逻辑：
            1. 遇到空白
            2. 遇到非空白
        */
        // 跳过遇到的空白字符
        while let Some(&c) = self.chars_iter.peek() {
            if c.is_whitespace() {
                self.chars_iter.next();
            } else {
                break;
            }
        }

        // 非空白字符
        match self.chars_iter.next() {
            Some(c) => Some(
                self.match_char(c)
            ), 
            None => None
        }
    }
    
    fn match_char(&self, c: char) -> Token {
        match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '^' => Token::Power,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '0'..='9' => Token::Number(c.to_string().parse::<i32>().unwrap()),
            _ => Token::Error("Unexpected char -> {c}".to_owned()), 
        }
    }
}

#[allow(dead_code)]
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
