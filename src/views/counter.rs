use maud::{html, Markup};
use super::common::{render_footer, render_styles};

pub fn render_counter_value(count: i32) -> Markup {
    html! {
        span id="counter-value" class="counter-value" { (count) }
    }
}

pub fn render_counter(count: i32) -> Markup {
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
                title { "Counter Demo 🥔" }
                script src="https://unpkg.com/htmx.org" {}
                style {
                    "
                    .counter-container {
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        gap: 1rem;
                        margin: 2rem 0;
                        padding: 1rem;
                        background: #f8f9fa;
                        border-radius: 8px;
                        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                    }
                    .counter-button {
                        width: 3rem;
                        height: 3rem;
                        font-size: 1.5rem;
                        border-radius: 50%;
                        border: none;
                        background: #007bff;
                        color: white;
                        cursor: pointer;
                        transition: all 0.2s ease;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                    }
                    .counter-button:hover {
                        background: #0056b3;
                        transform: scale(1.05);
                    }
                    .counter-button:active {
                        transform: scale(0.95);
                    }
                    .counter-value {
                        font-size: 2rem;
                        font-weight: bold;
                        min-width: 3rem;
                        text-align: center;
                        color: #212529;
                        padding: 0.5rem 1rem;
                        background: white;
                        border-radius: 4px;
                        box-shadow: inset 0 1px 3px rgba(0,0,0,0.1);
                    }
                    .counter-title {
                        text-align: center;
                        margin-bottom: 1rem;
                        color: #495057;
                    }
                    .back-button {
                        display: inline-block;
                        margin-top: 2rem;
                        padding: 0.5rem 1rem;
                        background: #6c757d;
                        color: white;
                        text-decoration: none;
                        border-radius: 4px;
                        transition: background 0.2s ease;
                    }
                    .back-button:hover {
                        background: #5a6268;
                    }
                    "
                }
                (render_styles())
            }
            body a="auto" {
                main class="content" aria-label="Content" {
                    div class="w" {
                        h1 class="counter-title" { "Counter Demo 🥔" }
                        div class="counter-container" {
                            button
                                class="counter-button"
                                hx-post="/counter/decrement"
                                hx-target="#counter-value"
                                hx-swap="innerHTML"
                                aria-label="Decrement counter" { "-" }
                            (render_counter_value(count))
                            button
                                class="counter-button"
                                hx-post="/counter/increment"
                                hx-target="#counter-value"
                                hx-swap="innerHTML"
                                aria-label="Increment counter" { "+" }
                        }
                        a href="/" class="back-button" { "← Back to Home" }
                    }
                }
                (render_footer())
            }
        }
    }
}
