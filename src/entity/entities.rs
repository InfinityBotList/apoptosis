use crate::entity::{Entity, EntityFlags, EntityInfo};

#[derive(Debug, Clone)]
pub struct Dummy {
    pool: sqlx::PgPool,
    diesel: crate::Db,
}

impl Dummy {
    /// Creates a new instance of the Dummy entity.
    pub fn new(pool: sqlx::PgPool, diesel: crate::Db) -> Self {
        Self { pool, diesel }
    }
}

impl Entity for Dummy {
    type FullObject = ();
    type PublicObject = ();
    type SummaryObject = ();

    fn pool(&self) -> &sqlx::PgPool {
        &self.pool
    }

    fn diesel(&self) -> &crate::Db {
        &self.diesel
    }

    fn name(&self) -> &'static str {
        "Dummy"
    }

    fn target_type(&self) -> &'static str {
        "dummy"
    }

    fn cdn_folder(&self) -> &'static str {
        "dummys"
    }

    async fn flags(&self, _id: &str) -> Result<EntityFlags, crate::Error> {
        Ok(EntityFlags::empty())
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