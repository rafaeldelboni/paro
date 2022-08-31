use std::io::{stdout, Stdout, Write};
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, AsyncReader};

pub fn build_stdio() -> (RawTerminal<Stdout>, Keys<AsyncReader>) {
  (stdout().into_raw_mode().unwrap(), async_stdin().keys())
}

pub fn are_you_sure(
  stdout: &mut RawTerminal<Stdout>,
  stdin: &mut Keys<AsyncReader>,
) -> bool {
  let mut answer = false;

  loop {
    let input = stdin.next();

    if let Some(Ok(key)) = input {
      match key {
        termion::event::Key::Char('y') => {
          answer = true;
          break;
        }
        termion::event::Key::Char('n') => {
          answer = false;
          break;
        }
        termion::event::Key::Esc | termion::event::Key::Char('\n') => break,
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
) -> bool {
  write!(stdout, "Overwrite? (y/N) ").unwrap();
  stdout.lock().flush().unwrap();
  let sure = are_you_sure(stdout, stdin);
  write!(stdout, "{}\r\n", &sure).unwrap();
  sure
}
