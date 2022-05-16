use std::io::Result;
use tiny_console::*;

pub fn main() -> Result<()> {
    let tc = TinyConsole::new(Mode::Stdout);
    tc.write("Hey, ")?;
    tc.clearln()?;
    tc.write("Hello, ")?;
    tc.writeln("World!")
}
