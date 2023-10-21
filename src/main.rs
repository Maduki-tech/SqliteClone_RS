use util::{
    commands::{handle_command, MetaCommandResult},
    input::{new_input_buffer, print_prompt, read_input},
    statements::{excute_statement, prepare_statement, Statement},
};

mod util;

fn main() -> ! {
    let mut input_buffer = new_input_buffer();

    loop {
        print_prompt();
        read_input(&mut input_buffer);
        let first_char = input_buffer.buffer.chars().next().unwrap();

        if first_char == '.' {
            match handle_command(&input_buffer.buffer) {
                MetaCommandResult::MetaCommandSuccess => continue,
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command '{}'.", input_buffer.buffer);
                    continue;
                }
            }
        }

        let mut statement = Statement {
            statement_type: Default::default(),
            row_to_insert: Default::default(),
        };
        prepare_statement(&mut input_buffer, &mut statement);

        excute_statement(&statement);
    }
}
