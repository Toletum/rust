use rusqlite::{Connection, Result, params};

#[derive(Debug)]
struct Persona {
    id: i32,
    nombre: String,
    edad: Option<i32>,
}

fn main() -> Result<()> {
    let conn = Connection::open("personas.db")?; // Abre o crea el archivo personas.db

    conn.execute(
        "CREATE TABLE IF NOT EXISTS persona (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre TEXT NOT NULL,
            edad INTEGER
        )",
        (),
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO persona (nombre, edad) VALUES (?1, ?2)",
        params!["Ricardo", 30],
    )?;
        conn.execute(
        "INSERT OR IGNORE INTO persona (nombre, edad) VALUES (?1, ?2)",
        params!["Ana", 25],
    )?;

    let mut stmt = conn.prepare("SELECT id, nombre, edad FROM persona")?;
    let persona_iter = stmt.query_map([], |row| {
        Ok(Persona {
            id: row.get(0)?,
            nombre: row.get(1)?,
            edad: row.get(2)?,
        })
    })?;

    for persona in persona_iter {
        println!("Persona: {:?}", persona?);
    }

    let mut stmt_con_parametros = conn.prepare("SELECT id, nombre, edad FROM persona where edad = ?1")?;
    let persona_iter_con_parametros = stmt_con_parametros.query_map(params![25], |row|{
        Ok(Persona{
            id: row.get(0)?,
            nombre: row.get(1)?,
            edad: row.get(2)?,
        })
    })?;

    println!("Personas con edad 25:");
    for persona_con_parametros in persona_iter_con_parametros{
        println!("Persona: {:?}", persona_con_parametros?);
    }

    Ok(())
}

