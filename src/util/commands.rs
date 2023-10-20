
#[derive(Debug)]
pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

pub fn handle_command(command: &str) -> MetaCommandResult {
    match command {
        ".exit" => std::process::exit(0),
        _ => MetaCommandResult::MetaCommandUnrecognizedCommand,
    }
}
