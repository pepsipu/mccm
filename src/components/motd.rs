use maud::{Markup, html};
use mctext::{MCText, Span, TextColor};

fn color_style(color: TextColor) -> String {
    match color {
        TextColor::Named(named) => format!("color: var(--mc-{});", named.name()),
        TextColor::Rgb { .. } => format!("color: {};", color.to_hex()),
    }
}

fn classes_for_span(span: &Span) -> String {
    let mut classes = String::from("mc-span");

    if span.style.bold {
        classes.push_str(" mc-bold");
    }
    if span.style.italic {
        classes.push_str(" mc-italic");
    }
    if span.style.underlined {
        classes.push_str(" mc-underlined");
    }
    if span.style.strikethrough {
        classes.push_str(" mc-strikethrough");
    }

    classes
}

pub fn render_motd(motd: &str) -> Markup {
    let text = MCText::parse(motd);

    html! {
        @for span in text.spans() {
            @let classes = classes_for_span(span);
            @match span.color {
                Some(color) => span class=(classes) style=(color_style(color)) { (&span.text) }
                None => span class=(classes) { (&span.text) }
            }
        }
    }
}
