//! Núcleo de Sastra: modelo de datos de la prenda y formato de proyecto `.sastra`.
//!
//! Un proyecto `.sastra` es una base de datos SQLite con un identificador de
//! aplicación propio. Cada cambio en el modelo se guarda en el momento (no hay
//! botón de guardar) y queda anotado en el registro de cambios, que es la base
//! del deshacer, de las versiones de la ficha y de la futura sincronización.

mod changes;
mod error;
mod product;
mod project;
mod schema;

pub use changes::{Change, ChangeKind};
pub use error::{Error, Result};
pub use product::{NewProduct, Product, ProductPatch, ProductStatus};
pub use project::Project;
pub use schema::{APPLICATION_ID, FILE_EXTENSION, FORMAT_VERSION};
