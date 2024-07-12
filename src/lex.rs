use std::{iter::Peekable, str::CharIndices};

pub enum TokenData {
    Num(f64),
    Sym(char),
}

pub struct Token<'a> {
    pub data: TokenData,
    pub pos: usize,
    pub substr: &'a str,
}

pub struct Lexer<'a> {
    pub original: &'a str,
    pub chars: Peekable<CharIndices<'a>>,
    pub pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            chars: s.char_indices().peekable(),
            pos: 0,
        }
    }

    fn step_char(&mut self) -> usize {
        if self.chars.next().is_some() {
            let oldpos = self.pos;
            self.pos += 1;
            oldpos
        } else {
            self.pos
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    fn get_substr(&mut self, start: usize) -> &'a str {
        match self.chars.peek() {
            None => &self.original[start..],
            Some(&(end, _)) => &self.original[start..end],
        }
    }

    fn lex_num(&mut self, start: usize) -> Token<'a> {
        let pos = self.pos;
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

            Token {
                data: TokenData::Num(integral_part + decimal_part / denominator),
                pos,
                substr: self.get_substr(start),
            }
        } else {
            Token {
                data: TokenData::Num(integral_part),
                pos,
                substr: self.get_substr(start),
            }
        }
    }

    fn lex(&mut self, curr: char, start: usize) -> Token<'a> {
        if curr.is_ascii_digit() || curr == '.' {
            self.lex_num(start)
        } else {
            let pos = self.step_char();
            Token {
                data: TokenData::Sym(curr),
                pos,
                substr: self.get_substr(start),
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        while let Some(&(p, c)) = self.chars.peek() {
            if c == ' ' {
                self.step_char();
                continue;
            }
            return Some(self.lex(c, p));
        }

        None
    }
}
