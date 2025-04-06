use super::common::render_footer;
use crate::views::components::header;
use maud::{html, Markup};

pub fn render_form() -> Markup {
    html! [
            (maud::DOCTYPE)
            html {
                 (header("title.to_string()"))
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
                             div class="demo-section" {
                                h2 class="demo-title" { "Layout Example" }
                                a href="/layout" class="demo-button" { "👉 See the Layout" }
                            }
                            div class="demo-section" {

                                h1 { "Hello, world!" }
                                p.intro {
                                    "This is an example of the "
                                    a href="https://github.com/lambda-fairy/maud" { "Maud" }
                                    " template language."
                                }



                    }
                }
                (render_footer())
            }
        }
    }
        ]
}

pub fn render_response(name: &str) -> Markup {
    html! {
        p { "Hello, " (name) "! 👋" }
    }
}
