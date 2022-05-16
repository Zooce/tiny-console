use tiny_console::*;

pub fn main() -> std::io::Result<()> {
    let tc = TinyConsole::new(Mode::Stdout);

    tc.write("What's the secret code? ")?;

    let secret = tc.secread()?;

    tc.writeln(&format!("Nope, the secret code is not: {}", secret))?;

    Ok(())
}
