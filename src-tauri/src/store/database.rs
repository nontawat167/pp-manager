use rusqlite::{params, Connection, Result};
use uuid::Uuid;

pub fn prepare_database() -> Result<()> {
    open_close_connection(|conn| {
        let sql = "CREATE TABLE IF NOT EXISTS sku (
            id varchar(40) not null unique primary key,
            createdAt DATETIME not null,
            updatedAt DATETIME not null,
            name text not null,
            price int not null,
            type varchar(10) not null
        )"
        .to_string();
        conn.execute(&sql, ())?;

        let id = Uuid::new_v4();
        conn.execute(
            "INSERT INTO sku (id, createdAt, updatedAt, name, price, type) 
        values (?1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?2, ?3, ?4);",
            params![
                id.to_string(),
                String::from("testname"),
                100i32,
                String::from("MAX")
            ],
        )?;

        Ok(())
    })?;
    Ok(())
}

pub fn open_close_connection<T, CB>(cb: CB) -> Result<T>
where
    CB: Fn(&Connection) -> Result<T>,
{
    let conn = Connection::open("D:\\test.db")?;
    let result = cb(&conn);
    conn.close().ok();
    result
}
