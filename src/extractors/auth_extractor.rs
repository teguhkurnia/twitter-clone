use actix_web::FromRequest;

use crate::lib::jwt;

pub struct Authenticated {
    pub id: i32,
    pub username: String,
    pub token: String,
}

impl FromRequest for Authenticated {
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let token = req.headers().get("Authorization").map(|h| h.to_str().unwrap_or("")).unwrap_or("");
        let claims = jwt::verify_jwt(&token.replace("Bearer ", ""));

        match claims {
            Ok(claims) => {
                let id = claims.id;
                let username = claims.username.clone();
                let token = token.to_string();
                let expired = claims.exp;
                
                if expired < chrono::Utc::now().timestamp() as usize {
                    return Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Token expired")) });
                }

                Box::pin(async move {
                    Ok(Authenticated {
                        id,
                        username,
                        token,
                    })
                })
            }
            Err(_) => Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Invalid token")) }),
        }
    }
}