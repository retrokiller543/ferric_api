use actix_oauth::types::AccessToken;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use uuid::Uuid;

/// Represents the current user context for database operations
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserContext {
    /// Anonymous user (no login)
    #[default]
    Anonymous,
    /// Authenticated user with their ID
    Authenticated { ext_id: Uuid, token: AccessToken },
    /// System context (bypasses RLS)
    System,
}

impl Display for UserContext {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Anonymous => write!(f, "anonymous"),
            Self::Authenticated { .. } => write!(f, "authenticated([REDACTED])"),
            Self::System => write!(f, "system"),
        }
    }
}

impl UserContext {
    /// Gets the user ID if authenticated
    pub fn user_id(&self) -> Option<Uuid> {
        match self {
            Self::Authenticated { ext_id: id, .. } => Some(*id),
            _ => None,
        }
    }

    pub fn token(&self) -> Option<&AccessToken> {
        match self {
            Self::Authenticated { token, .. } => Some(token),
            _ => None,
        }
    }

    /// Returns true if this is a system context
    pub fn is_system(&self) -> bool {
        matches!(self, Self::System)
    }

    /// Returns true if this is an authenticated user
    pub fn is_authenticated(&self) -> bool {
        matches!(self, Self::Authenticated { .. })
    }
}
