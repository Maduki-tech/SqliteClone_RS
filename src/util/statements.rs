use super::{
    input::InputBuffer,
    table::{self, serialize_row, Table, row_slot},
};

#[derive(Debug)]
pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

#[derive(Debug)]
pub enum StatementType {
    StatementInsert,
    StatementSelect,
}

enum ExecuteResult {
    ExecuteSuccess,
    ExecuteTableFull,
}

pub struct Statement {
    pub statement_type: StatementType,
    pub row_to_insert: super::table::Row,
}

pub fn execute_statement(statement: &Statement, table: Table) -> ExecuteResult {
    if table.num_rows >= table::TABLE_MAX_ROWS {
        return ExecuteResult::ExecuteTableFull;
    }

    match statement.statement_type {
        StatementType::StatementInsert => execute_insert(statement, table),
        StatementType::StatementSelect => execute_select(statement),
    }
}

fn execute_insert(statement: &Statement, table: Table) -> ExecuteResult {
    let row_to_insert = statement.row_to_insert;
    serialize_row(&row_to_insert, &mut row_slot(&mut table, &mut table.num_rows));

    ExecuteResult::ExecuteSuccess
}

fn execute_select(statement: &Statement) -> ExecuteResult {
    println!("This is where we would do a select.");
    ExecuteResult::ExecuteSuccess
}

pub fn prepare_statement(
    input_buffer: &mut InputBuffer,
    statement: &mut Statement,
) -> PrepareResult {
    if input_buffer.buffer.starts_with("insert") {
        *statement = Statement {
            statement_type: StatementType::StatementInsert,
            row_to_insert: Default::default(),
        };

        let args_assigned = std::io::stdin()
            .read_line(&mut input_buffer.buffer)
            .expect("Failed to read line");

        return PrepareResult::PrepareSuccess;
    }
    if input_buffer.buffer.starts_with("select") {
        *statement = Statement {
            statement_type: StatementType::StatementSelect,
            row_to_insert: Default::default(),
        };
        return PrepareResult::PrepareSuccess;
    }

    PrepareResult::PrepareUnrecognizedStatement
}
