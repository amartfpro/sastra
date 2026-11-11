import { useEffect, useRef, useState, type FormEvent } from "react";
import { errorMessage, type NewProduct, type Product } from "../api";
import { EMPTY_PRODUCT, validateProduct, type FieldErrors } from "../products";

interface Props {
  title: string;
  initial?: Product;
  existing: Product[];
  onSubmit: (input: NewProduct) => Promise<void>;
  onCancel: () => void;
}

const FIELDS: { key: keyof NewProduct; label: string; placeholder: string; required?: boolean }[] = [
  { key: "code", label: "Código", placeholder: "SS27-TS-001", required: true },
  { key: "name", label: "Nombre", placeholder: "Camiseta básica", required: true },
  { key: "season", label: "Temporada", placeholder: "SS27" },
  { key: "collection", label: "Colección", placeholder: "Esenciales" },
  { key: "garmentType", label: "Tipo de prenda", placeholder: "Camiseta" },
];

export function ProductForm({ title, initial, existing, onSubmit, onCancel }: Props) {
  const [values, setValues] = useState<NewProduct>(
    initial
      ? {
          code: initial.code,
          name: initial.name,
          season: initial.season,
          collection: initial.collection,
          garmentType: initial.garmentType,
        }
      : EMPTY_PRODUCT,
  );
  const [errors, setErrors] = useState<FieldErrors>({});
  const [submitError, setSubmitError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const firstInput = useRef<HTMLInputElement>(null);

  useEffect(() => {
    firstInput.current?.focus();
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && onCancel();
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [onCancel]);

  async function handleSubmit(e: FormEvent) {
    e.preventDefault();
    const found = validateProduct(values, existing, initial?.id);
    setErrors(found);
    if (Object.keys(found).length > 0) return;
    setBusy(true);
    setSubmitError(null);
    try {
      await onSubmit(values);
    } catch (err) {
      setSubmitError(errorMessage(err));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="overlay" onMouseDown={(e) => e.target === e.currentTarget && onCancel()}>
      <form className="panel" onSubmit={handleSubmit} noValidate>
        <h2>{title}</h2>
        {FIELDS.map((f, i) => (
          <label key={f.key} className="field">
            <span>
              {f.label}
              {f.required && <span className="required"> *</span>}
            </span>
            <input
              ref={i === 0 ? firstInput : undefined}
              value={values[f.key]}
              placeholder={f.placeholder}
              onChange={(e) => setValues({ ...values, [f.key]: e.target.value })}
              aria-invalid={Boolean(errors[f.key])}
            />
            {errors[f.key] && <small className="error">{errors[f.key]}</small>}
          </label>
        ))}
        {submitError && (
          <p className="error" role="alert">
            {submitError}
          </p>
        )}
        <div className="panel-actions">
          <button type="button" onClick={onCancel}>
            Cancelar
          </button>
          <button type="submit" className="primary" disabled={busy}>
            {initial ? "Guardar cambios" : "Crear producto"}
          </button>
        </div>
      </form>
    </div>
  );
}
