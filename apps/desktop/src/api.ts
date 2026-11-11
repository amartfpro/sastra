// Llamadas al núcleo en Rust. Los tipos reflejan los de `sastra-core`.
import { invoke } from "@tauri-apps/api/core";

export type ProductStatus =
  | "draft"
  | "in_development"
  | "sampling"
  | "approved"
  | "archived";

export interface Product {
  id: string;
  code: string;
  name: string;
  season: string;
  collection: string;
  garmentType: string;
  status: ProductStatus;
  createdAt: number;
  updatedAt: number;
}

export interface NewProduct {
  code: string;
  name: string;
  season: string;
  collection: string;
  garmentType: string;
}

export type ProductPatch = Partial<Omit<Product, "id" | "createdAt" | "updatedAt">>;

export interface ProjectInfo {
  path: string;
  name: string;
}

export interface AppInfo {
  version: string;
  formatVersion: number;
}

export interface Change {
  seq: number;
  at: number;
  kind: "create" | "update" | "delete";
  entity: string;
  entityId: string;
  before: Record<string, unknown> | null;
  after: Record<string, unknown> | null;
}

export const api = {
  appInfo: () => invoke<AppInfo>("app_info"),
  newProject: (path: string) => invoke<ProjectInfo>("new_project", { path }),
  openProject: (path: string) => invoke<ProjectInfo>("open_project", { path }),
  closeProject: () => invoke<void>("close_project"),
  projectInfo: () => invoke<ProjectInfo | null>("project_info"),
  listProducts: () => invoke<Product[]>("list_products"),
  createProduct: (input: NewProduct) => invoke<Product>("create_product", { input }),
  updateProduct: (id: string, patch: ProductPatch) =>
    invoke<Product>("update_product", { id, patch }),
  deleteProduct: (id: string) => invoke<void>("delete_product", { id }),
  recentChanges: (limit: number) => invoke<Change[]>("recent_changes", { limit }),
  saveCopy: (path: string) => invoke<void>("save_copy", { path }),
};

/** Convierte cualquier error de `invoke` en un texto para mostrar. */
export function errorMessage(e: unknown): string {
  if (typeof e === "string") return e;
  if (e instanceof Error) return e.message;
  return "Ha ocurrido un error inesperado.";
}
