// Evita que se abra una ventana de consola en Windows en la versión final. NO QUITAR.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    sastra_desktop_lib::run()
}
