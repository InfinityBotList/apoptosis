use crate::{Db, entity::{Entity, EntityFlags}, types::votes::{EntityVote, UserVote, VoteInfo}};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use sqlx::PgPool;

diesel::table! {
    entity_votes (itag) {
        itag -> Uuid,
        target_id -> Text,
        target_type -> Text,
        author -> Text,
        upvote -> Bool,
        void -> Bool,
        void_reason -> Nullable<Text>,
        created_at -> Timestamptz,
        vote_num -> Int4,
        voided_at -> Nullable<Timestamptz>,
        immutable -> Bool,
        credit_redeem -> Nullable<Uuid>,
    }
}

pub struct EntityManager<E: Entity> {
    db: Db,
	pool: PgPool,
    entity: E
}

impl<E: Entity> EntityManager<E> {
    /// Creates a new entity manager instance.
    pub fn new(entity: E, db: Db, pool: PgPool) -> Self {
        Self { pool, db, entity }
    }

    /// Returns a reference to the entity instance.
    pub fn entity(&self) -> &E {
        &self.entity
    }

	/// Fetches all votes for a given user and entity.
	/// 
	/// This always returns in created_at descending order (i.e. newest votes first).
	pub async fn fetch_votes(
		&self,
		user_id: &str,
		id: &str,
		only_valid: bool, // whether or not to only fetch non-void votes
		limit_offset: Option<(u32, u32)>, // (limit, offset)
	) -> Result<Vec<EntityVote>, crate::Error> {
		
		let mut base_query_dyn = entity_votes::table
		.select((
			entity_votes::itag,
			entity_votes::target_id,
			entity_votes::target_type,
			entity_votes::author,
			entity_votes::upvote,
			entity_votes::void,
			entity_votes::void_reason,
			entity_votes::voided_at,
			entity_votes::created_at,
			entity_votes::vote_num,
			entity_votes::immutable,
		))
		.filter(
			entity_votes::author.eq(user_id)
			.and(entity_votes::target_id.eq(id))
			.and(entity_votes::target_type.eq(self.entity.target_type()))
		)
		.into_boxed();

		if let Some((limit, offset)) = limit_offset {
			base_query_dyn = base_query_dyn.limit(limit as i64).offset(offset as i64);
		}
		
		if only_valid {
			base_query_dyn = base_query_dyn.filter(entity_votes::void.eq(false));
		}

		base_query_dyn = base_query_dyn.order(entity_votes::created_at.desc());

		let mut conn = self.db.get().await?;
		let results = base_query_dyn.load::<EntityVote>(&mut conn).await?;
		Ok(results)
	}

	/// Helper method to get full vote info for an entity, wrapping the underlying entity's get_vote_info method and adding flag info.
	pub async fn get_full_vote_info(&self, id: &str, user_id: Option<&str>) -> Result<VoteInfo, crate::Error> {
		let vi = self.entity.get_vote_info(id, user_id).await?;

			Ok(VoteInfo {
				per_user: vi.per_user,
				vote_time: vi.vote_time as u16,
				vote_credits: self.entity.flags().contains(EntityFlags::SUPPORTS_VOTE_CREDITS),
				multiple_votes: self.entity.flags().contains(EntityFlags::SUPPORTS_MULTIPLE_VOTES),
				supports_upvotes: self.entity.flags().contains(EntityFlags::SUPPORTS_UPVOTES),
				supports_downvotes: self.entity.flags().contains(EntityFlags::SUPPORTS_DOWNVOTES),
			})
	}

    /// Checks whether or not a user has voted for an entity
    pub async fn vote_check(&self, id: &str, user_id: &str) -> Result<UserVote, crate::Error> {
		let vi = self.get_full_vote_info(id, Some(user_id)).await?;
		let valid_votes = self.fetch_votes(user_id, id, true, None).await?;

		let mut vote_wait = None;
		// If there is a valid vote in this period and the entity supports multiple votes, figure out how long the user has to wait
		let mut has_voted = false;

		// Case 1: Multiple votes
		if vi.multiple_votes {
			if let Some(last_vote) = valid_votes.iter().next() {
				// Check if the user has voted in the last vote time
				has_voted = last_vote.created_at + chrono::Duration::hours(vi.vote_time as i64) > chrono::Utc::now();

				if has_voted {
					let time_elapsed = chrono::Utc::now() - last_vote.created_at;
					let time_to_wait = chrono::Duration::hours(vi.vote_time as i64) - time_elapsed;
					let hours = time_to_wait.num_hours();
					let minutes = time_to_wait.num_minutes() - (hours * 60);
					let seconds = time_to_wait.num_seconds() - (hours * 3600 + minutes * 60);

					vote_wait = Some(crate::types::votes::VoteWait {
						hours: hours as i32,
						minutes: minutes as i32,
						seconds: seconds as i32,
					});
				}
			}
		} else {
			// Case 2: Single vote entity
			has_voted = !valid_votes.is_empty();
		}

		Ok(UserVote {
			has_voted,
			valid_votes,
			vote_info: vi,
			wait: vote_wait,
		})
    }

	/// Returns the exact (non-cached/approximate) vote count for an entity
	pub async fn exact_vote_count(&self, id: &str, user_id: &str) -> Result<i64, crate::Error> {
		#[derive(sqlx::FromRow)]
		struct VoteCount {
			count: i64,
		}

		let upvotes: VoteCount = sqlx::query_as::<_, VoteCount>(
			"SELECT COUNT(*) FROM entity_votes WHERE target_id = $1 AND target_type = $2 AND void = false AND upvote = true"
		)
		.bind(id)
		.bind(self.entity.target_type())
		.bind(user_id)
		.fetch_one(&self.pool)
		.await?;

		let downvotes: VoteCount = sqlx::query_as::<_, VoteCount>(
			"SELECT COUNT(*) FROM entity_votes WHERE target_id = $1 AND target_type = $2 AND void = false AND upvote = false"
		)
		.bind(id)
		.bind(self.entity.target_type())
		.bind(user_id)
		.fetch_one(&self.pool)
		.await?;

		Ok(upvotes.count - downvotes.count)
	}

	/// Helper function to give votes to an entity 
	pub async fn give_votes(&self, id: &str, user_id: &str, upvote: bool) -> Result<(), crate::Error> {
		let vi = self.get_full_vote_info(id, Some(user_id)).await?;
		
		let mut tx = self.pool.begin().await?;

		// Keep adding votes until, but not including vote_info.per_user
		for i in 0..vi.per_user {
			sqlx::query(
				"INSERT INTO entity_votes (author, target_id, target_type, upvote, vote_num) VALUES ($1, $2, $3, $4, $5)",
			)
			.bind(user_id)
			.bind(id)
			.bind(self.entity.target_type())
			.bind(upvote)
			.bind(i as i32)
			.execute(&mut *tx)
			.await?;
		}

		// Update entity_approx_votes table
		sqlx::query(
			"INSERT INTO entity_approx_votes (target_id, target_type, approximate_votes) VALUES ($1, $2, $3)
			ON CONFLICT (target_id, target_type) DO UPDATE SET approximate_votes = entity_approx_votes.approximate_votes + EXCLUDED.approximate_votes",
		)
		.bind(id)
		.bind(self.entity.target_type())
		.bind(if upvote { vi.per_user as i64 } else { -(vi.per_user as i64) })
		.execute(&mut *tx)
		.await?;

		tx.commit().await?;

		Ok(())
	 }
}