//! Motor de la ficha técnica: compila plantillas Typst a PDF dentro del propio
//! programa, sin herramientas externas.
//!
//! La plantilla lee sus datos de archivos JSON virtuales (uno por sección) y de
//! imágenes virtuales. Cuando cambia un dato solo se sustituye su archivo y se
//! vuelve a compilar: Typst reutiliza en memoria todo lo que no depende de él,
//! que es la base de la vista previa en vivo (especificación, sección 6).

use std::collections::HashMap;
use std::sync::OnceLock;

use typst::diag::{FileError, FileResult, SourceDiagnostic};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};
pub use typst_layout::PagedDocument;

/// Ruta virtual de la plantilla principal.
const MAIN_PATH: &str = "/main.typ";

struct Fonts {
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
}

/// Las fuentes se cargan una sola vez por proceso.
fn fonts() -> &'static Fonts {
    static FONTS: OnceLock<Fonts> = OnceLock::new();
    FONTS.get_or_init(|| {
        let fonts: Vec<Font> = typst_assets::fonts()
            .flat_map(|data| Font::iter(Bytes::new(data)))
            .collect();
        Fonts {
            book: LazyHash::new(FontBook::from_fonts(&fonts)),
            fonts,
        }
    })
}

fn file_id(path: &str) -> FileId {
    let vpath = VirtualPath::new(path).expect("ruta virtual válida");
    RootedPath::new(VirtualRoot::Project, vpath).intern()
}

/// Un documento en preparación: la plantilla y sus archivos de datos e imágenes.
pub struct Ficha {
    library: LazyHash<Library>,
    main: Source,
    files: HashMap<FileId, Bytes>,
}

impl Ficha {
    pub fn new(template: &str) -> Self {
        Self {
            library: LazyHash::new(Library::default()),
            main: Source::new(file_id(MAIN_PATH), template.to_string()),
            files: HashMap::new(),
        }
    }

    /// Añade o sustituye un archivo virtual (`/datos/medidas.json`, `/img/delantero.svg`…).
    pub fn set_file(&mut self, path: &str, data: impl Into<Vec<u8>>) {
        self.files.insert(file_id(path), Bytes::new(data.into()));
    }

    /// Añade o sustituye un archivo JSON a partir de un valor.
    pub fn set_json(&mut self, path: &str, value: &serde_json::Value) {
        self.set_file(path, serde_json::to_vec(value).expect("JSON serializable"));
    }

    /// Compila el documento paginado.
    pub fn compile(&self) -> Result<PagedDocument, String> {
        let result = typst::compile::<PagedDocument>(self).output;
        // Limita la memoria de la caché incremental sin perder lo reciente.
        comemo::evict(10);
        result.map_err(|errors| format_errors(&errors))
    }

    /// Exporta a PDF un documento ya compilado.
    pub fn pdf(document: &PagedDocument) -> Result<Vec<u8>, String> {
        typst_pdf::pdf(document, &typst_pdf::PdfOptions::default())
            .map_err(|errors| format_errors(&errors))
    }
}

fn format_errors(errors: &[SourceDiagnostic]) -> String {
    errors
        .iter()
        .map(|e| e.message.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

impl World for Ficha {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &fonts().book
    }

    fn main(&self) -> FileId {
        self.main.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main.id() {
            Ok(self.main.clone())
        } else {
            Err(FileError::NotSource)
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if id == self.main.id() {
            return Ok(Bytes::from_string(self.main.clone()));
        }
        self.files
            .get(&id)
            .cloned()
            .ok_or_else(|| FileError::NotFound(id.vpath().get_without_slash().into()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        fonts().fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compila_un_documento_con_datos_json() {
        let mut ficha = Ficha::new(
            r#"#let d = json("/datos/producto.json")
= Ficha técnica de #d.nombre
Código: #d.codigo"#,
        );
        ficha.set_json(
            "/datos/producto.json",
            &serde_json::json!({ "nombre": "Camiseta básica", "codigo": "SS27-TS-001" }),
        );
        let doc = ficha.compile().expect("compila");
        assert_eq!(doc.pages().len(), 1);
        let pdf = Ficha::pdf(&doc).expect("exporta");
        assert!(pdf.starts_with(b"%PDF"));
    }

    #[test]
    fn un_archivo_que_falta_da_un_error_legible() {
        let ficha = Ficha::new(r#"#json("/datos/no-existe.json")"#);
        let err = ficha.compile().unwrap_err();
        assert!(
            err.contains("not found") || err.contains("no-existe"),
            "{err}"
        );
    }
}
