//! Prueba de rendimiento de la entrega 0: ficha técnica de unas 30 páginas.
//!
//! Ejecutar con: `cargo run -p sastra-report --example bench_ficha --profile bench-rapido`
//!
//! Mide la compilación en frío, la recompilación sin cambios y la recompilación
//! tras cambiar una sola celda (el caso de la vista previa en vivo), y escribe
//! el PDF resultante en `target/ficha-prueba.pdf`.

use std::io::Cursor;
use std::time::{Duration, Instant};

use sastra_report::Ficha;
use serde_json::{Value, json};

const TEMPLATE: &str = include_str!("../templates/ficha-prueba.typ");
const TALLAS: [&str; 8] = ["XXS", "XS", "S", "M", "L", "XL", "XXL", "3XL"];
const REPETICIONES: usize = 7;

fn main() {
    let mut ficha = Ficha::new(TEMPLATE);
    cargar_datos(&mut ficha);

    // 1. Compilación en frío (incluye la carga de fuentes).
    let t = Instant::now();
    let doc = ficha
        .compile()
        .unwrap_or_else(|e| panic!("error de compilación:\n{e}"));
    let frio = t.elapsed();
    let paginas = doc.pages().len();

    let t = Instant::now();
    let pdf = Ficha::pdf(&doc).expect("exportación a PDF");
    let exportar = t.elapsed();
    let salida = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/ficha-prueba.pdf");
    std::fs::write(salida, &pdf).expect("escribir el PDF");

    // 2. Recompilar sin cambios.
    let sin_cambios = mediana(|| {
        ficha.compile().unwrap();
    });

    // 3. Cambiar una celda de la tabla de medidas y recompilar.
    let mut pom = medidas();
    let mut n = 0.0;
    let una_medida = mediana(|| {
        n += 0.1;
        pom["puntos"][12]["valores"][3] = json!(52.0 + n);
        ficha.set_json("/datos/medidas.json", &pom);
        ficha.compile().unwrap();
    });

    // 4. Cambiar el nombre de un material y recompilar.
    let mut bom = materiales();
    let mut k = 0;
    let un_material = mediana(|| {
        k += 1;
        bom[5]["material"] = json!(format!("Punto jersey 180 g/m² · revisión {k}"));
        ficha.set_json("/datos/materiales.json", &bom);
        ficha.compile().unwrap();
    });

    // 5. Cambiar un dato de la cabecera (afecta a todas las páginas).
    let mut producto = producto();
    let mut v = 1;
    let cabecera = mediana(|| {
        v += 1;
        producto["version"] = json!(format!("v{v}"));
        ficha.set_json("/datos/producto.json", &producto);
        ficha.compile().unwrap();
    });

    // 6. Cambio de una celda + exportación a PDF completa.
    let mut m = 0.0;
    let celda_y_pdf = mediana(|| {
        m += 0.1;
        pom["puntos"][20]["valores"][5] = json!(60.0 + m);
        ficha.set_json("/datos/medidas.json", &pom);
        let doc = ficha.compile().unwrap();
        Ficha::pdf(&doc).unwrap();
    });

    println!(
        "Ficha de prueba: {paginas} páginas, PDF de {} KB",
        pdf.len() / 1024
    );
    println!("Mediana de {REPETICIONES} repeticiones salvo la primera compilación.\n");
    fila("Compilación en frío (con fuentes)", frio);
    fila("Exportar a PDF", exportar);
    fila("Recompilar sin cambios", sin_cambios);
    fila("Recompilar tras cambiar una medida", una_medida);
    fila("Recompilar tras cambiar un material", un_material);
    fila("Recompilar tras cambiar la cabecera", cabecera);
    fila("Cambiar una medida + PDF completo", celda_y_pdf);
    println!("\nCriterio de la entrega 0: vista previa en menos de 200 ms tras cambiar una celda.");
    println!("PDF escrito en {salida}");
}

fn fila(nombre: &str, d: Duration) {
    println!("{nombre:<40} {:>8.1} ms", d.as_secs_f64() * 1000.0);
}

