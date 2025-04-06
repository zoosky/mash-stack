use maud::{html, Markup, DOCTYPE};
use crate::views::common::render_styles;

/// A basic header with a dynamic `page_title`.
pub fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
        head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" href="https://yree.io/mold/assets/css/main.css";
                link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🥔</text></svg>";
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="";
                link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&amp;display=swap" rel="stylesheet";
                title { "A mash demo 🥔" }
                script src="https://unpkg.com/htmx.org" {}
                style {
                    "
                    .demo-section {
                        margin-top: 3rem;
                        text-align: center;
                    }
                    .demo-title {
                        color: #495057;
                        margin-bottom: 1rem;
                    }
                    .demo-button {
                        display: inline-block;
                        padding: 0.75rem 1.5rem;
                        background: #28a745;
                        color: white;
                        text-decoration: none;
                        border-radius: 4px;
                        transition: all 0.2s ease;
                    }
                    .demo-button:hover {
                        background: #218838;
                        transform: translateY(-1px);
                    }
                    .demo-button:active {
                        transform: translateY(0);
                    }
                    "
                }
                (render_styles())
            }
    }
}

/// A static footer.
pub fn footer() -> Markup {
    html! {
        footer {
            a href="rss.atom" { "RSS Feed" }
        }
    }
}
