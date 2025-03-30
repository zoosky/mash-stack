use crate::views::{render_counter, render_counter_value};
use axum::{extract::State, response::Html};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct CounterState {
    count: Arc<Mutex<i32>>,
}

impl Default for CounterState {
    fn default() -> Self {
        Self {
            count: Arc::new(Mutex::new(0)),
        }
    }
}

pub async fn get_counter() -> Html<String> {
    Html(render_counter(0).into_string())
}

pub async fn increment_counter(State(state): State<CounterState>) -> Html<String> {
    let mut count = state.count.lock().await;
    *count += 1;
    Html(render_counter_value(*count).into_string())
}

pub async fn decrement_counter(State(state): State<CounterState>) -> Html<String> {
    let mut count = state.count.lock().await;
    *count -= 1;
    Html(render_counter_value(*count).into_string())
}
