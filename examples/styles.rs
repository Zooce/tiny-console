use std::io::Result;
use tiny_console::*;

pub fn main() -> Result<()> {
    let s = Style::new().fg(Color::White).on_magenta();
    let tc = TinyConsole::new(Mode::Stdout);
    tc.swrite("Hey, ", &s)?;
    tc.clearln()?;
    tc.write("Hello")?;
    tc.swrite(", ", &s)?;
    tc.swrite("World", &Style::new().fg(Color::Black).on_white())?;
    tc.swriteln("!", &s)
}
