use crate::migrations::Migration;

pub static MIGRATION: Migration = Migration {
    id: "add_entity_approx_votes",
    description: "Add new entity_approx_votes table",
    up: |pool| {
        Box::pin(async move {
            let mut tx = pool.begin().await?;

            // TODO: Add actual statements here
            let stmts: [&str; _] = ["CREATE TABLE IF NOT EXISTS entity_approx_votes (target_id TEXT NOT NULL, target_type TEXT NOT NULL, approximate_votes INTEGER NOT NULL DEFAULT 0, PRIMARY KEY (target_id, target_type))"];

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
