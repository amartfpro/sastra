# Sastra · Especificación técnica

Copia de trabajo de las secciones técnicas de la especificación (requisitos, modelo de datos, arquitectura, tecnología, formatos y plan de entregas). Se actualiza cuando cambian.

## Revisión tras la investigación técnica

Revisado el Nov 11, 2026. La investigación mantiene casi todo el stack, pero saca el 3D y el patrón del webview: los dibuja un núcleo nativo en Rust, y la interfaz web queda para formularios, tablas y vista previa. También alarga la Fase 0 y la Fase 3 y añade trabajo legal desde el principio. Las secciones 6, 7, 9, 10 y 11 ya recogen estos cambios; el informe completo con sus fuentes es el archivo de investigación técnica.

| Pieza | Antes | Ahora | Motivo |
| --- | --- | --- | --- |
| 3D y lienzo de patrón | Three.js y PixiJS en el webview | Render nativo en Rust/wgpu bajo un webview transparente | [WebKitGTK no tiene WebGPU](https://bugs.webkit.org/show_bug.cgi?id=257694), y [Tauri advierte](https://v2.tauri.app/develop/debug/linux-graphics/) de WebGL poco fiable en Linux |
| Núcleo | Rust compilado a WASM | Rust nativo | Pasar la malla al webview en cada fotograma es demasiado lento, y los hilos de WASM fallan en macOS |
| Motor de simulación | XPBD propio | Dinámica proyectiva portada a wgpu, con XPBD como línea base de la prueba | Hay una implementación de referencia Apache-2.0 en [Newton](https://github.com/newton-physics/newton) |
| Avatar | MakeHuman / MPFB2 | Activos CC0 de MakeHuman con el modelo [Anny](https://github.com/naver/anny) (Apache-2.0) | El código de MakeHuman es AGPL y el de MPFB2, GPLv3 |
| Importar PSD | ag-psd | Lector en Rust de la imagen compuesta y las capas | ag-psd no lee CMYK, 16 bits ni PSB |
| Importar AI/PDF | PDFium | hayro (Rust), con PDFium de reserva | Mismo rasterizador que usa Typst, sin dependencia en C++ |
| Marcada | jagua-rs | sparrow sobre jagua-rs | jagua-rs es solo el motor de colisiones; [sparrow](https://arxiv.org/html/2509.13329v1) es el optimizador |
| Sincronización | Sin definir | Servidor con autoridad y registro de diferencias, sin CRDT | Es el modelo que [describe Figma](https://www.figma.com/blog/how-figmas-multiplayer-technology-works/) |
| Fase 0 | 6 semanas | 10 a 12 semanas, con estudio de patentes | En agosto de 2026 [CLO cerró su demanda contra Style3D](https://fashionunited.uk/news/business/clo-virtural-fashion-settles-us-patent-infringement-lawsuit-against-style3d/2026082689983) con reconocimiento de infracción |
| Fase 3 | 8 meses | 10 a 12 meses, en dos tramos | Las piezas que faltan en código abierto son las difíciles: autocolisión y capas |
| Linux | Plataforma oficial | Segunda fila al inicio: el 3D en ventana aparte | El webview transparente sobre render nativo aún no es viable allí |

**Qué no cambia:** el orden ficha técnica → patronaje → 3D, Rust y wgpu, Typst para el PDF, SQLite como formato de proyecto, y kurbo, i\_overlay y spade para la geometría.

**Dónde está la ventaja de Sastra:** CLO ya simula en GPU en Mac desde su versión 2026.1, así que el rendimiento 3D no diferencia. Lo que nadie ofrece es una ficha técnica derivada del mismo modelo que el patrón, sin conexión y con los datos en manos del usuario.

**Lo que la investigación no pudo comprobar:** nadie leyó las reivindicaciones de las patentes de CLO; no hay mediciones de Typst con fichas de 20 a 50 páginas ni de tela con autocolisión en wgpu sobre gráficas integradas o Apple Silicon; y Qt, Flutter y Avalonia no se evaluaron. Los plazos y los umbrales de las pruebas son estimaciones, no datos.

## 1. Visión y objetivos

Sastra es una aplicación de escritorio, construida con tecnología open source, que cubre el desarrollo de una prenda de principio a fin: patrón, tallaje, simulación 3D y ficha técnica para fábrica. Todo sale de **un único modelo de datos de la prenda**, así que un cambio en el patrón actualiza las medidas, los consumos y el PDF sin rehacer trabajo.

**Propuesta de valor frente a CLO3D, Browzwear o Style3D:** en esas herramientas el 3D y la ficha técnica viven separados, y la ficha se acaba montando a mano en Illustrator o Excel. Sastra nace desde la ficha técnica y convierte el 3D en una fuente más de datos para ella.

**Objetivos medibles**

1. Reducir al menos un 50 % el tiempo de crear una ficha técnica completa, comparado con el proceso actual (Illustrator + Excel). La línea base se mide en la Fase 0.
2. Cero discrepancias entre patrón, tabla de medidas y BOM, porque se calculan desde la misma fuente.
3. Que una fábrica pueda producir un sample correcto solo con la ficha y los archivos exportados (PDF + DXF), sin aclaraciones por correo.
4. Simular en 3D prendas básicas (camiseta, pantalón, sudadera) sobre un avatar hecho a partir de medidas, y usar esas vistas en la ficha.

**Fuera de alcance:** el diseño gráfico se hace en Adobe Illustrator y Photoshop. Sastra importa esos archivos y los coloca sobre la prenda, pero no los edita.

## 2. Usuarios y casos de uso

El usuario principal es el diseñador o diseñadora de producto que entrega fichas a fábrica. La fábrica es usuaria secundaria: primero recibe los archivos exportados y, a partir de la Fase 4, trabaja en un portal propio.

| Usuario | Qué necesita | Desde fase |
| --- | --- | --- |
| Diseñador/a de producto | Crear la prenda, su ficha técnica y sus variantes; controlar versiones y samples | 1 |
| Patronista | Patrón paramétrico, escalado, márgenes, piquetes y exportación DXF | 2 |
| Product manager / compras | BOM, consumos, proveedores y escandallo | 1 (escandallo, 5) |
| Fábrica / proveedor | Leer la ficha en su idioma, comentar y subir fotos y medidas de los samples | 1 (PDF), 4 (portal) |
| Control de calidad | Puntos de control, tolerancias y nivel AQL | 1 |

**Casos de uso principales**

1. **Nueva prenda desde plantilla:** se elige el tipo (camiseta, pantalón…) y se hereda su estructura de medidas, construcción y etiquetas.
2. **Completar la ficha técnica:** planos, BOM, POM con tolerancias, construcción, gráficos, etiquetas, packaging, controles de calidad y descripción final.
3. **Importar y colocar un gráfico:** se trae un PSD, AI o SVG, se elige técnica y colores, y se sitúa con medidas respecto a un punto de referencia (por ejemplo, el punto alto del hombro).
4. **Crear y escalar el patrón:** se dibuja en la talla base con fórmulas sobre medidas, y las tallas se generan solas.
5. **Simular en 3D:** se cosen las piezas sobre el avatar, se revisa el ajuste y la tensión, y se generan las vistas para la ficha.
6. **Exportar para fábrica:** ficha en PDF (en el idioma de la fábrica), patrones en DXF-AAMA y BOM en Excel.
7. **Gestionar samples:** se registra cada sample con medidas reales, fotos y comentarios; la app marca qué está fuera de tolerancia y genera una nueva versión de la ficha.

## 3. Requisitos funcionales

Hay 74 requisitos en 9 grupos. La prioridad sigue MoSCoW (**M** imprescindible, **S** debería, **C** podría) y la fase indica cuándo se entrega. Cada requisito tendrá criterios de aceptación detallados en su historia de usuario cuando empiece su fase.

### 3.1 Productos, colecciones y biblioteca

| ID | Requisito | Prioridad | Fase |
| --- | --- | --- | --- |
| RF-PR-01 | Crear, duplicar y archivar productos (estilos) con código, temporada, colección, tipo de prenda, género y estado | M | 1 |
| RF-PR-02 | Plantillas por tipo de prenda con POM, construcción y etiquetas predefinidas | M | 1 |
| RF-PR-03 | Variantes de color (colorways) con color por material y por gráfico | M | 1 |
| RF-PR-04 | Vista de colección con miniaturas, filtros y estado de cada producto | S | 1 |
| RF-BI-01 | Biblioteca de tejidos: composición, gramaje (g/m²), ancho útil, elasticidad, acabado, proveedor, referencia y precio | M | 1 |
| RF-BI-02 | Biblioteca de fornituras: cremalleras (tipo, galga, largo, tirador), botones, corchetes, cordones, ojales, elásticos e hilos | M | 1 |
| RF-BI-03 | Biblioteca de colores con importación de paletas ASE/ACO del usuario (Pantone, Coloro, propias) | M | 1 |
| RF-BI-04 | Proveedores y fábricas con contacto, idioma preferido y certificados | M | 1 |
| RF-BI-05 | Tablas de medidas corporales y rangos de tallas reutilizables (XS–XXL, numéricas, infantiles) | M | 1 |
| RF-BI-06 | Parámetros físicos de tejido para simulación (rigidez, flexión, densidad, fricción) | M | 3 |

### 3.2 Ficha técnica

| ID | Requisito | Prioridad | Fase |
| --- | --- | --- | --- |
| RF-FT-01 | Planos técnicos delantero, espalda y detalle, importados como SVG/AI/PDF o generados desde 3D | M | 1 (3D en 3) |
| RF-FT-02 | Llamadas (callouts) numeradas sobre los planos, vinculadas a construcción, fornituras o gráficos | M | 1 |
| RF-FT-03 | BOM: cada línea con material, ubicación, color por variante, consumo por talla, unidad, proveedor y coste | M | 1 |
| RF-FT-04 | POM: punto de medida con código, descripción, método de medición con ilustración, tolerancia ± y valor por talla | M | 1 |
| RF-FT-05 | Reglas de escalado del POM (incremento por talla, fijo o por fórmula) | M | 1 |
| RF-FT-06 | Construcción: operaciones con tipo de costura (ISO 4916), clase de puntada (ISO 4915), SPI, hilo, ancho de pespunte y ubicación | M | 1 |
| RF-FT-07 | Acabados: lavados, tintes en prenda, termosellados, remates, dobladillos | M | 1 |
| RF-FT-08 | Gráficos: técnica (serigrafía, DTG, DTF, transfer, sublimación, bordado, parche, láser), colores, tamaño y posición con cotas a un punto de referencia | M | 1 |
| RF-FT-09 | Escalado de gráficos por talla (fijo, proporcional o por grupos de tallas) | S | 1 |
| RF-FT-10 | Etiquetas: principal, talla, composición y cuidado (Reglamento UE 1007/2011, símbolos GINETEX/ISO 3758), origen, hangtag y su posición | M | 1 |
| RF-FT-11 | Generador de etiqueta de composición multilingüe a partir de la BOM | S | 1 |
| RF-FT-12 | Packaging: doblado con ilustración, bolsa, colgador, EAN-13, marcas de caja y surtido de tallas por caja | M | 1 |
| RF-FT-13 | Puntos de control del sample, clasificados como crítico, mayor o menor, y nivel AQL de inspección | M | 1 |
| RF-FT-14 | Descripción textual completa del producto generada desde los datos y editable | M | 1 |
| RF-FT-15 | Requisitos de ensayos (encogimiento, solidez del color, pilling, REACH) y certificados exigidos | S | 1 |
| RF-FT-16 | Comprobación de completitud: avisa de campos que faltan antes de exportar | M | 1 |
| RF-FT-17 | Plantillas de PDF personalizables con logo, portada, orden de secciones e idioma | S | 1 |

### 3.3 Importación de gráficos

| ID | Requisito | Prioridad | Fase |
| --- | --- | --- | --- |
| RF-IM-01 | Importar PSD con capas, conservando nombres, visibilidad y modos básicos | M | 1 |
| RF-IM-02 | Importar AI con compatibilidad PDF, PDF y SVG como vector | M | 1 |
| RF-IM-03 | Importar PNG, JPG y TIFF con DPI real y aviso si la resolución no basta para el tamaño de impresión | M | 1 |
| RF-IM-04 | Detectar colores planos del gráfico y proponer el color más cercano de la biblioteca (Delta E 2000) | S | 1 |
| RF-IM-05 | Recargar un gráfico cuando cambia el archivo original, conservando su colocación | S | 1 |
| RF-IM-06 | Importar archivos de bordado (DST, PES) con vista previa, número de puntadas y colores de hilo | C | 2 |

### 3.4 Patronaje 2D

| ID | Requisito | Prioridad | Fase |
| --- | --- | --- | --- |
| RF-PA-01 | Puntos paramétricos (a distancia y ángulo, intersección, en curva) definidos por fórmulas sobre medidas | M | 2 |
| RF-PA-02 | Curvas Bézier y splines con control de longitud | M | 2 |
| RF-PA-03 | Piezas con línea de corte, línea de costura y margen de costura variable por tramo | M | 2 |
| RF-PA-04 | Piquetes, taladros, hilo recto, simetría, pinzas, pliegues y frunces | M | 2 |
| RF-PA-05 | Herramientas de transformación: rotar pinza, abrir y repartir, cortar y unir piezas | S | 2 |
| RF-PA-06 | Medir y comparar longitudes de costuras que se unen (por ejemplo, sisa y copa de manga) | M | 2 |
| RF-PA-07 | Importar patrones de Seamly2D/Valentina y DXF-AAMA | S | 2 |
| RF-PA-08 | Importar fotos de patrones en papel, digitalizadas y calibradas con una referencia de escala | C | 5 |

### 3.5 Tallaje

| ID | Requisito | Prioridad | Fase |
| --- | --- | --- | --- |
| RF-TA-01 | Escalado automático del patrón al cambiar la tabla de medidas | M | 2 |
| RF-TA-02 | Escalado por reglas en puntos (tabla de incrementos x/y) como alternativa | S | 2 |
| RF-TA-03 | Vista de nido de tallas superpuestas | M | 2 |
| RF-TA-04 | POM medido automáticamente sobre el patrón en todas las tallas y comparado con la especificación | M | 2 |

### 3.6 Simulación 3D

| ID | Requisito | Prioridad | Fase |
| --- | --- | --- | --- |
| RF-3D-01 | Avatar paramétrico generado desde la tabla de medidas, con poses básicas | M | 3 |
| RF-3D-02 | Colocar piezas alrededor del avatar con puntos de ensamblaje (arrangement points) | M | 3 |
| RF-3D-03 | Definir costuras entre tramos de piezas y simular el cosido | M | 3 |
| RF-3D-04 | Simulación de tela con colisiones contra el avatar y entre capas de la prenda | M | 3 |
| RF-3D-05 | Mapas de tensión, estiramiento y holgura | S | 3 |
| RF-3D-06 | Gráficos y texturas proyectados sobre la prenda, en sincronía con su posición en la ficha | M | 3 |
| RF-3D-07 | Fornituras 3D básicas (botones, cremalleras como objetos rígidos o restricciones) | C | 3 |
| RF-3D-08 | Renders y vistas técnicas exportadas a la ficha técnica | M | 3 |
| RF-3D-09 | Cambio de talla del avatar y de la prenda en vivo | S | 3 |
| RF-3D-10 | Exportar la prenda en glTF/OBJ | S | 3 |

### 3.7 Exportación

| ID | Requisito | Prioridad | Fase |
| --- | --- | --- | --- |
| RF-EX-01 | Ficha técnica en PDF por secciones y completa, con idioma elegible | M | 1 |
| RF-EX-02 | BOM y POM a Excel (XLSX) y CSV | M | 1 |
| RF-EX-03 | Patrones a DXF-AAMA/ASTM D6673 con piquetes, hilo recto y tallas | M | 2 |
| RF-EX-04 | Patrones a PDF a escala 1:1 (plotter y A4 en mosaico) | S | 2 |
| RF-EX-05 | Estimación de consumo con marcada automática (nesting) por ancho de tejido | S | 2 |
| RF-EX-06 | Paquete para fábrica: ZIP con PDF, DXF, gráficos originales y registro de cambios | M | 2 |

### 3.8 Samples, versiones y colaboración

| ID | Requisito | Prioridad | Fase |
| --- | --- | --- | --- |
| RF-SA-01 | Registro de samples por etapa (proto, fit, PP, TOP) con fecha, fábrica y estado | M | 1 |
| RF-SA-02 | Medidas reales del sample con cálculo de desviación y marcado automático fuera de tolerancia | M | 1 |
| RF-SA-03 | Fotos y comentarios anclados a zonas de la prenda | S | 1 |
| RF-VE-01 | Versiones de la ficha con registro de cambios automático (qué cambió, quién y cuándo) | M | 1 |
| RF-VE-02 | Comparar dos versiones y resaltar diferencias en el PDF | S | 1 |
| RF-CO-01 | Servidor de sincronización opcional para trabajar en equipo | M | 4 |
| RF-CO-02 | Portal de fábrica: ver fichas, comentar, subir medidas y fotos de samples, aprobar | M | 4 |
| RF-CO-03 | Notificaciones de cambios y aprobaciones | S | 4 |

### 3.9 Asistente y extras

| ID | Requisito | Prioridad | Fase |
| --- | --- | --- | --- |
| RF-IA-01 | Redactar la descripción del producto y los puntos de control a partir de los datos, con modelo local o en la nube | S | 1 |
| RF-IA-02 | Traducir la ficha al idioma de la fábrica con glosario técnico textil propio | S | 4 |
| RF-IA-03 | Revisión de coherencia: detectar contradicciones entre BOM, construcción y etiquetas | C | 5 |
| RF-EX-07 | Escandallo: coste de materiales, confección, estampación y packaging por talla y variante | C | 5 |
| RF-EX-08 | Exportar datos para el Pasaporte Digital de Producto de la UE | C | 5 |

## 4. Requisitos no funcionales

La aplicación funciona sin conexión (local-first): el servidor solo hace falta para colaborar. Los objetivos de rendimiento se miden en un equipo de referencia de gama media: 16 GB de RAM y GPU integrada reciente o dedicada de entrada.

| ID | Categoría | Requisito |
| --- | --- | --- |
| RNF-01 | Plataformas | Primero Windows, que es tu sistema; al publicar, Windows 10/11 x64, macOS 13+ (Apple Silicon e Intel) y Linux (Ubuntu 22.04+) como plataforma de segunda fila al inicio, con el 3D en ventana aparte |
| RNF-02 | Sin conexión | Todas las funciones salvo colaboración e IA en la nube funcionan sin Internet |
| RNF-03 | Rendimiento 2D | Editor de patrón a 60 fps con 200 piezas y 7 tallas visibles |
| RNF-04 | Rendimiento 3D | Camiseta de unas 20.000 partículas a ≥ 30 fps en interacción; cosido inicial en menos de 10 s |
| RNF-05 | Exportación | Ficha de 20 páginas en PDF en menos de 5 s |
| RNF-06 | Arranque | Abrir la app en menos de 3 s y un proyecto típico en menos de 2 s |
| RNF-07 | Fiabilidad | Autoguardado continuo, recuperación tras cierre inesperado y deshacer ilimitado por sesión |
| RNF-08 | Datos | Formato de proyecto abierto y documentado (.sastra = SQLite + carpeta de recursos), con migraciones entre versiones |
| RNF-09 | Idiomas | Interfaz en español e inglés; PDFs en ES, EN, PT, IT, FR, TR y ZH (con fuentes CJK) |
| RNF-10 | Precisión | Geometría en milímetros con doble precisión; error de exportación DXF menor de 0,1 mm |
| RNF-11 | Seguridad | Binarios firmados (Windows y macOS), actualizaciones firmadas y datos cifrados en tránsito con el servidor |
| RNF-12 | Privacidad | Nada sale del equipo sin acción del usuario; la IA en la nube es opcional y se avisa de lo que se envía; RGPD si hay servidor |
| RNF-13 | Licencias | Solo dependencias con licencias compatibles con el modelo de negocio elegido; inventario SBOM y comprobación automática en CI |
| RNF-14 | Extensibilidad | API de plugins para importadores, exportadores y plantillas de PDF |
| RNF-15 | Calidad | Cobertura de pruebas ≥ 80 % en el núcleo geométrico; pruebas de regresión visual de PDF y del 3D |
| RNF-16 | Accesibilidad | Navegación por teclado completa en formularios y tablas; contraste AA |

## 5. Modelo de datos de la prenda

El **Producto** es la raíz: todas las demás entidades cuelgan de él o de la biblioteca compartida, y la ficha técnica es una vista calculada de ese conjunto, nunca un documento aparte. Cada cambio queda registrado como una operación, lo que permite deshacer, comparar versiones y sincronizar con el servidor.

| Entidad | Contiene | Se relaciona con |
| --- | --- | --- |
| Producto | Código, nombre, temporada, colección, tipo, género, estado, plantilla de origen | Todo lo demás |
| Versión | Número, fecha, autor, registro de operaciones, nota de cambios | Producto |
| Variante de color | Nombre, código, color asignado a cada línea de BOM y a cada color de gráfico | BOM, Gráfico, Color |
| Rango de tallas | Tallas, talla base, tabla de medidas corporales | Medidas, Patrón, POM |
| Patrón | Piezas, puntos paramétricos, curvas, fórmulas, márgenes, piquetes, hilo recto | Rango de tallas, Costura, POM |
| Pieza | Contorno, número de cortes, tejido, simétrica sí/no, malla 3D | Patrón, Línea de BOM |
| POM | Código, descripción, método, tolerancia ±, regla de escalado, valores por talla, vínculo opcional a geometría | Patrón, Sample |
| Línea de BOM | Material, ubicación, consumo por talla, unidad, color por variante, proveedor, coste | Material, Variante, Pieza |
| Material | Tejido o fornitura con sus atributos y parámetros físicos de simulación | Proveedor, biblioteca |
| Operación de construcción | Tipo de costura, clase de puntada, SPI, hilo, ancho, tramos de pieza que une | Pieza, Material (hilo) |
| Gráfico | Archivo de origen, capas, técnica, colores, tamaño, anclaje (punto de referencia + desplazamiento), escalado por talla | Pieza, Variante |
| Etiqueta | Tipo, contenido, idiomas, símbolos de cuidado, composición calculada, posición | Material, Pieza |
| Packaging | Doblado, bolsa, colgador, EAN por talla y variante, marcas de caja, surtido | Variante, Rango de tallas |
| Punto de control | Descripción, severidad (crítico, mayor, menor), referencia a POM u operación | POM, Construcción |
| Sample | Etapa, fábrica, fechas, medidas reales, fotos, comentarios, estado | POM, Punto de control, Versión |
| Proveedor / Fábrica | Datos de contacto, idioma, certificados | Material, Sample |
| Recurso | Archivo binario (imagen, PSD, PDF, DXF) con hash, guardado en la carpeta del proyecto | Gráfico, Plano, Sample |

**Decisiones de diseño del modelo**

- Todas las medidas se guardan en milímetros; la interfaz muestra cm o pulgadas.
- Los valores derivados (POM medido, consumos, composición) se recalculan y se guardan en caché, pero nunca se editan a mano. Si el usuario los sobrescribe, queda marcado como "valor manual".
- Los identificadores son UUID v7 para poder fusionar datos de varios equipos sin conflictos.
- El esquema se versiona con migraciones y se publica como especificación abierta del formato .sastra.

## 6. Arquitectura

Sastra es un núcleo nativo en Rust con una interfaz web encima. El núcleo guarda el modelo de la prenda, calcula la geometría y la simulación, y dibuja el patrón y el 3D sobre un único dispositivo gráfico (wgpu). El webview de Tauri se superpone como capa transparente con los formularios, las tablas y la vista previa de la ficha.

*(Diagrama: arquitectura de Sastra · 3 capas y servidor opcional. Ver la especificación original.)*

La malla de la prenda nunca cruza al webview: entre la interfaz y el núcleo solo viajan órdenes y datos de tablas. Es el patrón que [los mantenedores de Tauri](https://github.com/tauri-apps/tauri/issues/13740) dan por viable en Windows y macOS, y parecido al que adoptó [Graphite](https://github.com/GraphiteEditor/Graphite/issues/2535) tras dejar Tauri.

**Plan B:** si la ventana híbrida falla en la Fase 0, la interfaz pasa a Electron, que trae el mismo Chromium en los tres sistemas a cambio de instaladores mucho mayores. El motor y el render se escriben contra wgpu, que también funciona sobre WebGPU, para que ese cambio no obligue a reescribirlos.

**Linux:** al principio el 3D va en una ventana nativa separada, porque el webview transparente sobre render nativo aún no es viable allí.

**Simulación a dos velocidades:** un modo interactivo para trabajar (dinámica proyectiva en GPU, contactos por repulsión, presupuesto estimado de 10.000 a 50.000 vértices por prenda) y, más adelante, un modo opcional de ajuste final, más lento y sin penetraciones. El cuerpo colisiona mediante un campo de distancias precalculado.

**Principios de arquitectura**

- **Almacén de registros con diferencias reversibles:** cada edición produce una diferencia que sirve a la vez para deshacer, para el registro de cambios de cada versión, para invalidar datos derivados y, en la Fase 4, para sincronizar con un servidor que fija el orden. No hace falta CRDT de entrada.
- **Grafo de dependencias, no solver de restricciones:** medidas → puntos → curvas → piezas → margen → POM, consumos y malla. Si cambia una medida, solo se recalcula lo que depende de ella, y cambiar de talla es evaluar el mismo grafo con otra fila de medidas.
- **Geometría en milímetros y doble precisión;** solo se pasa a precisión simple para dibujar y simular.
- **Ficha reactiva:** un compilador Typst vivo dentro del núcleo recibe el modelo como datos, recompila solo las secciones afectadas y envía a la interfaz las páginas que cambian.
- **Plugins:** importadores, exportadores y plantillas de PDF se cargan como módulos con una API estable.
- **Local-first:** el archivo .sastra es la fuente de verdad; el servidor solo replica y coordina.

## 7. Stack tecnológico open source

La propuesta es **un núcleo nativo en Rust sobre wgpu, con Tauri 2 + React/TypeScript solo para la interfaz**. Se mantiene el árbol de dependencias sin GPL ni AGPL para dejar abierto cualquier modelo de negocio. La columna «Estado» distingue las licencias leídas en el repositorio durante la investigación de las que siguen pendientes de confirmar.

| Capa | Tecnología | Licencia | Estado | Uso |
| --- | --- | --- | --- | --- |
| Contenedor de escritorio | [Tauri 2](https://v2.tauri.app/release/tauri/) (2.12) | MIT / Apache-2.0 | Pendiente de confirmar | Ventana, menús, instaladores, actualizaciones; webview transparente sobre el render nativo |
| Interfaz | React + TypeScript + Vite | MIT | Pendiente de confirmar | Formularios, tablas, paneles, vista previa de la ficha |
| Componentes y tablas | Radix UI, Tailwind CSS, TanStack Table y Virtual | MIT | Pendiente de confirmar | BOM, POM y tablas de tallas con miles de celdas |
| GPU | [wgpu](https://github.com/gfx-rs/wgpu) | MIT / Apache-2.0 | Verificada | Único dispositivo gráfico para render y cómputo en Direct3D 12, Metal y Vulkan |
| Render 2D y 3D | Renderizador propio sobre wgpu | Propia | A confirmar con prueba | Patrón, nido de tallas, avatar y prenda. Sustituye a Three.js y PixiJS |
| Curvas | [kurbo](https://docs.rs/kurbo) | Apache-2.0 / MIT | Verificada | Longitud de arco, ajuste y paralelas; el margen de costura necesita una capa propia encima |
| Booleanas y desplazamiento | i\_overlay | MIT / Apache-2.0 | Verificada | Limpieza de autointersecciones, unión y corte de piezas |
| Mallado | [spade](https://github.com/Stoeoef/spade) | MIT / Apache-2.0 | Verificada | Triangulación de Delaunay con restricciones y refinamiento |
| Recálculo incremental | Evaluador propio de grafo de dependencias (salsa como referencia) | Propia | Inferencia | Fórmulas, puntos, piezas, POM y consumos por talla |
| Simulación interactiva | Dinámica proyectiva propia en WGSL, portada del solver Style3D de [Newton](https://github.com/newton-physics/newton) | Apache-2.0 (referencia) | Verificada | Cosido, caída y colisiones en tiempo real; ruta de CPU para gráficas débiles |
| Simulación de ajuste final | [ppf-contact-solver](https://github.com/st-tech/ppf-contact-solver) | Apache-2.0 | Verificada | Modo opcional sin penetraciones; no es de tiempo real ni está listo para producción |
| Avatar | Activos CC0 de MakeHuman + parametrización de [Anny](https://github.com/naver/anny), reimplementada en Rust | CC0 / Apache-2.0 | Verificada | Cuerpo desde medidas. Se excluye la topología SMPL-X opcional de Anny, no comercial |
| Marcada | [sparrow](https://arxiv.org/html/2509.13329v1) sobre jagua-rs | MIT / MPL-2.0 | Verificada (sparrow, por réplica) | Consumo de tejido; hilo y sentido del pelo como rotaciones permitidas |
| Base de datos local | SQLite | Dominio público | Pendiente de confirmar | Proyecto .sastra, con recursos pesados aparte, identificados por hash |
| PDF de la ficha | [Typst](https://typst.app/docs/changelog/0.15.0/) 0.15 embebido | Apache-2.0 | Pendiente de confirmar | Ficha multilingüe con PDF/A, PDF etiquetado y tintas directas |
| Lectura PSD | Crate [psd](https://github.com/chinedufn/psd) u otro lector propio | MIT / Apache-2.0 | Verificada | Imagen compuesta incrustada, nombres y posiciones de capas |
| Lectura PDF/AI | [hayro](https://docs.rs/crate/hayro/latest) 0.9, con PDFium de reserva | Apache-2.0 / MIT | Verificada (hayro) | Archivos de Illustrator guardados con compatibilidad PDF |
| SVG | resvg / usvg | Apache-2.0 / MIT | Pendiente de confirmar | Importación y rasterizado de vectores |
| DXF | Lector y escritor AAMA/ASTM propio (crate dxf como base, MIT) | Propia | Verificada (crate dxf) | Perfil R10/R12 conservador, escalado en RUL y en nido |
| Excel | rust\_xlsxwriter y calamine | MIT / Apache-2.0 | Pendiente de confirmar | BOM y POM con imágenes incrustadas |
| Color | [moxcms](https://docs.rs/crate/moxcms/0.8.0/source/README.md) | BSD-3 / Apache-2.0 | Verificada | Perfiles ICC, CMYK a RGB y diferencia de color |
| Bordado | pyembroidery como referencia de formato | MIT | Verificada | Lectura de DST y PES, reimplementada en Rust |
| IA opcional | llama.cpp (local) o API externa intercambiable | MIT | No investigada | Descripciones, puntos de control y traducciones |
| Servidor (Fase 4) | Rust (Axum) + PostgreSQL + almacenamiento S3 compatible | MIT / PostgreSQL | Pendiente de confirmar | Servidor con autoridad y portal de fábrica |
| Pruebas y CI | cargo test, Vitest, Playwright, GitHub Actions, cargo-deny | MIT / Apache-2.0 | Pendiente de confirmar | Pruebas, empaquetado y control de licencias |

**Por qué se mantiene Tauri, con otro papel:** una ficha técnica es sobre todo tablas y formularios, y ahí la web gana a las interfaces nativas de Rust. Lo que no puede hacer el webview del sistema es el 3D: en Linux no hay WebGPU, y en macOS 13 a 15 probablemente tampoco. Por eso el render y el cálculo son nativos. Electron queda como plan B y Tauri 3, que permite empaquetar Chromium, sigue en alfa.

**Avisos de licencia que afectan al producto**

- **Pantone** no se puede incluir: es contenido con licencia. El código Pantone se trata como un texto que escribe el usuario o que se lee de su propio arte, sin tablas de equivalencia.
- **Símbolos de cuidado GINETEX:** son [marcas registradas](https://www.ginetex.net/userfiles/files/gin_bro_gb_rz_brochure_web.pdf) y su reproducción exige autorización. Hay que consultar a GINETEX antes de incluirlos en la app.
- **Solo interoperar por archivos:** Seamly2D, Valentina (GPLv3) y Blender (GPL). No se copia su código.
- **Solo los activos, nunca el código:** MakeHuman (AGPL) y MPFB2 (GPLv3).
- **Prohibidos para uso comercial:** el simulador de GarmentCode (su código de patrones sí es MIT), SMPL-X, STAR y Triangle.
- **Evitar:** MuPDF (AGPL), el paquete de triangulación 2D de CGAL (GPL) y los solvers de restricciones de SolveSpace (GPLv3).
- **jagua-rs (MPL-2.0):** se puede usar, publicando las modificaciones hechas a sus propios archivos.

## 8. Formatos de importación y exportación

Para la fábrica basta con PDF y DXF-AAMA. Los demás formatos sirven para integrarse con Adobe, con otros CAD y con hojas de cálculo.

| Formato | Entrada / salida | Uso | Fase |
| --- | --- | --- | --- |
| .sastra (SQLite + recursos) | Ambos | Proyecto nativo, documentado y abierto | 1 |
| PSD | Entrada | Gráficos de Photoshop con capas | 1 |
| AI (con compatibilidad PDF), PDF, SVG | Entrada | Gráficos vectoriales y planos técnicos de Illustrator | 1 |
| PNG, JPG, TIFF | Entrada | Gráficos rasterizados, fotos de samples | 1 |
| ASE, ACO | Entrada | Paletas de color (Pantone, Coloro o propias) | 1 |
| PDF | Salida | Ficha técnica completa o por secciones | 1 |
| XLSX, CSV | Ambos | BOM, POM, tablas de medidas | 1 |
| PNG, SVG | Salida | Planos y vistas para presentaciones | 1 |
| DXF-AAMA / ASTM D6673 | Ambos | Patrones para CAD de fábrica (Gerber, Lectra, Optitex…) | 2 |
| PDF 1:1 y A4 en mosaico | Salida | Impresión de patrones | 2 |
| .val / .vit / .sm2d | Entrada | Patrones y medidas de Seamly2D y Valentina | 2 |
| DST, PES | Entrada | Archivos de bordado para vista previa y recuento | 2 |
| glTF, OBJ | Salida | Prenda 3D para render, e-commerce o realidad aumentada | 3 |
| ZIP de fábrica | Salida | PDF + DXF + gráficos originales + registro de cambios | 2 |
| JSON del Pasaporte Digital de Producto | Salida | Cumplimiento normativo UE, cuando se publique el acto delegado para textil | 5 |

Los archivos .ai que no se guardaron con compatibilidad PDF no se pueden leer de forma fiable, porque su formato interno es privado. La app lo detecta y pide volver a guardar el archivo o exportarlo a PDF/SVG.

## 9. Plan por fases

El plan se organiza por entregas, no por meses. Cada entrega es una versión que instalas y usas con una prenda real, y el ritmo depende de cuántas sesiones de trabajo hagamos y del tiempo que tengas para probar. La ficha técnica es la parte más corta y la que antes te ahorra trabajo; el 3D es, con diferencia, la más larga e incierta.

*(Diagrama: hoja de ruta · 6 fases y 5 puertas de calidad. Ver la especificación original.)*

**Entregas**

| Entrega | Contenido | Qué te aporta |
| --- | --- | --- |
| 0 · Preparación | Repositorio en GitHub, guía del proyecto (CLAUDE.md), pruebas automáticas, instalador generado automáticamente para tu sistema, aplicación mínima que se abre y guarda un archivo .sastra | La base de todo lo demás |
| 1.1 · Productos y tablas | Productos y plantillas; biblioteca de materiales, fornituras, colores y proveedores; BOM; POM con tolerancias y escalado; exportación a Excel | Sustituye tus Excel de materiales y medidas |
| 1.2 · Planos y gráficos | Planos importados (SVG, AI, PDF) con llamadas numeradas; construcción y acabados; gráficos de Photoshop e Illustrator con técnica, colores y colocación acotada | Las páginas visuales de la ficha |
| 1.3 · Etiquetas, packaging y calidad | Etiquetas con composición automática, packaging, puntos de control con AQL, descripción del producto | La ficha completa en contenido |
| 1.4 · PDF y versiones | Ficha en PDF con Typst, comprobación de campos que faltan, versiones con registro de cambios, registro de samples con medidas reales | Tu primera ficha real enviada a fábrica desde Sastra |
| 2 · Patronaje y tallaje | Patrón paramétrico, márgenes y piquetes, escalado, POM medido sobre el patrón, DXF-AAMA, PDF 1:1, consumo de tejido | Patrón y medidas conectados con la ficha |
| 3 · Vista 3D | Pruebas previas de ventana híbrida y motor de tela; avatar desde medidas; cosido y caída de prendas sencillas de una capa; gráficos sobre la prenda; vistas para la ficha | Ver la prenda antes del primer sample |
| 4 · Publicar en tu web | Elegir licencia; revisar patentes, marca y símbolos de cuidado; firmar los instaladores; página de descarga | Compartir Sastra con otras personas |

Tu papel en cada entrega es probar la versión con una prenda real y contarme qué falla o qué falta. Una entrega no se da por cerrada hasta que la has usado.

**Pruebas técnicas, justo antes de la entrega que las necesita**

Los criterios son una propuesta inicial para tener un aprobado o suspenso claro.

| Prueba | Antes de | Criterio propuesto |
| --- | --- | --- |
| Ficha Typst de 30 páginas con arte real | 0 (condiciona toda la ficha) | Vista previa actualizada en menos de 200 ms tras cambiar una celda |
| Importación de 20 archivos PSD y AI tuyos, con CMYK, 16 bits, PSB y AI sin compatibilidad PDF | 1.2 | Lista de casos admitidos y errores claros para el resto |
| Margen de costura sobre 50 patrones reales | 2 | Ningún fallo ni autointersección |
| Grafo incremental con 30 tallas | 2 | Todas las tallas recalculadas en un fotograma al mover un punto |
| Marcada con sparrow sobre un modelo real escalado | 2 | Aprovechamiento comparable a una marcada manual |
| Ida y vuelta de DXF-AAMA con escalado hacia CLO y hacia el CAD de una fábrica | Cierre de 2 | Tallas, piquetes e hilos intactos |
| Ventana híbrida: render nativo bajo el webview de Tauri en tu sistema | 3 | Sin parpadeo y 60 fps con 200.000 triángulos; si falla, plan B |
| Comparativa de motores de tela (XPBD frente a dinámica proyectiva) en tu ordenador | 3 | 30 fps con 20.000 vértices |
| Avatar con activos CC0 ajustado a seis medidas | 3 | Menos de 1 cm de diferencia en contornos principales |
| Revisión de patentes, marca y símbolos GINETEX | 4 | Informe de un abogado antes de publicar |

## Fuentes

Páginas en las que se apoyan los cambios de la revisión. El informe de investigación técnica recoge la lista completa y separa los hechos verificados de las inferencias.

- [WebKit Bugzilla 257694: WebGPU en WebKitGTK](https://bugs.webkit.org/show_bug.cgi?id=257694)
- [Tauri: gráficos en Linux](https://v2.tauri.app/develop/debug/linux-graphics/)
- [Tauri #13740: render nativo bajo el webview](https://github.com/tauri-apps/tauri/issues/13740)
- [Graphite #2535: interfaz con CEF sobre render nativo](https://github.com/GraphiteEditor/Graphite/issues/2535)
- [Newton, motor de física con el solver Style3D](https://github.com/newton-physics/newton)
- [ppf-contact-solver, de ZOZO](https://github.com/st-tech/ppf-contact-solver)
- [Vertex Block Descent, comparación con XPBD](https://arxiv.org/pdf/2403.06321)
- [Anny, modelo corporal de NAVER](https://github.com/naver/anny)
- [sparrow, optimizador de marcada](https://arxiv.org/html/2509.13329v1)
- [Typst 0.15, registro de cambios](https://typst.app/docs/changelog/0.15.0/)
- [Cómo funciona el multijugador de Figma](https://www.figma.com/blog/how-figmas-multiplayer-technology-works/)
- [Comunicado de CLO sobre el acuerdo con Style3D](https://prod-ss.aap.com.au/aapreleases/cision20260825ae32418/) y [nota de FashionUnited](https://fashionunited.uk/news/business/clo-virtural-fashion-settles-us-patent-infringement-lawsuit-against-style3d/2026082689983)
- [GINETEX, folleto sobre los símbolos de cuidado](https://www.ginetex.net/userfiles/files/gin_bro_gb_rz_brochure_web.pdf)
- [NLnet, financiación](https://nlnet.nl/funding.html)
- [CDTI, convocatoria Neotec 2026](https://www.cdti.es/en/noticias/cdti-innovacion-convocatoria-neotec-2026-subvencion-ebts-empresas-tecnologia)
