mod editor;
mod file;
mod input;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = env::args().nth(1);
    let mut terminal = ratatui::init();

    let mut editor = editor::Editor::new(filename);

    editor.run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}
