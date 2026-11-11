# Sastra · Guía del proyecto

Léela entera al empezar cada sesión. Al terminar, actualiza «Estado actual» y `docs/registro-de-cambios.md`.

## Qué es

Sastra es una aplicación de escritorio para diseñar prendas de principio a fin: ficha técnica para fábrica,
patronaje 2D con tallaje y, más adelante, una vista 3D. Todo sale de un único modelo de datos de la prenda,
de modo que la ficha técnica se regenera sola cuando cambia algo.

- **Especificación técnica:** `docs/especificacion.md` (requisitos con identificadores RF-xx y RNF-xx,
  modelo de datos, arquitectura, tecnología, plan de entregas).
- **Especificación completa** (incluye organización del trabajo y riesgos):
  https://claude.ai/artifact/8RXPt71eCv7P4PbvsmtomU (documento de Claude Docs del propietario).
- **Decisiones técnicas:** `docs/adr/`.

## Con quién trabajas

El propietario (usuario de GitHub `amartfpro`) es diseñador de moda, no programador. Él decide qué se
construye, aporta fichas, patrones y archivos reales, instala cada versión y la prueba con prendas reales.

- Háblale siempre en español, sin jerga técnica; si hace falta un término, explícalo en una frase.
- Usa Windows. El instalador que prueba sale de GitHub Actions (ver «Publicar una versión»).
- Antes de una decisión de producto con varias opciones razonables, pregúntale.

## Reglas para commits, ramas y textos públicos

- Autor de los commits: `amartfpro <200755886+amartfpro@users.noreply.github.com>`
  (configúralo en el clon con `git config user.name` / `user.email` si no lo está).
- Los mensajes de commit no llevan líneas `Co-Authored-By`, enlaces de sesión ni menciones a asistentes
  de IA. Tampoco el código, los comentarios, la documentación pública ni las notas de versión.
  Es una petición expresa del propietario.
- Mensajes de commit en español, en imperativo y concretos («Añade la tabla de medidas con tolerancias»).
- Se trabaja directamente en `main` salvo que el propietario pida otra cosa. El repositorio es público.

## Estructura

```
Cargo.toml                 # workspace de Rust
crates/sastra-core/        # modelo de datos y formato .sastra (SQLite) — sin dependencias de interfaz
crates/sastra-report/      # ficha técnica con Typst embebido (plantillas en templates/)
apps/desktop/              # aplicación Tauri 2: interfaz React + TypeScript
apps/desktop/src-tauri/    # puente entre interfaz y núcleo (órdenes en src/commands.rs)
docs/                      # especificación, ADR, registro de cambios, resultados de pruebas
samples/                   # archivos reales de prueba aportados por el propietario
.github/workflows/         # comprobaciones (ci.yml) y publicación de versiones (publicar.yml)
```

Más adelante (ver especificación, sección 9): `crates/geometry` (entrega 2), `crates/render` y
`crates/sim` (entrega 3), `crates/io` (importación PSD/AI/SVG, DXF, Excel).

## Comandos

```bash
# Todo el núcleo
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# Interfaz (desde apps/desktop)
pnpm install
pnpm typecheck && pnpm test && pnpm build
pnpm tauri dev                    # abre la aplicación en modo desarrollo

# Licencias (cargo-deny)
cargo deny check licenses bans sources

# Prueba de rendimiento de la ficha (escribe target/ficha-prueba.pdf)
cargo run -p sastra-report --example bench_ficha --profile bench-rapido
```

En Linux, compilar Tauri necesita `libwebkit2gtk-4.1-dev` y `librsvg2-dev`.

## Reglas técnicas

- **El núcleo manda.** Todo cálculo y todo acceso a archivos va en Rust; la interfaz solo dibuja y envía
  órdenes. Ninguna lógica de negocio solo en TypeScript (la validación de la interfaz es una ayuda; el núcleo
  vuelve a validar).
