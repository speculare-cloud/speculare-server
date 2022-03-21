use uuid::Uuid;

pub mod config;

#[derive(Clone, Debug)]
pub struct InnerUser {
    pub uuid: Uuid,
}
