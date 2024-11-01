use tower_sessions::{
    cookie::{time::Duration, Key},
    service::SignedCookie,
    session_store::Error,
    Expiry, MemoryStore, SessionManagerLayer,
};

pub async fn session_init() -> Result<SessionManagerLayer<MemoryStore, SignedCookie>, Error> {
    let key = Key::generate();
    let session_store = MemoryStore::default();
    let session_expiry = Expiry::OnInactivity(Duration::hours(3));

    let session_layer = SessionManagerLayer::new(session_store)
        .with_always_save(true)
        .with_secure(true)
        .with_same_site(tower_sessions::cookie::SameSite::None)
        .with_http_only(true)
        .with_expiry(session_expiry)
        .with_name("passkey_session")
        .with_signed(key);

    Ok(session_layer)
}
