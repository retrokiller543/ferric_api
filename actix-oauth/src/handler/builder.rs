use super::OAuth2Handler;
use crate::handler::default::{
    NotImplementedAuthCodeHandler, NotImplementedAuthorizationHandler,
    NotImplementedClientCredentialsHandler, NotImplementedPasswordHandler,
    NotImplementedRefreshTokenHandler,
};

use crate::traits::*;

pub struct OAuth2HandlerBuilder<
    PH = NotImplementedPasswordHandler,
    AH = NotImplementedAuthCodeHandler,
    CH = NotImplementedClientCredentialsHandler,
    RH = NotImplementedRefreshTokenHandler,
    AuthH = NotImplementedAuthorizationHandler,
> where
    PH: PasswordHandler,
    AH: AuthCodeHandler,
    CH: ClientCredentialsHandler,
    RH: RefreshTokenHandler,
    AuthH: AuthorizationHandler,
{
    password_grant_handler: PH,
    authorization_code_grant_handler: AH,
    client_credentials_grant_handler: CH,
    refresh_token_handler: RH,
    authorization_handler: AuthH,
}

impl OAuth2HandlerBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<PH, AH, CH, RH, AuthH> OAuth2HandlerBuilder<PH, AH, CH, RH, AuthH>
where
    PH: PasswordHandler,
    AH: AuthCodeHandler,
    CH: ClientCredentialsHandler,
    RH: RefreshTokenHandler,
    AuthH: AuthorizationHandler,
{
    pub fn password_handler<NewPH>(
        self,
        handler: NewPH,
    ) -> OAuth2HandlerBuilder<NewPH, AH, CH, RH, AuthH>
    where
        NewPH: PasswordHandler,
    {
        OAuth2HandlerBuilder {
            password_grant_handler: handler,
            authorization_code_grant_handler: self.authorization_code_grant_handler,
            client_credentials_grant_handler: self.client_credentials_grant_handler,
            refresh_token_handler: self.refresh_token_handler,
            authorization_handler: self.authorization_handler,
        }
    }

    pub fn authorization_code_handler<NewAH>(
        self,
        handler: NewAH,
    ) -> OAuth2HandlerBuilder<PH, NewAH, CH, RH, AuthH>
    where
        NewAH: AuthCodeHandler,
    {
        OAuth2HandlerBuilder {
            password_grant_handler: self.password_grant_handler,
            authorization_code_grant_handler: handler,
            client_credentials_grant_handler: self.client_credentials_grant_handler,
            refresh_token_handler: self.refresh_token_handler,
            authorization_handler: self.authorization_handler,
        }
    }

    pub fn client_credentials_handler<NewCH>(
        self,
        handler: NewCH,
    ) -> OAuth2HandlerBuilder<PH, AH, NewCH, RH, AuthH>
    where
        NewCH: ClientCredentialsHandler,
    {
        OAuth2HandlerBuilder {
            password_grant_handler: self.password_grant_handler,
            authorization_code_grant_handler: self.authorization_code_grant_handler,
            client_credentials_grant_handler: handler,
            refresh_token_handler: self.refresh_token_handler,
            authorization_handler: self.authorization_handler,
        }
    }

    pub fn refresh_handler<NewRH>(
        self,
        handler: NewRH,
    ) -> OAuth2HandlerBuilder<PH, AH, CH, NewRH, AuthH>
    where
        NewRH: RefreshTokenHandler,
    {
        OAuth2HandlerBuilder {
            password_grant_handler: self.password_grant_handler,
            authorization_code_grant_handler: self.authorization_code_grant_handler,
            client_credentials_grant_handler: self.client_credentials_grant_handler,
            refresh_token_handler: handler,
            authorization_handler: self.authorization_handler,
        }
    }

    pub fn authorization_handler<NewAuthH>(
        self,
        handler: NewAuthH,
    ) -> OAuth2HandlerBuilder<PH, AH, CH, RH, NewAuthH>
    where
        NewAuthH: AuthorizationHandler,
    {
        OAuth2HandlerBuilder {
            password_grant_handler: self.password_grant_handler,
            authorization_code_grant_handler: self.authorization_code_grant_handler,
            client_credentials_grant_handler: self.client_credentials_grant_handler,
            refresh_token_handler: self.refresh_token_handler,
            authorization_handler: handler,
        }
    }

    pub fn build(self) -> OAuth2Handler<PH, AH, CH, RH, AuthH> {
        OAuth2Handler {
            password_grant_handler: self.password_grant_handler,
            authorization_code_grant_handler: self.authorization_code_grant_handler,
            client_credentials_grant_handler: self.client_credentials_grant_handler,
            refresh_token_handler: self.refresh_token_handler,
            authorization_handler: self.authorization_handler,
        }
    }
}

impl Default for OAuth2HandlerBuilder {
    fn default() -> Self {
        Self {
            password_grant_handler: NotImplementedPasswordHandler,
            authorization_code_grant_handler: NotImplementedAuthCodeHandler,
            client_credentials_grant_handler: NotImplementedClientCredentialsHandler,
            refresh_token_handler: NotImplementedRefreshTokenHandler,
            authorization_handler: NotImplementedAuthorizationHandler,
        }
    }
}
