pub mod database;
mod sku;
pub mod schema;

trait Store<T> {
    fn find() -> T;
}
