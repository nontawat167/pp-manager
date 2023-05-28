use crate::model::Sku;
use crate::store::find;
use crate::Result;

pub fn get_all_sku() -> Result<Vec<Sku>> {
    find()
}
