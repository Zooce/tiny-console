use std::io::Result;
use tiny_console::*;

pub fn main() -> Result<()> {
    let style = Style::new().fg(Color::Cyan).on_magenta();
    let tc = TinyConsole::new(Target::Stdout, false);
    tc.swrite("Hey, ", &style)?;
    tc.clearln()?;
    tc.write("Hello")?;
    tc.swrite(", ", &style)?;
    tc.swrite("World", &Style::new().fg(Color::Black).on_white())?;
    tc.swriteln("!", &style)
}
