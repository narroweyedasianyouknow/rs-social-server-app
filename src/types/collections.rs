use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Collections {
    pub(crate) users: String, // Изменили тип поля на String
}

lazy_static! {
    pub static ref COLLECTION_NAMES: Collections = Collections {
        users: "users".to_string(),
    };
}
