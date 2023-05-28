use crate::model::Sku;
use crate::store::open_close_connection;
use crate::Error;

pub fn find() -> Result<Vec<Sku>, Error> {
    let skus = open_close_connection(|conn| {
        let mut stmt =
            conn.prepare("SELECT id, createdAt, updatedAt, name, price, type FROM sku;")?;
        let rows = stmt.query_map([], |row| Sku::try_from(row))?;
        let mut sku_vec = Vec::new();
        for sku_data in rows {
            sku_vec.push(sku_data?)
        }
        Ok(sku_vec)
    });
    Ok(skus?)
}
