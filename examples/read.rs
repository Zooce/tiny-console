use tiny_console::*;

pub fn main() -> std::io::Result<()> {
    let tc = TinyConsole::new(Mode::BufferedStdout);

    let c = Color::Cyan;

    tc.write("What's your ")?;
    tc.cwrite("name", c)?;
    tc.write("? ")?;
    tc.flush()?;

    let name = tc.read()?;

    tc.write("Nice to meet you, ")?;
    tc.cwriteln(&name, c)?;
    tc.flush()?;

    Ok(())
}
