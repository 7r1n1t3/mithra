use std::future::{Ready, ready};

use actix_session::SessionExt;
use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, error::ErrorUnauthorized};

use crate::auth::models::{ID, Session};

pub struct AuthedUser {
    pub id: ID,
}

impl FromRequest for AuthedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(
            req.get_session()
                .get::<Session>("session")
                .map_err(|_| ErrorUnauthorized("invalid session"))
                .and_then(|session| {
                    session
                        .map(|session| AuthedUser {
                            id: session.user_id,
                        })
                        .ok_or_else(|| ErrorUnauthorized("missing session"))
                }),
        )
    }
}
