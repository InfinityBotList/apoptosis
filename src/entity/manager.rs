use crate::{Db, entity::Entity, types::votes::{EntityVote, UserVote}};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

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
    entity: E
}

impl<E: Entity> EntityManager<E> {
    /// Creates a new entity manager instance.
    pub fn new(entity: E, db: Db) -> Self {
        Self { db, entity }
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

    /// Checks whether or not a user has voted for an entity
    pub async fn vote_check(&self, id: &str, user_id: &str) -> Result<UserVote, crate::Error> {
		let vi = self.entity.get_vote_info(id, Some(user_id)).await?;
		let valid_votes = self.fetch_votes(user_id, id, true, None).await?;

		//let mut vw = None;
		// If there is a valid vote in this period and the entity supports multiple votes, figure out how long the user has to wait
		let mut has_voted = false;

		todo!()

        /*
	var vw *types.VoteWait

	// If there is a valid vote in this period and the entity supports multiple votes, figure out how long the user has to wait
	var hasVoted bool

	// Case 1: Multiple votes
	if vi.MultipleVotes {
		if len(validVotes) > 0 {
			// Check if the user has voted in the last vote time
			hasVoted = validVotes[0].CreatedAt.Add(time.Duration(vi.VoteTime) * time.Hour).After(time.Now())

			if hasVoted {
				timeElapsed := time.Since(validVotes[0].CreatedAt)

				timeToWait := int64(vi.VoteTime)*60*60*1000 - timeElapsed.Milliseconds()

				timeToWaitTime := (time.Duration(timeToWait) * time.Millisecond)

				hours := timeToWaitTime / time.Hour
				mins := (timeToWaitTime - (hours * time.Hour)) / time.Minute
				secs := (timeToWaitTime - (hours*time.Hour + mins*time.Minute)) / time.Second

				vw = &types.VoteWait{
					Hours:   int(hours),
					Minutes: int(mins),
					Seconds: int(secs),
				}
			}
		}
	} else {
		// Case 2: Single vote entity
		hasVoted = len(validVotes) > 0
	}

	return &types.UserVote{
		HasVoted:   hasVoted,
		ValidVotes: validVotes,
		VoteInfo:   vi,
		Wait:       vw,
	}, nil
         */
    }
}