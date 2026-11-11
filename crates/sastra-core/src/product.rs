//! Producto (estilo): la raíz del modelo de datos de la prenda (especificación, sección 5).

use rusqlite::{OptionalExtension, Row, params};
use serde::{Deserialize, Serialize};

use crate::changes::{ChangeKind, now_millis, record};
use crate::{Error, Project, Result};

const MAX_CODE_LEN: usize = 64;
const MAX_TEXT_LEN: usize = 200;

/// Estado del producto en su desarrollo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductStatus {
    #[default]
    Draft,
    InDevelopment,
    Sampling,
    Approved,
    Archived,
}

impl ProductStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::InDevelopment => "in_development",
            Self::Sampling => "sampling",
            Self::Approved => "approved",
            Self::Archived => "archived",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "draft" => Some(Self::Draft),
            "in_development" => Some(Self::InDevelopment),
            "sampling" => Some(Self::Sampling),
            "approved" => Some(Self::Approved),
            "archived" => Some(Self::Archived),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: String,
    pub code: String,
    pub name: String,
    pub season: String,
    pub collection: String,
    pub garment_type: String,
    pub status: ProductStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Datos para crear un producto.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NewProduct {
    pub code: String,
    pub name: String,
    pub season: String,
    pub collection: String,
    pub garment_type: String,
}

/// Cambios parciales de un producto: solo se modifican los campos presentes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProductPatch {
    pub code: Option<String>,
    pub name: Option<String>,
    pub season: Option<String>,
    pub collection: Option<String>,
    pub garment_type: Option<String>,
    pub status: Option<ProductStatus>,
}

fn clean_required(value: &str, field: &str, max: usize) -> Result<String> {
    let v = value.trim();
    if v.is_empty() {
        return Err(Error::Invalid(format!("el campo «{field}» es obligatorio")));
    }
    clean_optional(v, field, max)
}

fn clean_optional(value: &str, field: &str, max: usize) -> Result<String> {
    let v = value.trim();
    if v.chars().count() > max {
        return Err(Error::Invalid(format!(
            "el campo «{field}» admite como máximo {max} caracteres"
        )));
    }
    Ok(v.to_string())
}

fn from_row(row: &Row<'_>) -> rusqlite::Result<Product> {
    let status: String = row.get("status")?;
    Ok(Product {
        id: row.get("id")?,
        code: row.get("code")?,
        name: row.get("name")?,
        season: row.get("season")?,
        collection: row.get("collection")?,
        garment_type: row.get("garment_type")?,
        status: ProductStatus::parse(&status).unwrap_or_default(),
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

const SELECT: &str = "SELECT id, code, name, season, collection, garment_type, status, created_at, updated_at FROM products";

impl Project {
    /// Todos los productos, del modificado más recientemente al más antiguo.
    pub fn list_products(&self) -> Result<Vec<Product>> {
        let mut stmt = self
            .conn
            .prepare(&format!("{SELECT} ORDER BY updated_at DESC, code ASC"))?;
        let rows = stmt.query_map([], from_row)?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }

    pub fn get_product(&self, id: &str) -> Result<Product> {
        self.conn
            .query_row(&format!("{SELECT} WHERE id = ?1"), [id], from_row)
            .optional()?
            .ok_or_else(|| Error::ProductNotFound(id.to_string()))
    }

    fn ensure_code_free(&self, code: &str, except_id: &str) -> Result<()> {
        let taken: bool = self.conn.query_row(
            "SELECT EXISTS (SELECT 1 FROM products WHERE code = ?1 AND id != ?2)",
            params![code, except_id],
            |row| row.get(0),
        )?;
        if taken {
            return Err(Error::DuplicateCode(code.to_string()));
        }
        Ok(())
    }

    pub fn create_product(&mut self, input: NewProduct) -> Result<Product> {
        let now = now_millis();
        let product = Product {
            id: uuid::Uuid::now_v7().to_string(),
            code: clean_required(&input.code, "código", MAX_CODE_LEN)?,
            name: clean_required(&input.name, "nombre", MAX_TEXT_LEN)?,
            season: clean_optional(&input.season, "temporada", MAX_TEXT_LEN)?,
            collection: clean_optional(&input.collection, "colección", MAX_TEXT_LEN)?,
            garment_type: clean_optional(&input.garment_type, "tipo de prenda", MAX_TEXT_LEN)?,
            status: ProductStatus::Draft,
            created_at: now,
            updated_at: now,
        };
        self.ensure_code_free(&product.code, &product.id)?;

        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT INTO products (id, code, name, season, collection, garment_type, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                product.id,
                product.code,
                product.name,
                product.season,
                product.collection,
                product.garment_type,
                product.status.as_str(),
                product.created_at,
                product.updated_at
            ],
        )?;
        record(
            &tx,
            ChangeKind::Create,
            "product",
            &product.id,
            None,
            Some(&product),
        )?;
        tx.commit()?;
        Ok(product)
    }

    pub fn update_product(&mut self, id: &str, patch: ProductPatch) -> Result<Product> {
        let before = self.get_product(id)?;
        let mut after = before.clone();
        if let Some(v) = patch.code {
            after.code = clean_required(&v, "código", MAX_CODE_LEN)?;
        }
        if let Some(v) = patch.name {
            after.name = clean_required(&v, "nombre", MAX_TEXT_LEN)?;
        }
        if let Some(v) = patch.season {
            after.season = clean_optional(&v, "temporada", MAX_TEXT_LEN)?;
        }
        if let Some(v) = patch.collection {
            after.collection = clean_optional(&v, "colección", MAX_TEXT_LEN)?;
        }
        if let Some(v) = patch.garment_type {
            after.garment_type = clean_optional(&v, "tipo de prenda", MAX_TEXT_LEN)?;
        }
        if let Some(v) = patch.status {
            after.status = v;
        }
        if after == before {
            return Ok(before);
        }
        self.ensure_code_free(&after.code, id)?;
        after.updated_at = now_millis().max(before.updated_at + 1);

        let tx = self.conn.transaction()?;
        tx.execute(
            "UPDATE products SET code = ?2, name = ?3, season = ?4, collection = ?5,
                 garment_type = ?6, status = ?7, updated_at = ?8
             WHERE id = ?1",
            params![
                id,
                after.code,
                after.name,
                after.season,
                after.collection,
                after.garment_type,
                after.status.as_str(),
                after.updated_at
            ],
        )?;
        record(
            &tx,
            ChangeKind::Update,
            "product",
            id,
            Some(&before),
            Some(&after),
        )?;
        tx.commit()?;
        Ok(after)
    }

    pub fn delete_product(&mut self, id: &str) -> Result<()> {
        let before = self.get_product(id)?;
        let tx = self.conn.transaction()?;
        tx.execute("DELETE FROM products WHERE id = ?1", [id])?;
        record(&tx, ChangeKind::Delete, "product", id, Some(&before), None)?;
        tx.commit()?;
        Ok(())
    }
}
