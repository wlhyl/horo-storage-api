use actix_web::{FromRequest, HttpMessage};
use log::debug;
use std::future::{ready, Ready};

use crate::error::Error;

#[derive(Clone, Copy)]
pub struct AuthenticatedUser {
    pub id: u32,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let authenticated_user = req
            .extensions()
            .get::<AuthenticatedUser>()
            .copied()
            .ok_or_else(|| {
                debug!("AuthenticatedUser 提取器错误，不能提取到数据！");
                Error::Forbidden("不能获取认证信息，或没有认证，查看debug日志，了解详情!".into())
            });
        ready(authenticated_user)
    }
}
