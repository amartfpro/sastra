# Prueba de Typst · ficha de 30 páginas (entrega 0)

**Resultado: superada.** Tras cambiar una celda, la ficha se recompila en unos 35–40 ms, cinco veces por
debajo del límite de 200 ms fijado para la vista previa en vivo.

## Qué se midió

Una ficha de prueba de 30 páginas A4 apaisadas (`crates/sastra-report/templates/ficha-prueba.typ`) con:
portada con dos planos vectoriales, 60 líneas de materiales con 3 variantes de color, 48 puntos de medida en
8 tallas, 90 operaciones de construcción, 18 páginas de gráficos (9 vectoriales y 9 imágenes de
1500 × 1500 px), 36 etiquetas y 70 puntos de control. Cada sección lee su propio archivo JSON.

Equipo: máquina virtual de 2 núcleos Intel Xeon a 2,1 GHz, Typst 0.15.1, perfil `bench-rapido`. Mediana de 7
repeticiones (salvo la compilación en frío). Para repetirla:

```bash
cargo run -p sastra-report --example bench_ficha --profile bench-rapido
```

| Operación | Tiempo |
| --- | ---: |
| Compilación en frío (incluye cargar fuentes) | 228 ms |
| Exportar a PDF por primera vez | 670 ms |
| Recompilar sin cambios | 1 ms |
| Recompilar tras cambiar una medida | 40 ms |
| Recompilar tras cambiar un material | 34 ms |
| Recompilar tras cambiar la cabecera (afecta a las 30 páginas) | 188 ms |
| Cambiar una medida y volver a exportar el PDF completo | 110 ms |

## Conclusiones

- **La arquitectura de la ficha reactiva se confirma:** un compilador Typst vivo dentro del núcleo, con un
  JSON por sección, da vista previa instantánea.
- **Los cambios que afectan a todas las páginas** (cabecera, versión) rozan los 200 ms en esta máquina; en un
  PC normal deberían quedar por debajo. Si hiciera falta, la vista previa puede mostrar primero la página
  visible.
- **El PDF pesa 35 MB** porque las imágenes de prueba son ruido difícil de comprimir. Con gráficos reales será
  mucho menor, pero conviene añadir una opción de exportación con imágenes reducidas para enviar por correo.
- **Fuentes:** la prueba usa las fuentes incluidas en Typst (con serifa). La ficha definitiva necesita una sans
  serif con licencia OFL y fuentes para chino, japonés y coreano.
