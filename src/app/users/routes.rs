use crate::app::auth::guards;
use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;

use super::controllers::{edit_current_user, get_current_user};

pub fn config(cfg: &mut ServiceConfig) {
    let session = HttpAuthentication::with_fn(guards::session_guard);
    let jwt = HttpAuthentication::bearer(guards::jwt_guard);

    cfg.service(
        web::scope("/users")
            .wrap(session)
            .wrap(jwt)
            .route("", web::get().to(get_current_user))
            .route("", web::put().to(edit_current_user)),
    );
}
