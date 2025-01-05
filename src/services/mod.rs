//! Services are the main business logic, if there is need for fetching data from a database we should create a repository
//! for that instead and then use that repository in the service, this is to minimize the amount of logic in each layer.

pub(crate) mod health;
pub(crate) mod oauth;
