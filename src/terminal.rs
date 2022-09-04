use std::fmt;
use std::io::{stdout, Stdout, Write};
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, AsyncReader};

#[derive(Debug, Clone, Copy)]
pub enum Inputs {
  Yes,
  No,
  Exit,
}

impl fmt::Display for Inputs {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Inputs::Yes => write!(f, "Yes"),
      Inputs::No => write!(f, "No"),
      Inputs::Exit => write!(f, "Exit"),
    }
  }
}

pub struct Stdio {
  pub stdout: RawTerminal<Stdout>,
  pub stdin: Keys<AsyncReader>,
}

impl Stdio {
  pub fn new() -> Self {
    Self {
      stdout: stdout().into_raw_mode().unwrap(),
      stdin: async_stdin().keys(),
    }
  }

  pub fn writeln(&mut self, message: String) {
    write!(self.stdout, "{}\r\n", message).unwrap();
  }

  pub fn write(&mut self, message: String) {
    write!(self.stdout, "{}", message).unwrap();
  }

  pub fn read_input(&mut self) -> Inputs {
    let mut input = Inputs::No;
    loop {
      if let Some(Ok(key)) = self.stdin.next() {
        match key {
          termion::event::Key::Char('y') => {
            input = Inputs::Yes;
            break;
          }
          termion::event::Key::Char('n') => {
            input = Inputs::No;
            break;
          }
          termion::event::Key::Esc | termion::event::Key::Char('\n') => break,
          termion::event::Key::Ctrl('d') | termion::event::Key::Ctrl('c') => {
            input = Inputs::Exit;
            break;
          }
          _ => {
            self.stdout.lock().flush().unwrap();
            continue;
          }
        }
      }
    }
    input
  }

  pub fn dialog(&mut self, question: String) -> Inputs {
    self.write(question + " [y/N] ");
    self.stdout.lock().flush().unwrap();
    let input = self.read_input();
    self.writeln(format!("{}", &input));
    input
  }
}
