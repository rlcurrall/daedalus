use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::{time::Duration, Key};

use crate::config::SessionSettings;

pub struct SessionMiddlewareBuilder;

impl SessionMiddlewareBuilder {
    pub fn build(settings: SessionSettings) -> SessionMiddleware<CookieSessionStore> {
        SessionMiddleware::builder(
            CookieSessionStore::default(),
            Key::from(&settings.secret.as_bytes()),
        )
        .cookie_secure(settings.secure)
        .session_lifecycle(
            PersistentSession::default()
                .session_ttl(Duration::seconds(settings.lifetime.as_secs() as i64)),
        )
        .build()
    }
}
