#![warn(clippy::all, clippy::pedantic)]

mod editor;
use editor::Editor;

mod terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut editor = Editor::default()?;
    editor.run();
    Ok(())
}
