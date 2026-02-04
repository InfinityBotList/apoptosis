use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
/*
// Represents a session that can be used to authorize/identify a user
type Session struct {
	ID         string      `db:"id" json:"id" description:"The ID of the session"`
	Name       pgtype.Text `db:"name" json:"name,omitempty" description:"The name of the session. Login sessions do not have any names by default"`
	CreatedAt  time.Time   `db:"created_at" json:"created_at" description:"The time the session was created"`
	Type       string      `db:"type" json:"type" description:"The type of session token"`
	TargetType string      `db:"target_type" json:"target_type" description:"The target (entities) type"`
	TargetID   string      `db:"target_id" json:"target_id" description:"The target (entities) ID"`
	PermLimits []string    `db:"perm_limits" json:"perm_limits" description:"The permissions the session has"`
	Expiry     time.Time   `db:"expiry" json:"expiry" description:"The time the session expires"`
}
 */

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// The ID of the session
    pub id: uuid::Uuid,
    /// The name of the session. Login sessions do not have any names by default
    pub name: Option<String>,
    /// The time the session was created
    pub created_at: DateTime<Utc>,
    /// The type of session token
    #[sqlx(rename = "type")]
    pub session_type: String,
    /// The target (entities) type
    pub target_type: String,
    /// The target (entities) ID
    pub target_id: String,
    /// The time the session expires
    pub expiry: DateTime<Utc>,
}