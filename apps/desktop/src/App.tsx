import { useEffect, useState } from "react";
import { api, errorMessage, type AppInfo, type ProjectInfo } from "./api";
import { Welcome } from "./components/Welcome";
import { ProjectView } from "./components/ProjectView";
import "./App.css";

export default function App() {
  const [project, setProject] = useState<ProjectInfo | null>(null);
  const [info, setInfo] = useState<AppInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    api.appInfo().then(setInfo).catch(() => setInfo(null));
    api.projectInfo().then(setProject).catch(() => setProject(null));
  }, []);

  async function close() {
    try {
      await api.closeProject();
      setProject(null);
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  return (
    <div className="app">
      {project ? (
        <ProjectView project={project} onClose={close} />
      ) : (
        <Welcome onOpened={setProject} version={info?.version} />
      )}
      {error && (
        <div className="toast toast-error" role="alert" onClick={() => setError(null)}>
          {error}
        </div>
      )}
    </div>
  );
}
