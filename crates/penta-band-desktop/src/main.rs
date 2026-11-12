mod assets;
mod footer;
mod header;
mod shell;

use assets::Assets;
use gpui::{App, AppContext, Application, Bounds, Window, WindowBounds, WindowOptions, px, size};
use shell::AppShell;

fn main() {
    Application::new()
        .with_assets(Assets::from_repository())
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(1100.0), px(720.0)), cx);

            cx.open_window(
                WindowOptions {
                    titlebar: Some(gpui::TitlebarOptions {
                        title: Some("Penta Band".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_window: &mut Window, cx| cx.new(|_| AppShell::default()),
            )
            .expect("failed to open Penta Band window");

            cx.activate(true);
        });
}
