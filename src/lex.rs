use crate::error;
use std::{iter::Peekable, str::CharIndices};

#[derive(Clone, Debug)]
pub enum Sym {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug)]
pub enum TokenData {
    Num(f64),
    Sym(Sym),
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

type LexResult<'a> = Result<Token<'a>, error::ParseError>;

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

    fn lex_num(&mut self) -> Token<'a> {
        let mut integral_part = 0.0;
        while let Some(d) = self.peek_char().and_then(|c| c.to_digit(10)) {
            integral_part *= 10.0;
            integral_part += d as f64;
            self.step_char();
        }

        if self.peek_char().filter(|&c| c == '.').is_some() {
            self.step_char();
            let mut decimal_part = 0.0;
            let mut denominator = 1.0;
            while let Some(d) = self.peek_char().and_then(|c| c.to_digit(10)) {
                decimal_part *= 10.0;
                decimal_part += d as f64;
                denominator *= 10.0;
                self.step_char();
            }

            self.make_token(TokenData::Num(integral_part + decimal_part / denominator))
        } else {
            self.make_token(TokenData::Num(integral_part))
        }
    }

    fn lex_token(&mut self, curr: char) -> LexResult<'a> {
        self.start_token();
        if curr.is_ascii_digit() || curr == '.' {
            Ok(self.lex_num())
        } else {
            self.step_char();
            match curr {
                '(' => Ok(self.make_token(TokenData::LBrack)),
                ')' => Ok(self.make_token(TokenData::RBrack)),
                '+' => Ok(self.make_token(TokenData::Sym(Sym::Add))),
                '-' => Ok(self.make_token(TokenData::Sym(Sym::Sub))),
                '*' => Ok(self.make_token(TokenData::Sym(Sym::Mul))),
                '/' => Ok(self.make_token(TokenData::Sym(Sym::Div))),
                _ => Err(error::unknown_symbol(curr, self.token_start_pos)),
            }
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

pub struct TokenLexer<'a> {
    lex_iter: Peekable<LexIter<'a>>,
    last_token_end_pos: usize,
}

impl<'a> TokenLexer<'a> {
    pub fn new(s: &'a str) -> TokenLexer<'a> {
        TokenLexer {
            lex_iter: LexIter::new(s).peekable(),
            last_token_end_pos: 0,
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
