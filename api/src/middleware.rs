use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures_util::{future::LocalBoxFuture, FutureExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::{error, warn};
use sea_orm::EntityTrait;
use std::future::{ready, Ready};
use std::rc::Rc;

use crate::{
    error,
    extractor::AuthenticatedUser,
    state::AppState,
    utils::token::{self, Claims},
};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Transform = AuthMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let auth = AuthMiddleware {
            service: Rc::new(service),
        };
        ready(Ok(auth))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 获取token
        let token = if let Some(token) = req.headers().get("token") {
            token
        } else {
            // 没有提供token，直接返回
            let fut = self.service.call(req);
            return async { Ok(fut.await?) }.boxed_local();
        };

        let token = match token.to_str() {
            Ok(s) => s,
            Err(error) => {
                warn!("token解析不正确：{error}");
                return async {
                    let e = error::Error::Forbidden("token不正确".into());
                    Err(e.into())
                }
                .boxed_local();
            }
        };

        let app_data = if let Some(data) = req.app_data::<web::Data<AppState>>() {
            data
        } else {
            warn!("没有配置app_data");
            return async {
                let e = error::Error::ActixError("Internal Server Error".into());
                Err(e.into())
            }
            .boxed_local();
        };

        let mut validation = Validation::default();
        validation.insecure_disable_signature_validation();
        let claims = match decode::<Claims>(&token, &DecodingKey::from_secret(&[]), &validation) {
            Ok(claims) => claims,
            Err(error) => {
                warn!("jwt解码错误：{}", error);
                let e = error::Error::Forbidden("无效的token！".into());
                return async { Err(e.into()) }.boxed_local();
            }
        };

        let user_id = claims.claims.id;
        let db = app_data.db.clone();
        let token = token.to_string();

        let svc = Rc::clone(&self.service);
        Box::pin(async move {
            let user = entity::user::Entity::find_by_id(user_id)
                .one(&db)
                .await
                .map_err(|error| {
                    error!("数据查询user_id: {}错误：{}", user_id, error);
                    error::Error::Forbidden("认证失败！".into())
                })?;

            let user = user.ok_or_else(|| {
                warn!("user id: {}不存在，这可能是一个伪造的token", user_id);
                error::Error::Forbidden("认证失败！".into())
            })?;

            // 校验token
            let authenticated_user = token::verify(&token, &user.salt)
                .map(|claims| AuthenticatedUser { id: claims.id })
                .map_err(|error| {
                    warn!("检验密码发生错误：{}", error);
                    error::Error::Forbidden("认证失败！".into())
                })?;

            // 将已经认证的用户信息保存在req中,
            // AuthenticatedUser 实现 了提取器接口
            req.extensions_mut().insert(authenticated_user);
            // let res = svc.call(req).await?;
            // Ok(res)
            svc.call(req).await
        })
    }
}
