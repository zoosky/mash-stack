use maud::{html, Markup};

pub fn render_footer() -> Markup {
    
    html! {
       
        footer {
            div class="w" {
                p { a href="https://yree.io/mash" { "mash" } " 🥔 :: a " a href="https://yree.io" { "Yree" } " stack ♥" }
            }
        }
    }
}

pub fn render_styles() -> Markup {
    html! {
        style {
            "
            footer {
                margin-top: 3rem;
                text-align: center;
                color: #6c757d;
            }
            footer a {
                color: #007bff;
                text-decoration: none;
                transition: color 0.2s ease;
            }
            footer a:hover {
                color: #0056b3;
            }
            "
        }
    }
}
