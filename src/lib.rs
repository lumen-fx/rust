//! The app, as a function: `main.rs` runs it, and the tests in `tests/`
//! drive the same one with no window.

use lumenui::prelude::*;

/// The whole app: the markup and stylesheet in `src/`, and two click
/// handlers that write the `clicks` signal the label is bound to.
pub fn app() -> App {
    App::new()
        // From disk with hot reload in a debug build, compiled into the
        // executable in a release build.
        .add_plugins(LumenDefaultPlugins.with_source(lumen_source!("src/main.lmn", "src/main.css")))
        .insert_signal("clicks", 0i64)
        .on_click("bump", |ctx| {
            let n = ctx.get_or::<i64>("clicks", 0) + 1;
            ctx.set("clicks", n);
        })
        .on_click("reset", |ctx| ctx.set("clicks", 0i64))
}
