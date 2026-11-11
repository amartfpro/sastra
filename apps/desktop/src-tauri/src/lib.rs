//! Puente entre la interfaz (React) y el núcleo (`sastra-core`).
//!
//! La interfaz solo envía órdenes y recibe datos; todo el cálculo y el acceso
//! al archivo ocurren aquí (especificación, sección 6).

mod commands;

use std::sync::Mutex;

use sastra_core::Project;

/// Proyecto abierto en la ventana. `None` mientras se muestra la pantalla de inicio.
#[derive(Default)]
pub struct AppState {
    pub project: Mutex<Option<Project>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::app_info,
            commands::new_project,
            commands::open_project,
            commands::close_project,
            commands::project_info,
            commands::list_products,
            commands::create_product,
            commands::update_product,
            commands::delete_product,
            commands::recent_changes,
            commands::save_copy,
        ])
        .run(tauri::generate_context!())
        .expect("no se pudo iniciar Sastra");
}
