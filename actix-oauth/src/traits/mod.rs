//! OAuth2 handler traits and core functionality.
//!
//! This module contains the core traits required to implement the various
//! OAuth2 grant types and authorization flows. Each trait corresponds to a
//! standard OAuth2 grant type or endpoint as defined in RFC 6749.
//!
//! # Architecture
//!
//! The OAuth2 implementation follows a trait-based design where each grant type
//! is represented by a separate trait. This allows for flexible composition and
//! customization of OAuth2 servers. The main components are:
//!
//! * [`OAuth2Manager`] - Core trait that coordinates all OAuth2 functionality
//! * [`PasswordHandler`] - Handles resource owner password credentials grant
//! * [`AuthCodeHandler`] - Handles authorization code grant
//! * [`ClientCredentialsHandler`] - Handles client credentials grant
//! * [`RefreshTokenHandler`] - Handles refresh token requests
//! * [`AuthorizationHandler`] - Handles authorization endpoint requests
#![allow(dead_code, async_fn_in_trait)]

mod auth_code_handler;
mod authorization_handler;
mod client_credentials_handler;
mod manager;
mod password_handler;
mod refresh_token_handler;

pub use auth_code_handler::*;
pub use authorization_handler::*;
pub use client_credentials_handler::*;
pub use manager::*;
pub use password_handler::*;
pub use refresh_token_handler::*;
