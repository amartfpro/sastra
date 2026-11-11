import { useState } from "react";
import { open, save } from "@tauri-apps/plugin-dialog";
import { api, errorMessage, type ProjectInfo } from "../api";

const FILTERS = [{ name: "Proyecto de Sastra", extensions: ["sastra"] }];

interface Props {
  onOpened: (project: ProjectInfo) => void;
  version?: string;
}

export function Welcome({ onOpened, version }: Props) {
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function run(action: () => Promise<ProjectInfo | null>) {
    setError(null);
    setBusy(true);
    try {
      const project = await action();
      if (project) onOpened(project);
    } catch (e) {
      setError(errorMessage(e));
    } finally {
      setBusy(false);
    }
  }

  const createProject = () =>
    run(async () => {
      const path = await save({ title: "Nuevo proyecto", filters: FILTERS, defaultPath: "Coleccion.sastra" });
      return path ? api.newProject(path) : null;
    });

  const openProject = () =>
    run(async () => {
      const path = await open({ title: "Abrir proyecto", filters: FILTERS, multiple: false, directory: false });
      return typeof path === "string" ? api.openProject(path) : null;
    });

  return (
    <main className="welcome">
      <div className="welcome-card">
        <h1 className="brand">Sastra</h1>
        <p className="tagline">Patronaje, tallaje y fichas técnicas desde un único modelo de la prenda.</p>

        <div className="welcome-actions">
          <button className="primary" onClick={createProject} disabled={busy}>
            Nuevo proyecto…
          </button>
          <button onClick={openProject} disabled={busy}>
            Abrir proyecto…
          </button>
        </div>

        <p className="hint">
          Un proyecto es un archivo <code>.sastra</code> que agrupa tus productos. Los cambios se guardan solos.
        </p>
        {error && (
          <p className="error" role="alert">
            {error}
          </p>
        )}
      </div>
      {version && <footer className="version">Versión {version}</footer>}
    </main>
  );
}
