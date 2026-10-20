//! The app, as a function: `main.rs` runs it, and the tests in `tests/`
//! drive the same one with no window.

use lumenui::prelude::*;

/// The whole app: the UI lumen.toml names, compiled by build.rs, and two
/// click handlers that write the `clicks` signal the label is bound to.
pub fn app() -> App {
    lumenui::app!()
        .signal("clicks", 0i64)
        .on_click("bump", |ui, _| ui.update("clicks", |n: i64| n + 1))
        .on_click("reset", |ui, _| ui.set("clicks", 0i64))
}
