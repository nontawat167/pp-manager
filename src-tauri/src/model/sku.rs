use rusqlite::Row;
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS, Debug)]
#[ts(export, export_to = "../src/service/types/sku.ts")]
pub struct Sku {
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub name: String,
    #[serde(rename = "type")]
    pub sku_type: String,
    pub price: i64,
}

impl<'a> TryFrom<&'a Row<'a>> for Sku {
    type Error = rusqlite::Error;
    fn try_from(row: &'a Row) -> Result<Self, rusqlite::Error> {
        let sku = Sku {
            id: row.get(0)?,
            created_at: row.get(1)?,
            updated_at: row.get(2)?,
            name: row.get(3)?,
            price: row.get(4)?,
            sku_type: row.get(5)?,
        };
        Ok(sku)
    }
}
