# rust

The click counter, written in Rust with the Lumen SDK (`lumenui`).

What it shows:

- The app as a Rust program. `src/lib.rs` builds a `lumenui::App` from the
  markup and stylesheet in `src/` and two click handlers; `src/main.rs`
  runs it.
- Signals from Rust. `insert_signal("clicks", 0)` seeds a named entry in the
  reactive store, and `on_click("bump", ...)` writes it. The label's
  `bind-text="clicks"` re-renders whenever it changes.
- Hot reload in a debug build. `lumen_source!` reads `src/main.lmn` and
  `src/main.css` from disk and watches them under `cargo run`, and compiles
  them into the executable under `cargo build --release`.
- A test that renders. `tests/screenshot.rs` paints the app with no window
  and checks the frame; the same `screenshot` call is how you compare a
  view with a golden image.

The engine is not compiled with the app: `lumenui` links the prebuilt engine
of the Lumen release it belongs to, and that engine is readable only by the
compiler that built it. `rust-toolchain.toml` names that compiler, so keep it
in step with the `lumenui` version in `Cargo.toml`.

Run it:

```sh
lumenc run .
```

or `cargo run`. With no window, for a CI job or a quick look:

```sh
lumenc run . --headless --ticks 60
cargo test
```

Ship it as a folder that runs on a machine with no Lumen installed:

```sh
lumenc package .
```
