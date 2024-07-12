use std::{iter::Peekable, str::CharIndices};

#[derive(Clone)]
pub enum Sym {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone)]
pub enum TokenData {
    Num(f64),
    Sym(Sym),
    LBrack,
    RBrack,
}

#[derive(Clone)]
pub struct Token<'a> {
    pub data: TokenData,
    pub pos: usize,
    pub substr: &'a str,
}

pub struct Lexer<'a> {
    pub original: &'a str,
    pub chars: Peekable<CharIndices<'a>>,
    pub actual_pos: usize,
    pub token_start: usize,
    pub token_start_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            chars: s.char_indices().peekable(),
            actual_pos: 0,
            token_start: 0,
            token_start_pos: 0,
        }
    }

    fn step_char(&mut self) {
        self.chars.next();
        self.actual_pos += 1;
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    fn start_token(&mut self) {
        if let Some(&(start, _)) = self.chars.peek() {
            self.token_start = start;
            self.token_start_pos = self.actual_pos;
        }
    }

    fn get_substr(&mut self) -> &'a str {
        match self.chars.peek() {
            None => &self.original[self.token_start..],
            Some(&(end, _)) => &self.original[self.token_start..end],
        }
    }

    fn make_token(&mut self, data: TokenData) -> Token<'a> {
        Token {
            data,
            pos: self.token_start_pos,
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

    fn lex_token(&mut self, curr: char) -> Token<'a> {
        self.start_token();
        if curr.is_ascii_digit() || curr == '.' {
            self.lex_num()
        } else {
            self.step_char();
            match curr {
                '(' => self.make_token(TokenData::LBrack),
                ')' => self.make_token(TokenData::RBrack),
                '+' => self.make_token(TokenData::Sym(Sym::Add)),
                '-' => self.make_token(TokenData::Sym(Sym::Sub)),
                '*' => self.make_token(TokenData::Sym(Sym::Mul)),
                '/' => self.make_token(TokenData::Sym(Sym::Div)),
                _ => todo!("implement Lex errors"),
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
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
