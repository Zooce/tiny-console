use std::io::Result;
use tiny_console::*;

pub fn main() -> Result<()> {
    let tc = TinyConsole::stdout();
    tc.write("Hey, ")?;
    tc.clearln()?;
    tc.write("Hello, ")?;
    tc.writeln("World!")
}
