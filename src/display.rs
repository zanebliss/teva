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

pub struct Logger {
    pub colors_enabled: bool,
    show_prefix: bool,
    color: Color,
    stream: Fd,
    text: String,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            colors_enabled: true,
            show_prefix: true,
            color: Color::None,
            text: "".to_string(),
            stream: Fd::Stdout,
        }
    }

    pub fn with_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn with_text(&mut self, text: String) -> &mut Self {
        self.text = text.to_string();
        self
    }

    pub fn with_stream(&mut self, stream: Fd) -> &mut Self {
        self.stream = stream;
        self
    }

    pub fn without_prefix(&mut self) -> &mut Self {
        self.show_prefix = false;
        self
    }

    pub fn call(&mut self) {
        let formatted_text = if self.colors_enabled {
            self.format_text(&self.color, &self.text)
        } else {
            self.format_text(&Color::None, &self.text)
        };

        if self.show_prefix {
            print!("{}", self.prefix())
        };

        match self.stream {
            Fd::Stdout => print!(" {formatted_text}"),
            Fd::Stderr => eprint!(" {formatted_text}") 
        }

        self.reset();
    }

    fn reset(&mut self) {
        self.show_prefix = true;
        self.color = Color::None;
        self.text = "".to_string();
        self.stream = Fd::Stdout;
    }

    fn format_text(&self, color: &Color, text: &String) -> String {
        match color {
            Color::Blue | Color::Yellow | Color::Red => {
                format!("\x1b[{color}m{text}\x1b[0m")
            }
            Color::None => format!("{text}"),
        }
    }

    fn prefix(&self) -> String {
        format!("\x1b[{}m[teva]\x1b[0m", Color::Blue)
    }
}
