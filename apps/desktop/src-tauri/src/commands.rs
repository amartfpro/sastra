//! Órdenes que la interfaz puede invocar. Los errores se devuelven como texto
//! en español, listo para mostrar.

use std::path::PathBuf;

use serde::Serialize;
use tauri::State;

use sastra_core::{Change, FORMAT_VERSION, NewProduct, Product, ProductPatch, Project};

use crate::AppState;

type CmdResult<T> = Result<T, String>;

fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}

fn with_project<T>(
    state: &State<'_, AppState>,
    f: impl FnOnce(&mut Project) -> sastra_core::Result<T>,
) -> CmdResult<T> {
    let mut guard = state.project.lock().map_err(err)?;
    let project = guard
        .as_mut()
        .ok_or_else(|| "no hay ningún proyecto abierto".to_string())?;
    f(project).map_err(err)
}

/// Añade la extensión `.sastra` si falta.
fn with_extension(path: String) -> PathBuf {
    let mut p = PathBuf::from(path);
    let has_ext = p
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case(sastra_core::FILE_EXTENSION));
    if !has_ext {
        let mut name = p.file_name().map(|n| n.to_os_string()).unwrap_or_default();
        name.push(".");
        name.push(sastra_core::FILE_EXTENSION);
        p.set_file_name(name);
    }
    p
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    version: &'static str,
    format_version: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    path: String,
    name: String,
}

fn info_of(project: &Project) -> ProjectInfo {
    let path = project.path();
    ProjectInfo {
        path: path.display().to_string(),
        name: path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default(),
    }
}

#[tauri::command]
pub fn app_info() -> AppInfo {
    AppInfo {
        version: env!("CARGO_PKG_VERSION"),
        format_version: FORMAT_VERSION,
    }
}

#[tauri::command]
pub fn new_project(path: String, state: State<'_, AppState>) -> CmdResult<ProjectInfo> {
    let project = Project::create(with_extension(path)).map_err(err)?;
    let info = info_of(&project);
    *state.project.lock().map_err(err)? = Some(project);
    Ok(info)
}

#[tauri::command]
pub fn open_project(path: String, state: State<'_, AppState>) -> CmdResult<ProjectInfo> {
    let project = Project::open(PathBuf::from(path)).map_err(err)?;
    let info = info_of(&project);
    *state.project.lock().map_err(err)? = Some(project);
    Ok(info)
}

#[tauri::command]
pub fn close_project(state: State<'_, AppState>) -> CmdResult<()> {
    *state.project.lock().map_err(err)? = None;
    Ok(())
}

#[tauri::command]
pub fn project_info(state: State<'_, AppState>) -> CmdResult<Option<ProjectInfo>> {
    Ok(state.project.lock().map_err(err)?.as_ref().map(info_of))
}

#[tauri::command]
pub fn list_products(state: State<'_, AppState>) -> CmdResult<Vec<Product>> {
    with_project(&state, |p| p.list_products())
}

#[tauri::command]
pub fn create_product(input: NewProduct, state: State<'_, AppState>) -> CmdResult<Product> {
    with_project(&state, |p| p.create_product(input))
}

#[tauri::command]
pub fn update_product(
    id: String,
    patch: ProductPatch,
    state: State<'_, AppState>,
) -> CmdResult<Product> {
    with_project(&state, |p| p.update_product(&id, patch))
}

#[tauri::command]
pub fn delete_product(id: String, state: State<'_, AppState>) -> CmdResult<()> {
    with_project(&state, |p| p.delete_product(&id))
}

#[tauri::command]
pub fn recent_changes(limit: usize, state: State<'_, AppState>) -> CmdResult<Vec<Change>> {
    with_project(&state, |p| p.recent_changes(limit.min(500)))
}

#[tauri::command]
pub fn save_copy(path: String, state: State<'_, AppState>) -> CmdResult<()> {
    with_project(&state, |p| p.save_copy_to(with_extension(path)))
}

#[cfg(test)]
mod tests {
    use super::with_extension;

    #[test]
    fn anade_la_extension_si_falta() {
        assert!(with_extension("coleccion".into()).ends_with("coleccion.sastra"));
        assert!(with_extension("coleccion.SASTRA".into()).ends_with("coleccion.SASTRA"));
        assert!(with_extension("v1.2".into()).ends_with("v1.2.sastra"));
    }
}
