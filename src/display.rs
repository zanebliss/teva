use std::fmt::Display;

pub enum Color {
    Blue,
    Yellow,
    Red,
    None,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match &self {
            Color::Blue => 94,
            Color::Yellow => 33,
            Color::Red => 91,
            Color::None => 0,
        };

        write!(f, "{code}")
    }
}

pub enum Fd {
    Stdout,
    Stderr,
}
