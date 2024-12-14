use rusqlite::Connection;

use std::io;


fn main() -> io::Result<()> {
    let conn = Connection::open_in_memory()?;


    conn.close()
    Ok(())
}
