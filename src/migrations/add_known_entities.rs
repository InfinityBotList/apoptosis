use crate::migrations::Migration;

const KNOWN_ENTITIES_TABLE: &str = r#"
CREATE TABLE known_entities (
    target_id TEXT NOT NULL, 
    target_type TEXT NOT NULL,
    PRIMARY KEY (target_id, target_type),

    -- If target_type is 'bot', _bot_fk becomes the ID. If not, it's NULL.
    _bot_fk TEXT GENERATED ALWAYS AS (
        CASE WHEN target_type = 'bot' THEN target_id ELSE NULL END
    ) STORED,

    -- Same with user
    _user_fk TEXT GENERATED ALWAYS AS (
        CASE WHEN target_type = 'user' THEN target_id ELSE NULL END
    ) STORED,

    -- Ensure referential integrity
    CONSTRAINT fk_known_bots 
        FOREIGN KEY (_bot_fk) REFERENCES bots(bot_id) 
        ON DELETE CASCADE,

    CONSTRAINT fk_known_users 
        FOREIGN KEY (_user_fk) REFERENCES users(user_id) 
        ON DELETE CASCADE
)
"#;

pub static MIGRATION: Migration = Migration {
    id: "add_entity_approx_votes",
    description: "Add new entity_approx_votes table",
    up: |pool| {
        Box::pin(async move {
            let mut tx = pool.begin().await?;

            // TODO: Add actual statements here
            let stmts: [&str; _] = [
                KNOWN_ENTITIES_TABLE,
            ];

            for stmt in stmts.iter() {
                sqlx::query(stmt)
                    .execute(&mut *tx)
                    .await?;
            }

            tx.commit().await?;

            Ok(())
        })
    },
};