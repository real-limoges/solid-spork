/*
An entity for Model B. It will be created from the request payload.
 */

pub struct ModelBEntity {
    pub name: String,
}
impl ModelBEntity {
    pub fn new(name: String) -> Self { Self { name } }
}