fn mediana(mut f: impl FnMut()) -> Duration {
    let mut tiempos: Vec<Duration> = (0..REPETICIONES)
        .map(|_| {
            let t = Instant::now();
            f();
            t.elapsed()
        })
        .collect();
    tiempos.sort();
    tiempos[REPETICIONES / 2]
}

fn cargar_datos(ficha: &mut Ficha) {
    ficha.set_json("/datos/producto.json", &producto());
    ficha.set_json("/datos/materiales.json", &materiales());
    ficha.set_json("/datos/medidas.json", &medidas());
    ficha.set_json("/datos/construccion.json", &construccion());
    ficha.set_json("/datos/etiquetas.json", &etiquetas());
    ficha.set_json("/datos/calidad.json", &calidad());
    ficha.set_file("/img/plano-delantero.svg", plano(false));
    ficha.set_file("/img/plano-espalda.svg", plano(true));

    let mut arts = Vec::new();
    for i in 0..18 {
        // La mitad de los gráficos son vectoriales y la otra mitad imágenes de 1500 × 1500 px,
        // como los que llegan desde Illustrator y Photoshop.
        let archivo = if i % 2 == 0 {
            let ruta = format!("/img/grafico-{i}.svg");
            ficha.set_file(&ruta, grafico_svg(i));
            ruta
        } else {
            let ruta = format!("/img/grafico-{i}.png");
            ficha.set_file(&ruta, grafico_png(i));
            ruta
        };
        arts.push(json!({
            "nombre": format!("Estampado {}", i + 1),
            "archivo": archivo,
            "tecnica": (["Serigrafía", "DTF", "Bordado", "Transfer"][i % 4]),
            "tamano": format!("{} × {} cm", 20 + i, 18 + i),
            "posicion": format!("Centrado, a {} cm del punto alto del hombro", 6 + i % 4),
            "colores": ["PANTONE 19-4052 TCX", "PANTONE 11-0601 TCX", "PANTONE 17-1463 TCX"],
            "escalado": "Proporcional por grupos de tallas (XXS–S, M–L, XL–3XL)",
        }));
    }
    ficha.set_json("/datos/graficos.json", &Value::Array(arts));
}

fn producto() -> Value {
    json!({
        "codigo": "SS27-TS-001",
        "nombre": "Camiseta básica algodón orgánico",
        "temporada": "Primavera-Verano 2027",
        "marca": "Estudio de prueba",
        "version": "v1",
        "fecha": "11/11/2026",
        "variantes": ["Blanco", "Negro", "Verde"],
        "campos": [
            ["Código", "SS27-TS-001"], ["Nombre", "Camiseta básica algodón orgánico"],
            ["Temporada", "Primavera-Verano 2027"], ["Colección", "Esenciales"],
            ["Tipo de prenda", "Camiseta manga corta"], ["Género", "Unisex"],
            ["Tejido principal", "Punto jersey 100 % algodón orgánico, 180 g/m²"],
            ["Talla base", "M"], ["Rango de tallas", "XXS – 3XL"],
            ["Fábrica", "Proveedor de ejemplo, Portugal"], ["Estado", "En muestras (fit)"],
        ],
    })
}

fn materiales() -> Value {
    let mut filas = Vec::new();
    for i in 0..60 {
        filas.push(json!({
            "material": format!("Material de prueba {}", i + 1),
            "ubicacion": (["Cuerpo", "Cuello", "Bajo", "Interior", "Manga"][i % 5]),
            "proveedor": format!("Proveedor {}", i % 7 + 1),
            "referencia": format!("REF-{:04}", 1000 + i * 7),
            "composicion": "95 % algodón orgánico, 5 % elastano",
            "colores": ["11-0601 TCX", "19-4005 TCX", "18-6018 TCX"],
            "consumo": 0.85 + (i as f64) * 0.01,
            "unidad": if i % 3 == 0 { "m" } else { "ud." },
        }));
    }
    Value::Array(filas)
}

