use crate::error;
use std::{iter::Peekable, str::CharIndices};

#[derive(Clone, Debug)]
pub enum TokenData {
    Num(f64),
    Int(i64),
    Add,
    Sub,
    Mul,
    Div,
    LBrack,
    RBrack,
    EndOfInput,
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub data: TokenData,
    pub start_pos: usize,
    pub end_pos: usize,
    pub substr: &'a str,
}

pub struct LexIter<'a> {
    original: &'a str,
    chars: Peekable<CharIndices<'a>>,
    curr_actual_pos: usize,
    token_start_pos: usize,
    token_start_byte: usize,
}

type LexResult<'a> = Result<Token<'a>, error::ParseError<'a>>;

impl<'a> LexIter<'a> {
    pub fn new(s: &'a str) -> LexIter<'a> {
        LexIter {
            original: s,
            chars: s.char_indices().peekable(),
            curr_actual_pos: 0,
            token_start_byte: 0,
            token_start_pos: 0,
        }
    }

    fn step_char(&mut self) {
        if self.chars.next().is_some() {
            self.curr_actual_pos += 1;
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    fn start_token(&mut self) {
        if let Some(&(start, _)) = self.chars.peek() {
            self.token_start_byte = start;
            self.token_start_pos = self.curr_actual_pos;
        }
    }

    fn get_substr(&mut self) -> &'a str {
        match self.chars.peek() {
            None => &self.original[self.token_start_byte..],
            Some(&(end, _)) => &self.original[self.token_start_byte..end],
        }
    }

    fn make_token(&mut self, data: TokenData) -> Token<'a> {
        Token {
            data,
            start_pos: self.token_start_pos,
            end_pos: self.curr_actual_pos,
            substr: self.get_substr(),
        }
    }

    fn try_parse_num(&mut self) -> LexResult<'a> {
        let substr = self.get_substr();
        if let Ok(n) = substr.parse::<i64>() {
            Ok(self.make_token(TokenData::Int(n)))
        } else if let Ok(x) = substr.parse::<f64>() {
            Ok(self.make_token(TokenData::Num(x)))
        } else {
            Err(error::ParseError {
                error: String::from("Unable to parse number"),
                start_pos: self.token_start_pos,
                end_pos: self.curr_actual_pos,
                original: self.original,
            })
        }
    }

    fn lex_num(&mut self) -> LexResult<'a> {
        let mut has_parsed_digits = false;
        while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
            self.step_char();
            has_parsed_digits = true;
        }

        if self.peek_char().is_some_and(|c| c == '.') {
            self.step_char();
            while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                self.step_char();
                has_parsed_digits = true;
            }
        }
        if !has_parsed_digits {
            return Err(error::ParseError {
                error: String::from("Lone decimal dot with no digits"),
                start_pos: self.token_start_pos,
                end_pos: self.curr_actual_pos,
                original: self.original,
            });
        }

        if self.peek_char().is_some_and(|c| c == 'e' || c == 'E') {
            self.step_char();
            if self.peek_char().is_some_and(|c| c == '-') {
                self.step_char();
            }
            if !self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                return Err(error::ParseError {
                    error: String::from("Missing exponent"),
                    start_pos: self.curr_actual_pos,
                    end_pos: self.curr_actual_pos,
                    original: self.original,
                });
            }
            self.step_char();
            while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                self.step_char();
            }
        }

        self.try_parse_num()
    }

    fn lex_token(&mut self, curr: char) -> LexResult<'a> {
        self.start_token();
        if curr.is_ascii_digit() || curr == '.' {
            return self.lex_num();
        }

        self.step_char();
        match curr {
            '(' => Ok(self.make_token(TokenData::LBrack)),
            ')' => Ok(self.make_token(TokenData::RBrack)),
            '+' => Ok(self.make_token(TokenData::Add)),
            '-' => Ok(self.make_token(TokenData::Sub)),
            '*' => Ok(self.make_token(TokenData::Mul)),
            '/' => Ok(self.make_token(TokenData::Div)),
            _ => Err(error::unknown_symbol(
                curr,
                self.token_start_pos,
                self.original,
            )),
        }
    }
}

impl<'a> Iterator for LexIter<'a> {
    type Item = LexResult<'a>;

    fn next(&mut self) -> Option<LexResult<'a>> {
        while let Some(c) = self.peek_char() {
            if c == ' ' {
                self.step_char();
                continue;
            }
            return Some(self.lex_token(c));
        }

        None
    }
}

pub struct Lexer<'a> {
    lex_iter: Peekable<LexIter<'a>>,
    last_token_end_pos: usize,
    pub original: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            lex_iter: LexIter::new(s).peekable(),
            last_token_end_pos: 0,
            original: s,
        }
    }

    pub fn make_eof_token(&self) -> Token<'a> {
        Token {
            data: TokenData::EndOfInput,
            start_pos: self.last_token_end_pos,
            end_pos: self.last_token_end_pos,
            substr: "end of input",
        }
    }

    pub fn next_token(&mut self) -> LexResult<'a> {
        match self.lex_iter.next() {
            Some(token) => {
                let token = token?;
                self.last_token_end_pos = token.end_pos;
                Ok(token)
            }
            None => Ok(self.make_eof_token()),
        }
    }

    pub fn peek_token(&mut self) -> LexResult<'a> {
        match self.lex_iter.peek() {
            Some(token) => token.clone(),
            None => Ok(self.make_eof_token()),
        }
    }
}
