use maud::{Markup, html};
use mctext::{MCText, TextColor};

fn color_style(color: TextColor) -> String {
    match color {
        TextColor::Named(named) => format!("color: var(--mc-{});", named.name()),
        TextColor::Rgb { .. } => format!("color: {};", color.to_hex()),
    }
}

pub fn render_motd(motd: &str) -> Markup {
    let text = MCText::parse(motd);

    html! {
        @for span in text.spans() {
            span
                .mc-span
                .mc-bold[span.style.bold]
                .mc-italic[span.style.italic]
                .mc-underlined[span.style.underlined]
                .mc-strikethrough[span.style.strikethrough]
                style=[span.color.map(color_style)]
            {
                (&span.text)
            }
        }
    }
}
