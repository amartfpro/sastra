use std::path::{Path, PathBuf};

use rusqlite::{Connection, OpenFlags, OptionalExtension, params};

use crate::changes::now_millis;
use crate::schema::{self, APPLICATION_ID};
use crate::{Error, Result};

/// Un proyecto `.sastra` abierto. Los cambios se escriben en disco al momento.
pub struct Project {
    pub(crate) conn: Connection,
    path: PathBuf,
}

impl std::fmt::Debug for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Project").field("path", &self.path).finish()
    }
}

impl Project {
    /// Crea un proyecto nuevo. Falla si ya existe un archivo en esa ruta.
    pub fn create(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if path.exists() {
            return Err(Error::AlreadyExists(path));
        }
        let mut conn = Connection::open(&path)?;
        conn.pragma_update(None, "application_id", APPLICATION_ID)?;
        schema::configure(&conn)?;
        schema::migrate(&mut conn)?;
        let now = now_millis().to_string();
        conn.execute(
            "INSERT INTO meta (key, value) VALUES ('created_at', ?1), ('created_with', ?2)",
            params![now, env!("CARGO_PKG_VERSION")],
        )?;
        Ok(Self { conn, path })
    }

    /// Abre un proyecto existente y lo actualiza al formato actual si hace falta.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if !path.is_file() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("no se encuentra {}", path.display()),
            )));
        }
        let mut conn = Connection::open_with_flags(
            &path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        // Un archivo que no es SQLite falla al leer la cabecera: también es «no es un proyecto».
        match schema::application_id(&conn) {
            Ok(APPLICATION_ID) => {}
            _ => return Err(Error::NotASastraFile(path)),
        }
        schema::configure(&conn)?;
        schema::migrate(&mut conn)?;
        Ok(Self { conn, path })
    }

    /// Ruta del archivo del proyecto.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Lee un metadato del proyecto (`created_at`, `created_with`…).
    pub fn meta(&self, key: &str) -> Result<Option<String>> {
        Ok(self
            .conn
            .query_row("SELECT value FROM meta WHERE key = ?1", [key], |row| {
                row.get(0)
            })
            .optional()?)
    }

    /// Guarda una copia limpia y compacta del proyecto en `dest`, que no debe existir.
    /// Sirve como copia de seguridad o para compartir el archivo.
    pub fn save_copy_to(&self, dest: impl AsRef<Path>) -> Result<()> {
        let dest = dest.as_ref();
        if dest.exists() {
            return Err(Error::AlreadyExists(dest.to_path_buf()));
        }
        let dest_str = dest
            .to_str()
            .ok_or_else(|| Error::Invalid("la ruta de destino no es válida".into()))?;
        self.conn.execute("VACUUM INTO ?1", [dest_str])?;
        Ok(())
    }
}
