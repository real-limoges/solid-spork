/*
An entity for Model A. It will be created from the request payload.
 */

pub struct ModelAEntity {
    pub name: String,
}
impl ModelAEntity {
    pub fn new(name: String) -> Self { Self { name } }
}