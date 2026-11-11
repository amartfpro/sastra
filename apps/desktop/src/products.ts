// Reglas de presentación y validación de productos, sin dependencias de Tauri
// para poder probarlas de forma aislada.
import type { NewProduct, Product, ProductStatus } from "./api";

export const STATUS_LABELS: Record<ProductStatus, string> = {
  draft: "Borrador",
  in_development: "En desarrollo",
  sampling: "En muestras",
  approved: "Aprobado",
  archived: "Archivado",
};

export const STATUS_ORDER: ProductStatus[] = [
  "draft",
  "in_development",
  "sampling",
  "approved",
  "archived",
];

export const EMPTY_PRODUCT: NewProduct = {
  code: "",
  name: "",
  season: "",
  collection: "",
  garmentType: "",
};

export type FieldErrors = Partial<Record<keyof NewProduct, string>>;

/**
 * Comprueba en la interfaz lo mismo que el núcleo, para avisar antes de enviar.
 * El núcleo vuelve a validarlo: esta función es solo una ayuda.
 */
export function validateProduct(input: NewProduct, existing: Product[], editingId?: string): FieldErrors {
  const errors: FieldErrors = {};
  const code = input.code.trim();
  if (!code) errors.code = "El código es obligatorio.";
  else if ([...code].length > 64) errors.code = "Máximo 64 caracteres.";
  else if (
    existing.some(
      (p) => p.id !== editingId && p.code.toLocaleLowerCase("es") === code.toLocaleLowerCase("es"),
    )
  )
    errors.code = "Ya existe un producto con este código.";

  if (!input.name.trim()) errors.name = "El nombre es obligatorio.";
  for (const field of ["name", "season", "collection", "garmentType"] as const) {
    if (!errors[field] && [...input[field].trim()].length > 200) errors[field] = "Máximo 200 caracteres.";
  }
  return errors;
}

/** Filtra por código, nombre, temporada, colección o tipo, sin distinguir acentos ni mayúsculas. */
export function filterProducts(products: Product[], query: string): Product[] {
  const q = normalize(query.trim());
  if (!q) return products;
  return products.filter((p) =>
    [p.code, p.name, p.season, p.collection, p.garmentType].some((v) => normalize(v).includes(q)),
  );
}

function normalize(s: string): string {
  return s.normalize("NFD").replace(/\p{Diacritic}/gu, "").toLocaleLowerCase("es");
}

const DATE_FORMAT = new Intl.DateTimeFormat("es-ES", {
  day: "numeric",
  month: "short",
  year: "numeric",
  hour: "2-digit",
  minute: "2-digit",
});

export function formatDate(ms: number): string {
  return DATE_FORMAT.format(new Date(ms));
}
