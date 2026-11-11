# ADR 0001 · Núcleo nativo en Rust con interfaz Tauri 2

**Estado:** aceptada · 11/11/2026

## Contexto

Sastra necesita formularios y tablas abundantes (ficha técnica), un editor 2D de patrones y, más adelante,
render y simulación 3D en tiempo real en Windows (primero) y en macOS y Linux (después).

La investigación técnica (noviembre de 2026) encontró que el webview del sistema no ofrece WebGPU en Linux
(WebKitGTK) ni previsiblemente en macOS 13–15, que WebGL en WebKitGTK no es fiable, y que copiar mallas grandes
del núcleo al webview en cada fotograma es demasiado lento con el IPC de Tauri.

## Decisión

- Interfaz en React + TypeScript dentro de Tauri 2: formularios, tablas, vista previa de la ficha.
- Núcleo en Rust nativo (no WASM): modelo de datos, geometría, Typst, importación y exportación.
- El render 2D/3D y la simulación vivirán en el núcleo sobre wgpu, dibujando bajo un webview transparente
  (entrega 3, precedida de una prueba). Plan B: Electron con el mismo núcleo.
- La interfaz y el núcleo solo intercambian órdenes y datos de tablas; nunca mallas.

## Consecuencias

- Dos lenguajes (Rust y TypeScript), con la lógica siempre en Rust.
- Linux será plataforma de segunda fila al principio (3D en ventana aparte).
- Instaladores pequeños (decenas de MB) frente a Electron.
