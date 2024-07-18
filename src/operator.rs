pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

pub fn try_get_binary_operator(data: char) -> Option<(BinaryOp, u8, bool)> {
    match data {
        '+' => Some((BinaryOp::Add, 10, false)),
        '-' => Some((BinaryOp::Sub, 10, false)),
        '*' => Some((BinaryOp::Mul, 20, false)),
        '/' => Some((BinaryOp::Div, 20, false)),
        _ => None,
    }
}

pub enum UnaryOp {
    Positive,
    Negative,
    Percent,
    Factorial,
    RootN(i8),
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
    Log,
    Undim,
}

pub fn try_get_prefix_operator(data: char) -> Option<UnaryOp> {
    match data {
        '+' => Some(UnaryOp::Positive),
        '-' => Some(UnaryOp::Negative),
        _ => None,
    }
}

pub fn try_get_postfix_operator(data: char) -> Option<UnaryOp> {
    match data {
        '%' => Some(UnaryOp::Percent),
        '!' => Some(UnaryOp::Factorial),
        _ => None,
    }
}

pub fn try_get_function(w: &str) -> Option<UnaryOp> {
    match w {
        "sqrt" => Some(UnaryOp::RootN(2)),
        "cbrt" => Some(UnaryOp::RootN(3)),
        "sin" => Some(UnaryOp::Sin),
        "cos" => Some(UnaryOp::Cos),
        "tan" => Some(UnaryOp::Tan),
        "exp" => Some(UnaryOp::Exp),
        "ln" => Some(UnaryOp::Ln),
        "log" => Some(UnaryOp::Log),
        _ => None,
    }
}
