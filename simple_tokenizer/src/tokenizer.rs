use std::str::Chars;

#[allow(dead_code)]
#[derive(Debug, Clone)]
#[derive(PartialEq)]
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
    chars_iter: std::iter::Peekable<Chars<'a>>,
}

#[allow(dead_code)]
impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
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

    fn match_char(&mut self, c: char) -> Token {
        match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '^' => Token::Power,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '0'..='9' => {
                let mut num_str = c.to_string();
                // 遇到char == digit时就peek下一个，如果还是digit，就next，然后循环一直到peek的下一个字符不是digit
                while let Some(&next_c) = self.chars_iter.peek() {
                    if next_c.is_ascii_digit() {
                        num_str.push(next_c);
                        self.chars_iter.next();
                    } else {
                        break
                    }
                }
                Token::Number(num_str.parse::<i32>().unwrap())
            }
            _ => Token::Error(format!("Unexpected char -> {}", c)),
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

////////////////
// unit test
///////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_char_single_symbols() {
        let mut tokenizer = Tokenizer::new("");
        assert_eq!(tokenizer.match_char('+'), Token::Plus);
        assert_eq!(tokenizer.match_char('-'), Token::Minus);
        assert_eq!(tokenizer.match_char('*'), Token::Multiply);
        assert_eq!(tokenizer.match_char('/'), Token::Divide);
        assert_eq!(tokenizer.match_char('^'), Token::Power);
        assert_eq!(tokenizer.match_char('('), Token::LParen);
        assert_eq!(tokenizer.match_char(')'), Token::RParen);
    }

    #[test]
    fn test_match_char_digit() {
        let mut tokenizer = Tokenizer::new("");
        assert_eq!(tokenizer.match_char('0'), Token::Number(0));
        assert_eq!(tokenizer.match_char('5'), Token::Number(5));
        assert_eq!(tokenizer.match_char('9'), Token::Number(9));
    }

    #[test]
    fn test_match_char_unexpected() {
        let mut tokenizer = Tokenizer::new("");
        let token = tokenizer.match_char('x');
        match token {
            Token::Error(msg) => assert!(msg.contains("x")),
            _ => panic!("Expected error token"),
        }
    }

    #[test]
    fn test_next_token_basic() {
        let mut tokenizer = Tokenizer::new(" + - * / ^ ( ) 1 2 3");
        assert_eq!(tokenizer.next_token(), Some(Token::Plus));
        assert_eq!(tokenizer.next_token(), Some(Token::Minus));
        assert_eq!(tokenizer.next_token(), Some(Token::Multiply));
        assert_eq!(tokenizer.next_token(), Some(Token::Divide));
        assert_eq!(tokenizer.next_token(), Some(Token::Power));
        assert_eq!(tokenizer.next_token(), Some(Token::LParen));
        assert_eq!(tokenizer.next_token(), Some(Token::RParen));
        assert_eq!(tokenizer.next_token(), Some(Token::Number(1)));
        assert_eq!(tokenizer.next_token(), Some(Token::Number(2)));
        assert_eq!(tokenizer.next_token(), Some(Token::Number(3)));
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_iterator_trait() {
        let tokens: Vec<Token> = Tokenizer::new("1 + 2").collect();
        assert_eq!(tokens, vec![
            Token::Number(1),
            Token::Plus,
            Token::Number(2),
        ]);
    }

    #[test]
    fn test_skip_whitespace() {
        let mut tokenizer = Tokenizer::new("   +   ");
        assert_eq!(tokenizer.next_token(), Some(Token::Plus));
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_empty_input() {
        let mut tokenizer = Tokenizer::new("");
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_match_char_single_digit() {
        let mut tokenizer = Tokenizer::new("1");
        tokenizer.chars_iter.next();
        let token = tokenizer.match_char('1');
        assert_eq!(token, Token::Number(1));
    }

    #[test]
    fn test_match_char_multi_digit() {
        let mut tokenizer = Tokenizer::new("1234");
        // 手动先吃掉第一个字符，让 chars_iter 从 index 1 开始
        tokenizer.chars_iter.next();
        let token = tokenizer.match_char('1');
        assert_eq!(token, Token::Number(1234));
    }

    #[test]
    fn test_match_char_digit_followed_by_symbol() {
        let mut tokenizer = Tokenizer::new("12+");
        // 手动先吃掉第一个字符
        tokenizer.chars_iter.next();
        let token = tokenizer.match_char('1');
        assert_eq!(token, Token::Number(12));
        // 检查下一个字符是 '+'
        assert_eq!(tokenizer.chars_iter.peek(), Some(&'+'));
    }

    #[test]
    fn test_match_char_digit_followed_by_whitespace() {
        let mut tokenizer = Tokenizer::new("456 ");
        tokenizer.chars_iter.next();
        let token = tokenizer.match_char('4');
        assert_eq!(token, Token::Number(456));
        // peek() 是空格
        assert_eq!(tokenizer.chars_iter.peek(), Some(&' '));
    }

    #[test]
    fn test_match_char_symbol() {
        let mut tokenizer = Tokenizer::new("+");
        tokenizer.chars_iter.next();
        let token = tokenizer.match_char('+');
        assert_eq!(token, Token::Plus);
    }

    #[test]
    fn test_match_char_unexpected2() {
        let mut tokenizer = Tokenizer::new("x");
        tokenizer.chars_iter.next();
        let token = tokenizer.match_char('x');
        match token {
            Token::Error(msg) => assert!(msg.contains("x")),
            _ => panic!("Expected error token"),
        }
    }
}

