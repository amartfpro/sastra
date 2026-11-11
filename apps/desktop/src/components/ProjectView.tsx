import { useCallback, useEffect, useMemo, useState } from "react";
import { ask, save } from "@tauri-apps/plugin-dialog";
import { api, errorMessage, type NewProduct, type Product, type ProductStatus, type ProjectInfo } from "../api";
import { STATUS_LABELS, STATUS_ORDER, filterProducts, formatDate } from "../products";
import { ProductForm } from "./ProductForm";

interface Props {
  project: ProjectInfo;
  onClose: () => void;
}

type Editing = { mode: "new" } | { mode: "edit"; product: Product } | null;

export function ProjectView({ project, onClose }: Props) {
  const [products, setProducts] = useState<Product[]>([]);
  const [query, setQuery] = useState("");
  const [editing, setEditing] = useState<Editing>(null);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);

  const reload = useCallback(async () => {
    try {
      setProducts(await api.listProducts());
    } catch (e) {
      setError(errorMessage(e));
    }
  }, []);

  useEffect(() => {
    reload();
  }, [reload]);

  const visible = useMemo(() => filterProducts(products, query), [products, query]);

  async function submit(input: NewProduct) {
    if (editing?.mode === "edit") {
      await api.updateProduct(editing.product.id, input);
    } else {
      await api.createProduct(input);
    }
    setEditing(null);
    await reload();
  }

  async function changeStatus(product: Product, status: ProductStatus) {
    try {
      await api.updateProduct(product.id, { status });
      await reload();
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  async function remove(product: Product) {
    const ok = await ask(`¿Borrar «${product.code} · ${product.name}»? Esta acción no se puede deshacer todavía.`, {
      title: "Borrar producto",
      kind: "warning",
      okLabel: "Borrar",
      cancelLabel: "Cancelar",
    });
    if (!ok) return;
    try {
      await api.deleteProduct(product.id);
      await reload();
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  async function saveCopy() {
    setError(null);
    try {
      const path = await save({
        title: "Guardar una copia",
        filters: [{ name: "Proyecto de Sastra", extensions: ["sastra"] }],
        defaultPath: `${project.name} (copia).sastra`,
      });
      if (!path) return;
      await api.saveCopy(path);
      setNotice("Copia guardada.");
      setTimeout(() => setNotice(null), 3000);
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  return (
    <div className="project">
      <header className="topbar">
        <div className="topbar-title">
          <span className="brand small">Sastra</span>
          <span className="project-name" title={project.path}>
            {project.name}
          </span>
        </div>
        <div className="topbar-actions">
          <button onClick={saveCopy}>Guardar una copia…</button>
          <button onClick={onClose}>Cerrar proyecto</button>
        </div>
      </header>

      <main className="content">
        <div className="toolbar">
          <h2>Productos</h2>
          <input
            className="search"
            type="search"
            placeholder="Buscar por código, nombre, temporada…"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
          <button className="primary" onClick={() => setEditing({ mode: "new" })}>
            Nuevo producto
          </button>
        </div>

        {error && (
          <p className="error" role="alert" onClick={() => setError(null)}>
            {error}
          </p>
        )}

        {products.length === 0 ? (
          <div className="empty">
            <p>Todavía no hay productos en este proyecto.</p>
            <button className="primary" onClick={() => setEditing({ mode: "new" })}>
              Crear el primero
            </button>
          </div>
        ) : (
          <table className="grid">
            <thead>
              <tr>
                <th>Código</th>
                <th>Nombre</th>
                <th>Temporada</th>
                <th>Colección</th>
                <th>Tipo de prenda</th>
                <th>Estado</th>
                <th>Modificado</th>
                <th aria-label="Acciones" />
              </tr>
            </thead>
            <tbody>
              {visible.map((p) => (
                <tr key={p.id}>
                  <td className="mono">{p.code}</td>
                  <td>{p.name}</td>
                  <td>{p.season}</td>
                  <td>{p.collection}</td>
                  <td>{p.garmentType}</td>
                  <td>
                    <select
                      value={p.status}
                      onChange={(e) => changeStatus(p, e.target.value as ProductStatus)}
                      aria-label={`Estado de ${p.code}`}
                    >
                      {STATUS_ORDER.map((s) => (
                        <option key={s} value={s}>
                          {STATUS_LABELS[s]}
                        </option>
                      ))}
                    </select>
                  </td>
                  <td className="muted">{formatDate(p.updatedAt)}</td>
                  <td className="row-actions">
                    <button className="link" onClick={() => setEditing({ mode: "edit", product: p })}>
                      Editar
                    </button>
                    <button className="link danger" onClick={() => remove(p)}>
                      Borrar
                    </button>
                  </td>
                </tr>
              ))}
              {visible.length === 0 && (
                <tr>
                  <td colSpan={8} className="muted center">
                    Ningún producto coincide con «{query}».
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        )}
      </main>

      <footer className="statusbar">
        <span>
          {products.length} {products.length === 1 ? "producto" : "productos"}
        </span>
        <span>{notice ?? "Los cambios se guardan automáticamente."}</span>
      </footer>

      {editing && (
        <ProductForm
          title={editing.mode === "new" ? "Nuevo producto" : "Editar producto"}
          initial={editing.mode === "edit" ? editing.product : undefined}
          existing={products}
          onSubmit={submit}
          onCancel={() => setEditing(null)}
        />
      )}
    </div>
  );
}
