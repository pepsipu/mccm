use maud::{Markup, html};

fn env_row(key: &str, value: &str) -> Markup {
    html! {
        .env-row {
            input
                .env-key
                type="text"
                name="key[]"
                placeholder="KEY"
                value=(key) {}
            input
                type="text"
                name="value[]"
                placeholder="VALUE"
                value=(value) {}
        }
    }
}

pub fn env_editor(server_name: &str, env: &[(String, String)]) -> Markup {
    let action = format!("/server/{}", server_name);

    html! {
        div {
            h3 { "Environment" }
            form .env-editor method="post" action=(action) {
                @for (key, value) in env {
                    (env_row(key, value))
                }
                (env_row("", ""))
                button type="submit" { "Save" }
            }
        }
    }
}
