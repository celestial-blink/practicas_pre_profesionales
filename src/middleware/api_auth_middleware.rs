use actix_web::{Error, body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, web};
use crate::modules::tokens::presentation::auth_token::auth_token;
use crate::general_types::State;

pub async fn api_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    let token = req.headers().get("Authorization").map(|header| header.to_str().unwrap().to_string());
    let token = token.map(|token| token.replace("Bearer ", ""));

    if let Some(token) = token {
        let state = req.app_data::<web::Data<State>>().unwrap();
        let token = auth_token(state.db.clone(), token).await;
        if let Some(token) = token {
            // valida estado

            if token.estado == 0 { // inactivo
                return Err(actix_web::error::ErrorUnauthorized("Token inactivo"));
            }

            if token.estado == 2 { // bloqueado
                return Err(actix_web::error::ErrorUnauthorized("Token bloqueado"));
            }

            // valida expiracion
            if let Some(expira_en) = token.expira_en {
                if expira_en < time::OffsetDateTime::now_utc() {
                    return Err(actix_web::error::ErrorUnauthorized("Token expirado"));
                }
            }
        } else {
            return Err(actix_web::error::ErrorUnauthorized("Token no encontrado"));
        }
    } else {
        return Err(actix_web::error::ErrorUnauthorized("Token no proporcionado"));
    }


    next.call(req).await
    // post-processing

}
