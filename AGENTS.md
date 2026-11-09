# rust

Workspace de Cargo con ejemplos de aprendizaje de Rust: un "hola mundo" y un ejemplo de SQLite.

## Visibilidad y rama

- Público. Rama por defecto: `main`.

## Stack

- Rust, edición 2021, workspace de Cargo (`resolver = "2"`).
- Dependencia clave: `rusqlite = "0.29"` (crate `sqlite`, con SQLite embebido).

## Estructura

- `Cargo.toml`: workspace con los miembros `sqlite` y `hola_mundo`.
- `hola_mundo/src/main.rs`: imprime "¡Hola, mundo!".
- `sqlite/Cargo.toml` y `sqlite/src/main.rs`: crea `personas.db` en el directorio actual, inserta dos filas y lista personas (y las filtradas por edad 25).
- `README.md`: solo indica `cargo build`.
- `.gitignore`: ignora `/target/`, `/Cargo.lock`, `/Cargo.bak` y `*.db`.

## Comandos

- Compilar todo: `cargo build` (según README).
- Ejecutar un miembro: `cargo run -p hola_mundo` o `cargo run -p sqlite` (estándar de Cargo; no aparece en el repo).
- Tests: no hay tests definidos (no verificado con `cargo test`).

## Configuración

- No hay variables de entorno ni ficheros de configuración.
- `sqlite` escribe `personas.db` en el directorio de ejecución: ejecutarlo desde la raíz genera el fichero en el repo (ya ignorado por `.gitignore`).

## Convenciones para agentes

- Código de ejemplo didáctico; mantener el estilo simple de cada `main.rs`.
- Los textos de salida y comentarios están en español; mantener el idioma.
- No versionar `*.db` ni `Cargo.lock` (ya ignorados).

## Estado

- Repo pequeño y poco mantenido; no hay CI ni tests.
- `sqlite/src/main.rs`: la tabla `persona` no tiene restricción `UNIQUE`, así que `INSERT OR IGNORE` no ignora nada y cada ejecución duplica "Ricardo" y "Ana".
- `sqlite/src/main.rs` línea 26: indentación incorrecta (no afecta a la compilación).
- `sqlite/Cargo.toml`: el comentario "o la versión más reciente" indica que la versión 0.29 está fijada a mano; no verificado si está actualizada.
