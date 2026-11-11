//! Registro de cambios: cada modificación del modelo guarda cómo estaba antes y
//! cómo queda después. De aquí saldrán el deshacer, el historial de versiones de
//! la ficha y la sincronización (ver especificación, sección 6).

use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Transaction, params};
use serde::{Deserialize, Serialize};

use crate::{Project, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    Create,
    Update,
    Delete,
}

impl ChangeKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Update => "update",
            Self::Delete => "delete",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        match s {
            "create" => Some(Self::Create),
            "update" => Some(Self::Update),
            "delete" => Some(Self::Delete),
            _ => None,
        }
    }
}

/// Una entrada del registro de cambios.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub seq: i64,
    /// Milisegundos desde 1970 (UTC).
    pub at: i64,
    pub kind: ChangeKind,
    /// Tipo de entidad: `product`, y más adelante `bom_line`, `pom`…
    pub entity: String,
    pub entity_id: String,
    pub before: Option<serde_json::Value>,
    pub after: Option<serde_json::Value>,
}

pub(crate) fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

pub(crate) fn record<T: Serialize>(
    tx: &Transaction<'_>,
    kind: ChangeKind,
    entity: &str,
    entity_id: &str,
    before: Option<&T>,
    after: Option<&T>,
) -> Result<()> {
    let before = before.map(serde_json::to_string).transpose()?;
    let after = after.map(serde_json::to_string).transpose()?;
    tx.execute(
        "INSERT INTO changes (at, kind, entity, entity_id, before, after)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            now_millis(),
            kind.as_str(),
            entity,
            entity_id,
            before,
            after
        ],
    )?;
    Ok(())
}

impl Project {
    /// Últimos cambios del proyecto, del más reciente al más antiguo.
    pub fn recent_changes(&self, limit: usize) -> Result<Vec<Change>> {
        let mut stmt = self.conn.prepare(
            "SELECT seq, at, kind, entity, entity_id, before, after
             FROM changes ORDER BY seq DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit as i64], |row| {
            let kind: String = row.get(2)?;
            let before: Option<String> = row.get(5)?;
            let after: Option<String> = row.get(6)?;
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                kind,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                before,
                after,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (seq, at, kind, entity, entity_id, before, after) = row?;
            out.push(Change {
                seq,
                at,
                kind: ChangeKind::parse(&kind).unwrap_or(ChangeKind::Update),
                entity,
                entity_id,
                before: before.map(|s| serde_json::from_str(&s)).transpose()?,
                after: after.map(|s| serde_json::from_str(&s)).transpose()?,
            });
        }
        Ok(out)
    }
}
