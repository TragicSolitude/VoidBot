use std::sync::Arc;
use std::env;
use serenity::prelude::*;
use sentry::capture_message;
use sentry::Level;
use sentry::internals::ClientInitGuard;
use sentry::integrations::panic::register_panic_handler;

pub struct ErrorManager {
    use_sentry: bool,
    _sentry_guard: Arc<Option<ClientInitGuard>>
}

impl TypeMapKey for ErrorManager {
    type Value = Self;
}

impl ErrorManager {
    pub fn new() -> Self {
        let _sentry_guard = match env::var("SENTRY_URL") {
            Ok(sentry_key) => {
                println!("Using Sentry error tracking.");
                let guard = sentry::init(sentry_key);
                register_panic_handler();

                Arc::new(Some(guard))
            },
            Err(_) => Arc::new(None)
        };

        let use_sentry = _sentry_guard.is_some();

        ErrorManager { use_sentry, _sentry_guard }
    }

    pub fn error_log(&self, msg: &str) -> () {
        println!("ERROR: {}", msg);
        if self.use_sentry {
            capture_message(msg, Level::Error);
        }
    }
}
