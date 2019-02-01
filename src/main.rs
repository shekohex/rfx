use json_color::{Color, Colorizer};
use std::{fs::File, io};
fn main() -> Result<(), ExitDisplay<io::Error>> {
  let args: Vec<_> = std::env::args().skip(1).collect();
  if !args.is_empty() {
    let file = File::open(&args[0])?;
    process_input(file)?
  } else {
    let stdin = io::stdin();
    let stdin_handle = stdin.lock();
    process_input(stdin_handle)?
  }
  Ok(())
}

fn process_input(mut f: impl io::Read) -> Result<(), ExitDisplay<io::Error>> {
  let colorizer = Colorizer::new()
    .escape_sequence(Color::Yellow)
    .null(Color::Red)
    .boolean(Color::Cyan)
    .number(Color::Yellow)
    .string(Color::Green)
    .key(Color::Purple)
    .build();
  let mut json = String::new();
  f.read_to_string(&mut json)?;
  let stdout = io::stdout();
  let mut stdout = stdout.lock();
  colorizer.colorize_to_writer(&json, &mut stdout)?;
  Ok(())
}

pub struct ExitDisplay<E: std::fmt::Display>(E);
impl<E: std::fmt::Display> std::fmt::Debug for ExitDisplay<E> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl<E: std::fmt::Display> From<E> for ExitDisplay<E> {
  fn from(e: E) -> Self { ExitDisplay(e) }
}
