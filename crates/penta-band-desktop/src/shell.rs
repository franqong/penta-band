use crate::{footer::Footer, header::Header};
use gpui::{Context, IntoElement, Render, Window, div, prelude::*, rgb};

#[derive(Default)]
pub struct AppShell;

impl Render for AppShell {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x070a0c))
            .child(Header)
            .child(div().flex_1().bg(rgb(0x0b1118)))
            .child(Footer)
    }
}
