pub mod entities;
pub mod manager;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

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
pub trait Entity {
    /// The full object type for the entity
    type FullObject: Serialize + for<'de> Deserialize<'de> + Send + Sync;
    /// The public object type for the entity
    type PublicObject: Serialize + for<'de> Deserialize<'de> + Send + Sync;
    /// The summary (short form) object type for the entity
    type SummaryObject: Serialize + for<'de> Deserialize<'de> + Send + Sync;

    /// Returns the name of the entity type.
    fn name(&self) -> &'static str;

    /// Returns the CDN folder to use when saving assets for this entity type.
    fn cdn_folder(&self) -> &'static str;

    /// Returns the base flags for the entity type.
    fn flags(&self) -> EntityFlags {
        EntityFlags::NONE
    }

    /// Fetches the entity information for the given ID.
    async fn get_info(&self, id: &str) -> Result<Option<EntityInfo>, crate::Error>;

    /// Returns core vote info about the entity (such as the amount of cooldown time the entity has)
    ///
    /// If user id is specified, then in the future special perks for the user will be returned as well
    ///
    /// If vote time is negative, then it is not possible to revote
    async fn get_vote_info(&self, id: &str, _user_id: Option<&str>) -> Result<Option<EntityVoteInfo>, crate::Error>;

    /// Any entity specific post-vote actions
    async fn post_vote(&self, _id: &str, _user_id: &str) {}

    /// Fetches the full object for the entity
    async fn get_full(&self, id: &str) -> Result<Self::FullObject, crate::Error>;

    /// Fetches the public object for the entity
    async fn get_public(&self, id: &str) -> Result<Self::PublicObject, crate::Error>;

    /// Fetches the summary (short form) object for the entity
    async fn get_summary(&self, _id: &str) -> Result<Self::SummaryObject, crate::Error>;
}

/// Macro to create a enum of entity types
/// 
/// # Example
/// ```ignore
/// entity_enum! {
///     Bot = (BotEntity, "bot" | "bots", FullBotObject, PublicBotObject, SummaryBotObject),
///  }
#[macro_export]
macro_rules! entity_enum {
    ($( $name:ident = ( $entity_type:ty, $matcher:pat, $full_type:ty, $public_type:ty, $summary_type:ty ) ),* $(,)?) => {
        #[allow(dead_code)]
        pub type AnyEntityManager = crate::entity::manager::EntityManager<EntityType>;

        #[derive(Debug)]
        pub enum EntityType {
            $( $name( $entity_type ), )*
        }
        #[allow(unused_variables)]
        impl EntityType {
            /// Creates a new entity type from the given name.
            pub fn from_name(name: &str, pool: sqlx::PgPool) -> Option<Self> {
                match name {
                    $( $matcher => Some(Self::$name(<$entity_type>::new(pool))), )*
                    _ => None,
                }
            }
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum EntityEnumFullObject {
            $( $name( $full_type ), )*
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub enum EntityEnumPublicObject {
            $( $name( $public_type ), )*
        } 
        #[derive(Debug, Serialize, Deserialize)]
        pub enum EntityEnumSummaryObject {
            $( $name( $summary_type ), )*
        }

        #[allow(unused_variables)]
        impl Entity for EntityType {
            type FullObject = EntityEnumFullObject;
            type PublicObject = EntityEnumPublicObject;
            type SummaryObject = EntityEnumSummaryObject;

            fn name(&self) -> &'static str {
                match self {
                    $( Self::$name(n) => n.name(), )*
                }
            }

            fn cdn_folder(&self) -> &'static str {
                match self {
                    $( Self::$name(e) => e.cdn_folder(), )*
                }
            }

            fn flags(&self) -> EntityFlags {
                match self {
                    $( Self::$name(e) => e.flags(), )*
                }
            }

            async fn get_info(&self, id: &str) -> Result<Option<EntityInfo>, crate::Error> {
                match self {
                    $( Self::$name(e) => e.get_info(id).await, )*
                }
            }

            async fn get_vote_info(&self, id: &str, user_id: Option<&str>) -> Result<Option<EntityVoteInfo>, crate::Error> {
                match self {
                    $( Self::$name(e) => e.get_vote_info(id, user_id).await, )*
                }
            }

            async fn post_vote(&self, id: &str, user_id: &str) {
                match self {
                    $( Self::$name(e) => e.post_vote(id, user_id).await, )*
                }
            }

            async fn get_full(&self, id: &str) -> Result<Self::FullObject, crate::Error> {
                match self {
                    $( Self::$name(e) => {
                        let full = e.get_full(id).await?;
                        Ok(EntityEnumFullObject::$name(full))
                    }, )*
                }
            }

            async fn get_public(&self, id: &str) -> Result<Self::PublicObject, crate::Error> {
                match self {
                    $( Self::$name(e) => {
                        let public = e.get_public(id).await?;
                        Ok(EntityEnumPublicObject::$name(public))
                    }, )*
                }
            }

            async fn get_summary(&self, id: &str) -> Result<Self::SummaryObject, crate::Error> {
                match self {
                    $( Self::$name(e) => {
                        let summary = e.get_summary(id).await?;
                        Ok(EntityEnumSummaryObject::$name(summary))
                    }, )*
                }
            }
        }
    };
}

entity_enum! {
    Dummy = (entities::Dummy, "dummy" | "dodo", (), (), ()),
}