fn medidas() -> Value {
    let mut puntos = Vec::new();
    for i in 0..48 {
        let base = 30.0 + i as f64;
        let valores: Vec<f64> = (0..TALLAS.len())
            .map(|t| ((base + t as f64 * 2.0) * 10.0).round() / 10.0)
            .collect();
        puntos.push(json!({
            "codigo": format!("P{:02}", i + 1),
            "descripcion": format!("Punto de medida {} — descripción completa", i + 1),
            "metodo": "En plano, de costura a costura, 2 cm bajo sisa",
            "tolerancia": if i % 2 == 0 { 0.5 } else { 1.0 },
            "valores": valores,
        }));
    }
    json!({ "tallas": TALLAS, "puntos": puntos })
}

fn construccion() -> Value {
    let mut ops = Vec::new();
    for i in 0..90 {
        ops.push(json!({
            "operacion": format!("Operación de confección {}", i + 1),
            "costura": (["1.01.01", "2.04.06", "6.03.01", "8.03.03"][i % 4]),
            "puntada": (["301", "401", "504", "514", "406"][i % 5]),
            "ppp": 10 + i % 4,
            "hilo": "Poliéster 120, a tono",
            "notas": "Sin frunces; rematar al inicio y al final",
        }));
    }
    Value::Array(ops)
}

fn etiquetas() -> Value {
    let mut filas = Vec::new();
    for i in 0..36 {
        filas.push(json!({
            "tipo": (["Principal", "Talla", "Composición y cuidado", "Hangtag", "Bolsa"][i % 5]),
            "material": "Satén poliéster tejido",
            "contenido": "Texto de ejemplo en ES, EN, FR, PT e IT con símbolos de cuidado",
            "posicion": "Centro espalda, cosida en la costura del cuello",
        }));
    }
    Value::Array(filas)
}

fn calidad() -> Value {
    let mut filas = Vec::new();
    for i in 0..70 {
        filas.push(json!({
            "comprobacion": format!("Comprobación de calidad {} sobre el sample", i + 1),
            "severidad": (["Crítico", "Mayor", "Menor"][i % 3]),
            "referencia": format!("P{:02}", i % 48 + 1),
        }));
    }
    Value::Array(filas)
}

fn plano(espalda: bool) -> Vec<u8> {
    let cuello = if espalda {
        "Q 150 40 190 30"
    } else {
        "Q 150 90 190 30"
    };
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 360">
<path d="M110 30 {cuello} L260 60 L300 140 L250 160 L240 130 L240 340 L60 340 L60 130 L50 160 L0 140 L40 60 Z"
 fill="none" stroke="#222" stroke-width="2"/>
<path d="M60 330 L240 330" stroke="#222" stroke-dasharray="4 3"/>
</svg>"##
    )
    .into_bytes()
}

fn grafico_svg(i: usize) -> Vec<u8> {
    let mut formas = String::new();
    for k in 0..400 {
        let x = (k * 37 + i * 11) % 500;
        let y = (k * 53 + i * 7) % 500;
        let r = 4 + (k * 13) % 18;
        let color = ["#1f3a5f", "#c8553d", "#f2d0a4", "#2f4a3f"][k % 4];
        formas.push_str(&format!(
            r#"<circle cx="{x}" cy="{y}" r="{r}" fill="{color}"/>"#
        ));
    }
    format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 500 500">{formas}</svg>"#)
        .into_bytes()
}

fn grafico_png(i: usize) -> Vec<u8> {
    let lado = 1500;
    let img = image::RgbImage::from_fn(lado, lado, |x, y| {
        let r = ((x * 255) / lado) as u8;
        let g = ((y * 255) / lado) as u8;
        let b = (((x ^ y) as usize * (i + 3)) % 256) as u8;
        image::Rgb([r, g, b])
    });
    let mut out = Cursor::new(Vec::new());
    img.write_to(&mut out, image::ImageFormat::Png)
        .expect("PNG");
    out.into_inner()
}
