use cli_table::{print_stdout, Table, TableStruct};
use anyhow::Result;
use std::io::Write;

pub fn print_key_value_table(data: &[(String, String)]) -> Result<()> {
    let mut table_data = Vec::new();
    for (key, value) in data {
        table_data.push(vec![key.clone(), value.clone()]);
    }

    let table: TableStruct = table_data.table();

    print_stdout(table)?;
    std::io::stdout().flush()?;
    Ok(())
}
