mod dtos;
mod handlers;
mod router;

pub use router::run_server;

trait DomainConversion {
    type DomainEntity;
    fn to_domain(&self) -> Result<Self::DomainEntity, String>;
    
}