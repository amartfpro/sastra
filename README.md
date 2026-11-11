# Sastra

Aplicación de escritorio para diseñar prendas y preparar su documentación para fábrica: ficha técnica,
patronaje con tallaje y, más adelante, vista 3D. Todo sale de un único modelo de datos de la prenda, así que
cuando cambia una medida o un material la ficha técnica se actualiza sola.

**Estado:** en desarrollo temprano (versión 0.1). Ya permite crear proyectos y gestionar productos; la ficha
técnica completa llega en las próximas versiones.

## Descargar

Instalador para Windows 10 y 11 en la página [Releases](https://github.com/amartfpro/sastra/releases).
El instalador todavía no está firmado: si Windows muestra un aviso, pulsa «Más información» y luego
«Ejecutar de todas formas».

## Plan

| Entrega | Contenido | Estado |
| --- | --- | --- |
| 0 | Base del proyecto, formato `.sastra`, instalador | Hecha |
| 1.1 – 1.4 | Ficha técnica: materiales, medidas, planos, gráficos, etiquetas, packaging, calidad, PDF, versiones | En curso |
| 2 | Patronaje 2D paramétrico, tallaje, DXF-AAMA, consumo de tejido | Pendiente |
| 3 | Vista 3D: avatar desde medidas y simulación de prendas sencillas | Pendiente |

Detalle en [docs/especificacion.md](docs/especificacion.md).

## Tecnología

Núcleo en Rust · interfaz React + TypeScript en Tauri 2 · proyecto en un único archivo SQLite ·
ficha técnica con Typst · todo con dependencias de código abierto y licencias permisivas.

## Desarrollo

```bash
# Requisitos: Rust estable, Node 22 y pnpm 10
cargo test --workspace
cd apps/desktop
pnpm install
pnpm tauri dev
```

## Licencia

Todos los derechos reservados mientras el proyecto está en desarrollo. La licencia de código abierto se
decidirá antes de la primera versión estable.