- **Unidades:** geometría y medidas en milímetros, `f64`. La interfaz muestra cm.
- **Formato `.sastra`:** SQLite con `application_id` propio. Para cambiar el esquema se añade una migración al
  final de `MIGRATIONS` en `crates/sastra-core/src/schema.rs`; nunca se edita una ya publicada.
- **Registro de cambios del modelo:** toda modificación pasa por una transacción que anota el antes y el
  después en la tabla `changes` (base del deshacer, las versiones de la ficha y la sincronización).
- **Licencias:** ninguna dependencia GPL ni AGPL (`deny.toml`). Excluidos también: código de MakeHuman/MPFB2,
  simulador de GarmentCode, SMPL-X, CGAL, Triangle, MuPDF. Pantone no se incluye; los símbolos GINETEX
  requieren autorización.
- **Pruebas:** cada función nueva del núcleo con su prueba en `crates/*/tests/`. Nombres de prueba en español
  describiendo el comportamiento. Las pruebas de la interfaz con Vitest junto al archivo (`*.test.ts`).
- **Textos de la interfaz** en español de España. Los errores del núcleo también, porque llegan a la pantalla.
- **Rendimiento:** la vista previa de la ficha debe actualizarse en menos de 200 ms tras cambiar un dato
  (ver `docs/pruebas/typst-entrega-0.md`). Cada sección de la ficha lee su propio JSON.

## Cómo es una sesión

1. Lee esta guía, `docs/registro-de-cambios.md` y la parte de la especificación que toque.
2. Confirma con el propietario qué entrega o qué fallo toca, si no lo ha dicho.
3. Programa con pruebas. Antes de subir: `fmt`, `clippy`, `cargo test`, `pnpm typecheck`, `pnpm test`.
4. Sube a `main`. Si la entrega está lista para probar, publica una versión.
5. Explica al propietario, en lenguaje llano, qué ha cambiado y qué debe probar.
6. Actualiza «Estado actual» y el registro de cambios.

## Publicar una versión

1. Sube la versión en `apps/desktop/package.json` (Tauri la lee de ahí) y en `[workspace.package]` del
   `Cargo.toml` raíz.
2. `git tag vX.Y.Z && git push origin vX.Y.Z`.
3. El flujo `publicar.yml` compila el instalador de Windows y lo adjunta a la página Releases del repositorio.
   El instalador no está firmado: Windows avisa y hay que pulsar «Más información» → «Ejecutar de todas formas».

Cada cambio en `main` ejecuta además `ci.yml`, que deja el instalador como artefacto descargable.

## Estado actual

- **Entrega 0 · Preparación: hecha (v0.1.0).** Proyecto `.sastra` (crear, abrir, guardar copia), productos
  (crear, editar, cambiar estado, borrar), registro de cambios, motor Typst con prueba de 30 páginas,
  compilación automática e instalador de Windows.
- **Siguiente: entrega 1.1 · Productos y tablas.** Plantillas por tipo de prenda; biblioteca de materiales,
  fornituras, colores (importación ASE) y proveedores; BOM con color por variante; POM con tolerancias y
  reglas de escalado; exportación a Excel. Requisitos RF-PR-01 a 04, RF-BI-01 a 05, RF-FT-03 a 05, RF-EX-02.
- **Pendiente de aportar por el propietario:** 3 fichas técnicas reales (básica, media, compleja) en
  `samples/fichas/` como referencia de formato; tiempo que tarda hoy en hacer una ficha.

### Pendientes técnicos conocidos

- Deshacer y rehacer (el registro de cambios ya guarda lo necesario).
- Abrir un `.sastra` con doble clic (asociación de archivos) y lista de proyectos recientes.
- Copia de seguridad automática periódica del proyecto (RNF-07).
- Fuentes de la ficha: ahora solo las incluidas en Typst (Libertinus, New Computer Modern, DejaVu Sans Mono);
  hará falta una sans serif con licencia OFL y fuentes CJK para fábricas en China.
