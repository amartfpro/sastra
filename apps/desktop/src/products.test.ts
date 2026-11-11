import { describe, expect, it } from "vitest";
import type { Product } from "./api";
import { EMPTY_PRODUCT, filterProducts, validateProduct } from "./products";

const camiseta: Product = {
  id: "1",
  code: "SS27-TS-001",
  name: "Camiseta básica",
  season: "SS27",
  collection: "Esenciales",
  garmentType: "Camiseta",
  status: "draft",
  createdAt: 0,
  updatedAt: 0,
};

describe("validateProduct", () => {
  it("exige código y nombre", () => {
    const errors = validateProduct(EMPTY_PRODUCT, []);
    expect(errors.code).toBeDefined();
    expect(errors.name).toBeDefined();
  });

  it("detecta códigos repetidos sin distinguir mayúsculas", () => {
    const errors = validateProduct({ ...EMPTY_PRODUCT, code: "ss27-ts-001", name: "Otra" }, [camiseta]);
    expect(errors.code).toMatch(/Ya existe/);
  });

  it("permite conservar el propio código al editar", () => {
    const errors = validateProduct({ ...camiseta }, [camiseta], camiseta.id);
    expect(errors).toEqual({});
  });
});

describe("filterProducts", () => {
  it("busca sin distinguir acentos", () => {
    expect(filterProducts([camiseta], "basica")).toHaveLength(1);
    expect(filterProducts([camiseta], "ESENCIALES")).toHaveLength(1);
    expect(filterProducts([camiseta], "pantalón")).toHaveLength(0);
  });
});
