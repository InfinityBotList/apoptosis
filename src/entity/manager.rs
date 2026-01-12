use crate::entity::Entity;

pub struct EntityManager<E: Entity> {
    pool: sqlx::PgPool,
    entity: E
}

impl<E: Entity> EntityManager<E> {
    /// Creates a new entity manager instance.
    pub fn new(pool: sqlx::PgPool) -> Self {
        let entity = E::new(pool.clone());
        Self { pool, entity }
    }

    /// Returns a reference to the entity instance.
    pub fn entity(&self) -> &E {
        &self.entity
    }

    /// Checks whether or not a user has voted for an entity
    pub async fn vote_check(&self) {
        /*
	vi, err := EntityVoteInfo(ctx, c, targetId, targetType)

	if err != nil {
		return nil, err
	}

	var rows pgx.Rows

	rows, err = c.Query(
		ctx,
		"SELECT "+entityVoteCols+" FROM entity_votes WHERE author = $1 AND target_id = $2 AND target_type = $3 AND void = false ORDER BY created_at DESC",
		userId,
		targetId,
		targetType,
	)

	if err != nil {
		return nil, err
	}

	validVotes, err := pgx.CollectRows(rows, pgx.RowToStructByName[types.EntityVote])

	if errors.Is(err, pgx.ErrNoRows) {
		validVotes = []types.EntityVote{}
	} else if err != nil {
		return nil, err
	}

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