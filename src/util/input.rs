use std::io::Write;

#[derive(Debug)]
pub struct InputBuffer {
    pub buffer: String,
    pub buffer_length: usize,
    pub input_length: usize,
}

pub fn new_input_buffer() -> InputBuffer {
    InputBuffer {
        buffer: String::new(),
        buffer_length: 0,
        input_length: 0,
    }
}

pub fn read_input(input_buffer: &mut InputBuffer) -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading input");
    input_buffer.buffer = buffer.trim().to_string();
    input_buffer.buffer_length = input_buffer.buffer.len();
    input_buffer.input_length = input_buffer.buffer.len();

    return input_buffer.buffer.clone();
}

pub fn close_input_buffer(input_buffer: &mut InputBuffer) {
    input_buffer.buffer = String::new();
    input_buffer.buffer_length = 0;
    input_buffer.input_length = 0;
}

pub fn print_prompt() {
    print!("db > ");
    std::io::stdout().flush().unwrap();
}
