use serde::{Deserialize, Serialize};
use crate::api::DomainConversion;
use crate::domain::entity::{
    model_a_entity::ModelAEntity,
    model_b_entity::ModelBEntity,
};

#[derive(Debug, Deserialize)]
pub struct ModelARequest {
    pub name: String,
}
impl DomainConversion for ModelARequest {
    type DomainEntity = ModelAEntity;
    fn to_domain(&self) -> Result<Self::DomainEntity, String> {
        Ok(ModelAEntity::new(self.name.clone()))
    }
}

#[derive(Debug, Serialize)]
pub struct ModelAResponse {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ModelBRequest {
    pub name: String,
}
impl DomainConversion for ModelBRequest {
    type DomainEntity = ModelBEntity;
    fn to_domain(&self) -> Result<Self::DomainEntity, String> {
        Ok(ModelBEntity::new(self.name.clone()))
    }
}

#[derive(Debug, Serialize)]
pub struct ModelBResponse {
    pub name: String,
}
