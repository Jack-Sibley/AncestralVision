use maud::{DOCTYPE, Markup, Render, html};

pub fn page(contents: &impl Render) -> Markup {
    html! {
        (DOCTYPE);
        html lang="en" {
            head {
                meta charset="UTF-8";
                title {"MTG Rules"}
                meta name="viewport" content="width=device-width,initial-scale=1";
                meta name="description" content "";
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css";
            }
            body {
                main .container {
                    (contents)
                }
            }
        }
    }
}
