use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|term| {
	let size = term.size();
	let block = Block::default()
	    .title("Block")
	    .borders(Borders::ALL);
	term.render_widget(block, size);
    })?;
    

    Ok(())
}
