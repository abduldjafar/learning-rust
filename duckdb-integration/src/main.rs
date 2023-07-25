use duckdb::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute_batch(
        r"INSTALL json;
        LOAD json;
        CREATE TABLE asoi as (
            select * from read_ndjson('data.json')
        );
         ")?;

    Ok(())
}