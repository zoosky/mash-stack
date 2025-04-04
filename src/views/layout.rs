use maud::{html, Markup};

use crate::views::components::footer;
use crate::views::components::header;
/// A basic page layout with a header, body, and footer.
/// The final Markup, including `header` and `footer`.
///
/// Additionally takes a `greeting_box` that's `Markup`, not `&str`.
pub fn render_layout(title: &str, body: Markup) -> Markup {
    html! {
        // Add the header markup to the page
        (header(title))
        h1 { (title) }
        (body)
        (footer())
    }
}
