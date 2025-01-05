pub(crate) mod oauth_client;
pub(crate) mod oauth_token;
pub(crate) mod user;

pub(crate) trait Model {
    type Id;

    fn get_id(&self) -> Option<Self::Id>;
}
