//! Renders the app with no window and checks the frame it painted. Set
//! `UPDATE_SCREENSHOT=1` to write it to `target/screenshot.png` as well.

use lumenui::HeadlessOptions;

#[test]
fn the_app_paints_its_page() {
    let frame = app::app()
        .screenshot(HeadlessOptions {
            dpr: 1.0,
            ticks: Some(3),
        })
        .expect("the app renders");

    // `[window] size` in lumen.toml, at a device pixel ratio of 1.
    assert_eq!((frame.width(), frame.height()), (480, 360));
    // A corner shows the page background, --color-page in main.css.
    assert_eq!(frame.pixel(2, 2), Some([0x0c, 0x1c, 0x30, 0xff]));

    if std::env::var_os("UPDATE_SCREENSHOT").is_some() {
        frame
            .save_png(concat!(env!("CARGO_MANIFEST_DIR"), "/target/screenshot.png"))
            .expect("write the screenshot");
    }
}
