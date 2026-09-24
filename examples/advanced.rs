use colored_print::{Color, ColoredText, Style};

fn main() {
    // Create reusable styled text
    let error = ColoredText::new("ERROR")
        .fg(Color::BrightRed)
        .style(Style::Bold);
    
    let warning = ColoredText::new("WARNING")
        .fg(Color::Yellow)
        .style(Style::Bold);
    
    let info = ColoredText::new("INFO")
        .fg(Color::Cyan)
        .style(Style::Bold);

    // Use in different contexts
    println!("{}: Something went wrong!", error);
    println!("{}: This might be an issue", warning);
    println!("{}: Process completed", info);
    
    // Print without newline - use crate:: prefix
    colored_print::print_colored!("Loading", fg: Color::Blue, styles: [Style::Bold]);
    print!(".");
    colored_print::print_colored!(" Done!", fg: Color::Green, styles: [Style::Bold]);
    println!();
    
    // Combine multiple colored texts
    let red = ColoredText::new("RED").fg(Color::Red);
    let white = ColoredText::new("WHITE").fg(Color::White);
    let blue = ColoredText::new("BLUE").fg(Color::Blue);
    println!("{} {} {}", red, white, blue);
}