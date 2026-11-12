use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

#[derive(IntoElement)]
pub struct Footer;

impl RenderOnce for Footer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .h(px(80.0))
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .px_6()
            .bg(rgb(0x12161c))
            .border_t_1()
            .border_color(rgb(0x676767))
            .text_color(rgb(0xedeff1))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_5()
                    .child("About")
                    .child("News")
                    .child("Apps")
                    .child("Contact")
                    .child("Downloads"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .text_color(rgb(0x8f969e))
                    .child("GitHub")
                    .child("Mastodon")
                    .child("Social"),
            )
    }
}
