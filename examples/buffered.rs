use std::io::Result;
use tiny_console::*;

pub fn main() -> Result<()> {
    let tc = TinyConsole::new(Mode::BufferedStdout);
    tc.write("Hey, ")?;
    tc.clearln()?;
    tc.write("Hello, ")?;
    tc.writeln("World!")?;
    Ok(())
    // no output since we didn't call `tc.flush()`
}
