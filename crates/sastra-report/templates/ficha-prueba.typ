// Ficha técnica de prueba para medir el rendimiento de Typst (entrega 0).
// Cada sección lee su propio JSON para que un cambio en una tabla no invalide
// las demás. No es todavía la plantilla definitiva de la ficha.

#let meta = json("/datos/producto.json")
#let bom = json("/datos/materiales.json")
#let pom = json("/datos/medidas.json")
#let ops = json("/datos/construccion.json")
#let arts = json("/datos/graficos.json")
#let etiquetas = json("/datos/etiquetas.json")
#let calidad = json("/datos/calidad.json")

#set document(title: meta.codigo + " · " + meta.nombre, author: meta.marca)
#set text(font: "Libertinus Serif", size: 8.5pt, lang: "es")
#set table(stroke: 0.4pt + luma(170), inset: (x: 4pt, y: 3pt))
#show table.cell.where(y: 0): set text(weight: "bold")
#show heading.where(level: 1): it => {
  set text(size: 13pt)
  block(below: 8pt, it)
}

#set page(
  paper: "a4",
  flipped: true,
  margin: (x: 14mm, top: 20mm, bottom: 15mm),
  header: [
    #set text(size: 8pt)
    #grid(
      columns: (1fr, auto),
      [*#meta.codigo* · #meta.nombre · #meta.temporada],
      [Versión #meta.version · #meta.fecha],
    )
    #v(-4pt)
    #line(length: 100%, stroke: 0.4pt)
  ],
  footer: context [
    #set text(size: 8pt)
    #grid(
      columns: (1fr, auto),
      [#meta.marca · Ficha técnica],
      [Página #counter(page).display() de #counter(page).final().first()],
    )
  ],
)

#let seccion(titulo) = {
  pagebreak(weak: true)
  heading(level: 1, titulo)
}

// Portada
= Ficha técnica
#grid(
  columns: (1fr, 1.4fr),
  gutter: 10mm,
  table(
    columns: (auto, 1fr),
    ..meta.campos.map(c => (c.at(0), c.at(1))).flatten(),
  ),
  grid(
    columns: 2,
    gutter: 6mm,
    image("/img/plano-delantero.svg", height: 110mm),
    image("/img/plano-espalda.svg", height: 110mm),
  ),
)

#seccion[Lista de materiales]
#table(
  columns: (auto, 2fr, 1.4fr, 1.2fr, auto, 1.6fr) + (auto,) * meta.variantes.len() + (auto, auto),
  table.header(
    [Nº], [Material], [Ubicación], [Proveedor], [Ref.], [Composición],
    ..meta.variantes.map(v => [#v]),
    [Consumo], [Ud.],
  ),
  ..bom
    .enumerate()
    .map(((i, r)) => (
      str(i + 1),
      r.material,
      r.ubicacion,
      r.proveedor,
      r.referencia,
      r.composicion,
      ..r.colores,
      str(r.consumo),
      r.unidad,
    ))
    .flatten(),
)

#seccion[Tabla de medidas (cm)]
#table(
  columns: (auto, 2.6fr, 2fr, auto) + (auto,) * pom.tallas.len(),
  align: (x, y) => if x >= 3 { right } else { left },
  table.header([Cód.], [Punto de medida], [Cómo se mide], [Tol. ±], ..pom.tallas.map(t => [#t])),
  ..pom
    .puntos
    .map(p => (p.codigo, p.descripcion, p.metodo, str(p.tolerancia), ..p.valores.map(v => str(v))))
    .flatten(),
)

#seccion[Construcción]
#table(
  columns: (auto, 2.2fr, auto, auto, auto, 1fr, 2fr),
  table.header([Nº], [Operación], [Costura (ISO 4916)], [Puntada (ISO 4915)], [PPP], [Hilo], [Notas]),
  ..ops
    .enumerate()
    .map(((i, o)) => (str(i + 1), o.operacion, o.costura, o.puntada, str(o.ppp), o.hilo, o.notas))
    .flatten(),
)

#for a in arts {
  seccion[Gráfico · #a.nombre]
  grid(
    columns: (1.3fr, 1fr),
    gutter: 8mm,
    image(a.archivo, width: 100%, height: 125mm, fit: "contain"),
    [
      #table(
        columns: (auto, 1fr),
        [Técnica], a.tecnica,
        [Tamaño], a.tamano,
        [Posición], a.posicion,
        [Colores], a.colores.join(", "),
        [Escalado], a.escalado,
      )
      #v(4mm)
      #image("/img/plano-delantero.svg", height: 70mm)
    ],
  )
}

#seccion[Etiquetas y packaging]
#table(
  columns: (auto, 1.5fr, 2.5fr, 1.5fr),
  table.header([Tipo], [Material], [Contenido], [Posición]),
  ..etiquetas.map(e => (e.tipo, e.material, e.contenido, e.posicion)).flatten(),
)

#seccion[Puntos de control del sample]
#table(
  columns: (auto, 3fr, auto, 1.2fr),
  table.header([Nº], [Comprobación], [Severidad], [Referencia]),
  ..calidad.enumerate().map(((i, c)) => (str(i + 1), c.comprobacion, c.severidad, c.referencia)).flatten(),
)
