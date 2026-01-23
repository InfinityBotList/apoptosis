use crate::migrations::Migration;

pub static MIGRATION: Migration = Migration {
    id: "add_pkeys",
    description: "Add better primary keys",
    up: |pool| {
        Box::pin(async move {
            let mut tx = pool.begin().await?;

            // TODO: Add actual statements here
            let stmts: [&str; _] = ["alter table entity_votes add primary key (itag)"];

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
