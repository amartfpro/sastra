use std::path::PathBuf;

/// Errores del núcleo. Los mensajes están en español porque llegan a la interfaz.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("el archivo no es un proyecto de Sastra: {0}")]
    NotASastraFile(PathBuf),

    #[error("ya existe un archivo en {0}")]
    AlreadyExists(PathBuf),

    #[error(
        "el proyecto se creó con una versión más reciente de Sastra (formato {found}, esta versión entiende hasta el {supported})"
    )]
    NewerFormat { found: i64, supported: i64 },

    #[error("no existe ningún producto con el identificador {0}")]
    ProductNotFound(String),

    #[error("ya existe un producto con el código «{0}»")]
    DuplicateCode(String),

    #[error("dato no válido: {0}")]
    Invalid(String),

    #[error("error de la base de datos: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("error de serialización: {0}")]
    Json(#[from] serde_json::Error),

    #[error("error de archivo: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
