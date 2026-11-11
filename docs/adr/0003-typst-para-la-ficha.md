# ADR 0003 · Typst embebido para la ficha técnica

**Estado:** aceptada · 11/11/2026

## Decisión

La ficha técnica se genera con Typst 0.15 como librería dentro del núcleo (`crates/sastra-report`), con un
mundo virtual propio: la plantilla y sus datos son archivos en memoria, uno por sección (`/datos/*.json`,
`/img/*`). Al cambiar un dato solo se sustituye su archivo y se recompila; Typst reutiliza el resto gracias a
su caché incremental.

## Motivo

La prueba de la entrega 0 (`docs/pruebas/typst-entrega-0.md`) recompila una ficha de 30 páginas en unos 40 ms
tras cambiar una celda. Typst además genera PDF/A y PDF etiquetado, admite tintas directas e incrusta PDF y SVG
como imágenes, útil para el arte de Illustrator.

## Consecuencias

- Las plantillas de la ficha se escriben en lenguaje Typst (`templates/`).
- Hay que incluir fuentes propias con licencia OFL (sans serif y CJK).
- La caché se limita con `comemo::evict(10)` tras cada compilación.
