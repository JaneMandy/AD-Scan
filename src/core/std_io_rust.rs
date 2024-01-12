

use colored::Colorize;
pub mod std_io_rust {
    use colored::Colorize;

    pub enum MessageType {
        Info,
        Good,
        Warning,
        Error,
    }


    pub fn print_message(message_type: MessageType, format_str: &str, args: Option<std::fmt::Arguments>) {
        let prefix = match message_type {
            MessageType::Info => "[*]".bold().blue(),
            MessageType::Good => "[+]".bold().green(),
            MessageType::Warning => "[!]".bold().yellow(),
            MessageType::Error => "[-]".bold().red(),
        };

        match args {
            Some(args) => {
                println!("{} {}", prefix.color(color_for_message(message_type)), format_args!("{} {}", format_str, args).to_string());
            }
            None => {
                println!("{} {}", prefix.color(color_for_message(message_type)), format_str.bold());
            }
        }
    }

    fn color_for_message(message_type: MessageType) -> colored::Color {
        match message_type {
            MessageType::Info => colored::Color::Blue,
            MessageType::Good => colored::Color::Green,
            MessageType::Warning => colored::Color::Yellow,
            MessageType::Error => colored::Color::Red,
        }
    }
}