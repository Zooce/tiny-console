use std::io::Result;
use tiny_console::*;

pub fn main() -> Result<()> {
    let tc = TinyConsole::new(Target::Stdout, true);
    tc.write("Hey, ")?;
    tc.clearln()?;
    tc.write("Hello, ")?;
    tc.writeln("World!")
    // no output since we didn't call `tc.flush()`
}
