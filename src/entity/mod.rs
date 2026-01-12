pub mod entities;
pub mod manager;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serenity::async_trait;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct EntityFlags: u32 {
        const NONE = 0;
        /// The entity supports having webhooks attached to it.
        const SUPPORTS_WEBHOOKS = 1 << 0;
        /// The entity supports voting.
        const SUPPORTS_VOTING = 1 << 1;
        /// Whether or not the entity supports multiple votes as opposed to single vote only
        const SUPPORTS_MULTIPLE_VOTES = 1 << 2;
        /// Whether or not the entity supports upvotes
        const SUPPORTS_UPVOTES = 1 << 3;
        /// Whether or not the entity supports downvotes
        const SUPPORTS_DOWNVOTES = 1 << 4;
        /// Whether or not the entity supports vote credits
        const SUPPORTS_VOTE_CREDITS = 1 << 5;
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Base information about an entity.
pub struct EntityInfo {
    pub name: String,
    pub url: String,
    pub vote_url: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityVoteInfo {
    /// The amount of votes a single vote creates on this entity
    /// 
    /// TODO: Rename this field in the future maybe?
    pub per_user: u32,

    /// The amount of time in hours until a usser can vote again
    pub vote_time: u32,
}

#[allow(async_fn_in_trait)]
#[async_trait]
pub trait Entity {
    /// Creates a new entity instance.
    fn new(pool: sqlx::PgPool) -> Self where Self: Sized;

    /// Returns the name of the entity type.
    fn name(&self) -> &'static str;

    /// Returns the base flags for the entity type.
    fn flags(&self) -> EntityFlags {
        EntityFlags::NONE
    }

    /// Fetches the entity information for the given ID.
    async fn get_info(&self, id: &str) -> Option<EntityInfo>;

    /// Returns core vote info about the entity (such as the amount of cooldown time the entity has)
    ///
    /// If user id is specified, then in the future special perks for the user will be returned as well
    ///
    /// If vote time is negative, then it is not possible to revote
    async fn get_vote_info(&self, id: &str, _user_id: Option<&str>) -> Option<EntityVoteInfo>;

    /// Any entity specific post-vote actions
    async fn post_vote(&self, _id: &str, _user_id: &str) {}
}

/// Asserts that `dyn Entity` is object safe.
mod assert_entity_dyn {
    use super::Entity;

    fn _assert_entity_dyn<T: Entity + ?Sized>() {}

    #[allow(dead_code)]
    pub fn assert_entity_dyn_impl() {
        _assert_entity_dyn::<dyn Entity>();
    }
}