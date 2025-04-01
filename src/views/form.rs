use maud::{html, Markup};
use super::common::{render_footer, render_styles};

pub fn render_form() -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
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
            body a="auto" {
                main class="content" aria-label="Content" {
                    div class="w" id="markdown-view" _="on load call MathJax.typeset()" {
                        h1 { "Mash 🥔" }
                        p { "A simple demo using the mash stack." }
                        h2 { "What's your name?" }
                        form hx-post="/submit" hx-target="#response" {
                            div class="grid" {
                                input
                                    type="text"
                                    id="name"
                                    name="name"
                                    required
                                    placeholder="Enter your name"
                                    class="input";
                                button type="submit" class="button" { "Submit" }
                            }
                        }
                        div id="response" {}
                        div class="demo-section" {
                            h2 class="demo-title" { "Counter Example" }
                            a href="/counter" class="demo-button" { "👉 Try the Counter" }
                        }
                    }
                }
                (render_footer())
            }
        }
    }
}

pub fn render_response(name: &str) -> Markup {
    html! {
        p { "Hello, " (name) "! 👋" }
    }
}
