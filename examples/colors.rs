use std::io::Result;
use tiny_console::*;

pub fn main() -> Result<()> {
    let g = Color::Green;
    let tc = TinyConsole::stdout();
    tc.cwrite("Hey, ", g)?;
    tc.clearln()?;
    tc.write("Hello")?;
    tc.cwrite(", ", Color::Magenta)?;
    tc.cwrite("World", g)?;
    tc.writeln("!")
}
