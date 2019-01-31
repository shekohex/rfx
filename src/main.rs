use json_color::{Color, Colorizer};
use std::io;
use std::io::Read;

fn main() -> Result<(), ExitDisplay<io::Error>> {
  let stdin = io::stdin();
  let stdout = io::stdout();
  let mut stdin_handle = stdin.lock();
  let mut buffer = String::new();
  let colorizer = Colorizer::new()
    .escape_sequence(Color::Purple)
    .null(Color::Cyan)
    .boolean(Color::Yellow)
    .number(Color::Magenta)
    .string(Color::Green)
    .key(Color::Blue)
    .build();
  stdin_handle.read_to_string(&mut buffer)?;
  let json: serde_json::Value = serde_json::from_str(&buffer)
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
  let mut stdout_handle = stdout.lock();
  buffer = serde_json::to_string_pretty(&json)
    .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;
  colorizer.colorize_to_writer(&buffer, &mut stdout_handle)?;
  Ok(())
}

pub struct ExitDisplay<E: std::fmt::Display>(E);
impl<E: std::fmt::Display> std::fmt::Debug for ExitDisplay<E> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl<E: std::fmt::Display> From<E> for ExitDisplay<E> {
  fn from(e: E) -> Self {
    ExitDisplay(e)
  }
}
