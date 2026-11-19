use std::{
    fs,
    path::{Path, PathBuf},
};

use blocks_render::View;
use primitive_app::{font_paths, frame, write_png, App, Change, HEIGHT, WIDTH};
use primitive_ui::{Canvas, Input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .unwrap_or_else(|| "build/previews".into()),
    );
    fs::create_dir_all(&output)?;
    let fonts =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../doannt-kindle-launcher/assets/fonts");
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &font_paths(&fonts))?;
    let mut app = View::preview(2026);
    write_png(
        &output.join("blocks.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, false),
    )?;
    assert_eq!(app.input(Input::Tap(230.0, 1400.0)), Change::Partial);
    assert_eq!(app.input(Input::Tap(180.0, 330.0)), Change::Partial);
    assert!(app.game.score() > 0);
    write_png(
        &output.join("blocks-playing-dark.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, true),
    )?;
    assert_eq!(app.input(Input::Tap(600.0, 1560.0)), Change::Full);
    write_png(
        &output.join("blocks-reset.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, false),
    )?;
    assert_eq!(app.input(Input::Tap(850.0, 855.0)), Change::Full);
    assert_eq!(app.game.score(), 0);
    assert!(app.game.best() > 0);
    Ok(())
}
