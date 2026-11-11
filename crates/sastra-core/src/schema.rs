//! Esquema del archivo `.sastra` y sus migraciones.
//!
//! Reglas para cambiar el esquema:
//! - Nunca se edita una migración ya publicada; se añade una nueva al final de `MIGRATIONS`.
//! - `FORMAT_VERSION` es siempre el número de migraciones.
//! - Todas las medidas se guardan en milímetros (ver especificación, sección 5).

use rusqlite::Connection;

use crate::{Error, Result};

/// Extensión de los archivos de proyecto.
pub const FILE_EXTENSION: &str = "sastra";

/// Identificador de aplicación de SQLite: «SAST» en ASCII.
/// Permite reconocer un `.sastra` aunque cambie de extensión.
pub const APPLICATION_ID: i32 = 0x5341_5354;

/// Versión del formato que escribe y entiende esta versión de Sastra.
pub const FORMAT_VERSION: i64 = MIGRATIONS.len() as i64;

const MIGRATIONS: &[&str] = &[
    // 1 · Metadatos, productos y registro de cambios.
    r#"
    CREATE TABLE meta (
        key   TEXT PRIMARY KEY,
        value TEXT NOT NULL
    ) STRICT;

    CREATE TABLE products (
        id           TEXT PRIMARY KEY,
        code         TEXT NOT NULL UNIQUE COLLATE NOCASE,
        name         TEXT NOT NULL,
        season       TEXT NOT NULL DEFAULT '',
        collection   TEXT NOT NULL DEFAULT '',
        garment_type TEXT NOT NULL DEFAULT '',
        status       TEXT NOT NULL DEFAULT 'draft',
        created_at   INTEGER NOT NULL,
        updated_at   INTEGER NOT NULL
    ) STRICT;

    CREATE TABLE changes (
        seq       INTEGER PRIMARY KEY AUTOINCREMENT,
        at        INTEGER NOT NULL,
        kind      TEXT NOT NULL,
        entity    TEXT NOT NULL,
        entity_id TEXT NOT NULL,
        before    TEXT,
        after     TEXT
    ) STRICT;

    CREATE INDEX changes_entity ON changes (entity, entity_id);
    "#,
];

/// Prepara una conexión recién abierta: claves foráneas, journal y migraciones.
pub(crate) fn configure(conn: &Connection) -> Result<()> {
    conn.pragma_update(None, "foreign_keys", true)?;
    // DELETE en vez de WAL: el proyecto es un único archivo, sin archivos -wal
    // ni -shm a su lado, más seguro dentro de carpetas sincronizadas.
    conn.pragma_update(None, "journal_mode", "DELETE")?;
    conn.pragma_update(None, "synchronous", "FULL")?;
    Ok(())
}

/// Aplica las migraciones pendientes dentro de una transacción.
pub(crate) fn migrate(conn: &mut Connection) -> Result<()> {
    let current: i64 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if current > FORMAT_VERSION {
        return Err(Error::NewerFormat {
            found: current,
            supported: FORMAT_VERSION,
        });
    }
    if current == FORMAT_VERSION {
        return Ok(());
    }
    let tx = conn.transaction()?;
    for sql in &MIGRATIONS[current as usize..] {
        tx.execute_batch(sql)?;
    }
    tx.pragma_update(None, "user_version", FORMAT_VERSION)?;
    tx.commit()?;
    Ok(())
}

pub(crate) fn application_id(conn: &Connection) -> Result<i32> {
    Ok(conn.pragma_query_value(None, "application_id", |row| row.get(0))?)
}
