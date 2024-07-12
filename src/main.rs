use lex::Lexer;

mod lex;

fn main() {
    let s = "10.28 + 27.6 * 75.4";
    for t in Lexer::new(s) {
        match t.data {
            lex::TokenData::Num(x) => print!("Num {x}: "),
            lex::TokenData::Sym(s) => print!("Sym {s}: "),
        }

        println!("'{}' ({})", t.substr, t.pos);
    }
}
