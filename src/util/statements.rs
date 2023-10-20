use super::input::InputBuffer;

#[derive(Debug)]
pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

const STATEMENT_INSERT: &str = "insert";
const STATEMENT_SELECT: &str = "select";


pub fn excute_statement(statement: &str) {
    match statement {
        STATEMENT_INSERT => println!("This is where we would do an insert."),
        STATEMENT_SELECT => println!("This is where we would do a select."),
        _ => println!("Unrecognized keyword at start of '{}'.", statement),
    }
}


pub fn prepare_statement(input_buffer: &mut InputBuffer, statement: &mut String) -> PrepareResult {
    if input_buffer.buffer.starts_with(STATEMENT_INSERT) {
        *statement = STATEMENT_INSERT.to_string();
        return PrepareResult::PrepareSuccess;
    }
    if input_buffer.buffer.starts_with(STATEMENT_SELECT) {
        *statement = STATEMENT_SELECT.to_string();
        return PrepareResult::PrepareSuccess;
    }

    
    PrepareResult::PrepareUnrecognizedStatement
}
