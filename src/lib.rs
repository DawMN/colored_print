use std::fmt;

/// ANSI color codes for terminal output
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    fn as_str(&self) -> &'static str {
        match self {
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
            Color::BrightBlack => "90",
            Color::BrightRed => "91",
            Color::BrightGreen => "92",
            Color::BrightYellow => "93",
            Color::BrightBlue => "94",
            Color::BrightMagenta => "95",
            Color::BrightCyan => "96",
            Color::BrightWhite => "97",
        }
    }
}

/// Style options for text
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    Strikethrough,
}

impl Style {
    fn as_str(&self) -> &'static str {
        match self {
            Style::Bold => "1",
            Style::Dim => "2",
            Style::Italic => "3",
            Style::Underline => "4",
            Style::Blink => "5",
            Style::Reverse => "7",
            Style::Hidden => "8",
            Style::Strikethrough => "9",
        }
    }
}

/// A colored and styled text wrapper
pub struct ColoredText {
    text: String,
    foreground: Option<Color>,
    background: Option<Color>,
    styles: Vec<Style>,
}

impl ColoredText {
    /// Create a new colored text instance
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self {
            text: text.into(),
            foreground: None,
            background: None,
            styles: Vec::new(),
        }
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Add a style
    pub fn style(mut self, style: Style) -> Self {
        self.styles.push(style);
        self
    }

    /// Add multiple styles
    pub fn styles(mut self, styles: &[Style]) -> Self {
        self.styles.extend_from_slice(styles);
        self
    }

    /// Generate ANSI escape code string
    fn ansi_code(&self) -> String {
        let mut codes: Vec<String> = Vec::new();

        if let Some(fg) = self.foreground {
            codes.push(fg.as_str().to_string());
        }

        if let Some(bg) = self.background {
            // Convert background color code (add 10 to foreground code)
            let bg_code = bg.as_str().parse::<i32>().unwrap() + 10;
            codes.push(bg_code.to_string());
        }

        for style in &self.styles {
            codes.push(style.as_str().to_string());
        }

        if codes.is_empty() {
            String::new()
        } else {
            format!("\x1b[{}m", codes.join(";"))
        }
    }

    /// Get the formatted string with ANSI codes
    pub fn to_colored_string(&self) -> String {
        let code = self.ansi_code();
        if code.is_empty() {
            self.text.clone()
        } else {
            format!("{}{}\x1b[0m", code, self.text)
        }
    }
}

impl fmt::Display for ColoredText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_colored_string())
    }
}

/// Macro for printing colored text
#[macro_export]
macro_rules! println_colored {
    // With foreground color only
    ($text:expr, fg: $fg:expr) => {
        println!("{}", $crate::ColoredText::new($text).fg($fg));
    };
    
    // With foreground and background
    ($text:expr, fg: $fg:expr, bg: $bg:expr) => {
        println!("{}", $crate::ColoredText::new($text).fg($fg).bg($bg));
    };
    
    // With foreground and styles
    ($text:expr, fg: $fg:expr, styles: [$($style:expr),*]) => {{
        let mut ct = $crate::ColoredText::new($text).fg($fg);
        $(ct = ct.style($style);)*
        println!("{}", ct);
    }};
    
    // With foreground, background, and styles
    ($text:expr, fg: $fg:expr, bg: $bg:expr, styles: [$($style:expr),*]) => {{
        let mut ct = $crate::ColoredText::new($text).fg($fg).bg($bg);
        $(ct = ct.style($style);)*
        println!("{}", ct);
    }};
    
    // Simple usage with just a ColoredText object
    ($ct:expr) => {
        println!("{}", $ct);
    };
}

/// Macro for printing colored text without newline
#[macro_export]
macro_rules! print_colored {
    ($text:expr, fg: $fg:expr) => {
        print!("{}", $crate::ColoredText::new($text).fg($fg));
    };
    
    ($text:expr, fg: $fg:expr, bg: $bg:expr) => {
        print!("{}", $crate::ColoredText::new($text).fg($fg).bg($bg));
    };
    
    ($text:expr, fg: $fg:expr, styles: [$($style:expr),*]) => {{
        let mut ct = $crate::ColoredText::new($text).fg($fg);
        $(ct = ct.style($style);)*
        print!("{}", ct);
    }};
    
    ($text:expr, fg: $fg:expr, bg: $bg:expr, styles: [$($style:expr),*]) => {{
        let mut ct = $crate::ColoredText::new($text).fg($fg).bg($bg);
        $(ct = ct.style($style);)*
        print!("{}", ct);
    }};
    
    ($ct:expr) => {
        print!("{}", $ct);
    };
}