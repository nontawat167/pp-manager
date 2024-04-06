
pub const NEW_ENTITY_ID: &'static str = "NEW-ENTITY-ID";

pub trait Entity {
    fn get_id(&self) -> String;
}