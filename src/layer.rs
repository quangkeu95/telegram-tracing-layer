use tracing::{Event, Subscriber};
use tracing_subscriber::{layer::Context, Layer};

pub struct TelegramLayer {
    token_id: String,
    chat_id: i64,
}

impl TelegramLayer {
    pub fn new(token_id: String, chat_id: i64) -> Self {
        Self { token_id, chat_id }
    }
}

impl<S> Layer<S> for TelegramLayer
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {}
}
