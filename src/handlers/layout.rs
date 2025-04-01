use crate::views::layout;
use axum::{extract::Form, response::Html};
use serde::Deserialize;

pub async fn get_layout() -> Html<String> {
    Html(get_layout().into_string())
}

pub async fn handle_submit(Form(form): Form<NameForm>) -> Html<String> {
    Html(render_response(&form.name).into_string())
}
