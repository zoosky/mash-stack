use crate::views::render_layout;
use crate::views::render_layout;

use axum::response::Html;

pub async fn get_layout() -> Html<String> {
    Html(render_layout("Hello", new Html()).into_string())
}
