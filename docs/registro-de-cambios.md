# Registro de cambios

Lo más reciente arriba. Cada entrada dice qué cambia para quien usa la aplicación y qué queda pendiente.

## v0.1.0 · Entrega 0 · Preparación — 11/11/2026

**Qué se puede hacer**

- Crear un proyecto nuevo (archivo `.sastra`) o abrir uno existente.
- Añadir productos con código, nombre, temporada, colección y tipo de prenda; editarlos, cambiar su estado
  (borrador, en desarrollo, en muestras, aprobado, archivado), buscarlos y borrarlos.
- Guardar una copia limpia del proyecto para compartirla o como copia de seguridad.
- Todo se guarda al momento; no hay botón de guardar.

**Por dentro**

- Formato `.sastra` (SQLite) con migraciones y registro de cambios (antes y después de cada modificación).
- Motor de la ficha técnica con Typst embebido; prueba de rendimiento de 30 páginas superada
  (ver `docs/pruebas/typst-entrega-0.md`).
- Comprobaciones automáticas en cada cambio (formato, revisión de código, pruebas, licencias) e instalador
  de Windows generado en GitHub.

**Pendiente**

- Deshacer y rehacer, abrir con doble clic, proyectos recientes, copias de seguridad automáticas.
