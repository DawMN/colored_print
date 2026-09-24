use colored_print::{Color, ColoredText, Style};
use colored_print::{println_colored}; // Import macro

fn main() {
    // Method 1: Using ColoredText directly
    let text = ColoredText::new("Hello, World!")
        .fg(Color::Green)
        .style(Style::Bold);
    println!("{}", text);

    // Method 2: Using the macro
    println_colored!("This is red text", fg: Color::Red);
    println_colored!("Green on blue", fg: Color::Green, bg: Color::Blue);
    
    // With styles
    println_colored!(
        "Bold yellow text",
        fg: Color::Yellow,
        styles: [Style::Bold]
    );
    
    // Multiple styles
    println_colored!(
        "Underlined cyan on magenta",
        fg: Color::Cyan,
        bg: Color::Magenta,
        styles: [Style::Underline, Style::Bold]
    );
}