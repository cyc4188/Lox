
pub fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

pub fn is_alpha(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

pub fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}


