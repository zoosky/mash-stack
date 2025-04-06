use crate::views::render_layout;
use axum::response::Html;
use maud::html;

pub async fn get_layout() -> Html<String> {
    let body = html! {
        div class="content" {
            p { "This is the layout body content." }
            p { "It demonstrates how to compose layouts with Maud templates." }
        }
    };
    
    Html(render_layout("Layout Example", body).into_string())
}
