use crate::entity::{Entity, EntityFlags, EntityInfo, EntityVoteInfo};

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

    fn name(&self) -> &'static str {
        "Dummy"
    }

    fn flags(&self) -> EntityFlags {
        EntityFlags::empty()
    }

    async fn get_info(&self, _id: &str) -> Result<Option<EntityInfo>, crate::Error> {
        Ok(None)
    }

    async fn get_vote_info(&self, _id: &str, _user_id: Option<&str>) -> Result<Option<EntityVoteInfo>, crate::Error> {
        Ok(None)
    }

    async fn get_full(&self, _id: &str) -> Result<Self::FullObject, crate::Error> {
        Ok(())
    }

    async fn get_public(&self, _id: &str) -> Result<Self::PublicObject, crate::Error> {
        Ok(())
    }
}