use crate::entity::{Entity, EntityFlags, EntityInfo};

#[derive(Debug)]
pub struct Dummy {}

impl Dummy {
    /// Creates a new instance of the Dummy entity.
    pub fn new(_pool: sqlx::PgPool) -> Self {
        Self {}
    }
}

impl Entity for Dummy {
    type FullObject = ();
    type PublicObject = ();
    type SummaryObject = ();

    fn name(&self) -> &'static str {
        "Dummy"
    }

    fn target_type(&self) -> &'static str {
        "dummy"
    }

    fn cdn_folder(&self) -> &'static str {
        "dummys"
    }

    fn flags(&self) -> EntityFlags {
        EntityFlags::empty()
    }

    async fn get_info(&self, _id: &str) -> Result<Option<EntityInfo>, crate::Error> {
        Ok(None)
    }

    async fn get_full(&self, _id: &str) -> Result<Self::FullObject, crate::Error> {
        Ok(())
    }

    async fn get_public(&self, _id: &str) -> Result<Self::PublicObject, crate::Error> {
        Ok(())
    }

    async fn get_summary(&self, _id: &str) -> Result<Self::SummaryObject, crate::Error> {
        Ok(())
    }
}