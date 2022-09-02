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

pub fn build_stdio() -> (RawTerminal<Stdout>, Keys<AsyncReader>) {
  (stdout().into_raw_mode().unwrap(), async_stdin().keys())
}

pub fn are_you_sure(
  stdout: &mut RawTerminal<Stdout>,
  stdin: &mut Keys<AsyncReader>,
) -> Inputs {
  let mut answer = Inputs::No;

  loop {
    let input = stdin.next();

    if let Some(Ok(key)) = input {
      match key {
        termion::event::Key::Char('y') => {
          answer = Inputs::Yes;
          break;
        }
        termion::event::Key::Char('n') => {
          answer = Inputs::No;
          break;
        }
        termion::event::Key::Esc | termion::event::Key::Char('\n') => break,
        termion::event::Key::Ctrl('d') | termion::event::Key::Ctrl('c') => {
          answer = Inputs::Exit;
          break;
        }
        _ => {
          stdout.lock().flush().unwrap();
          continue;
        }
      }
    }
  }

  answer
}

pub fn can_i_overwrite(
  stdout: &mut RawTerminal<Stdout>,
  stdin: &mut Keys<AsyncReader>,
  thing: &str,
) -> Inputs {
  write!(stdout, "Overwrite? {} [y/N] ", thing).unwrap();
  stdout.lock().flush().unwrap();
  let sure = are_you_sure(stdout, stdin);
  write!(stdout, "{}\r\n", &sure).unwrap();
  sure
}
