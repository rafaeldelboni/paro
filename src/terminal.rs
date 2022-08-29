use std::io::{stdout, Write};
use termion::async_stdin;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn are_you_sure() -> bool {
  let stdout = stdout().into_raw_mode().unwrap();
  let mut stdin = async_stdin().keys();
  let mut answer = false;

  loop {
    let input = stdin.next();

    if let Some(Ok(key)) = input {
      match key {
        termion::event::Key::Char('y') => {
          answer = true;
          break;
        }
        termion::event::Key::Char('n') => break,
        termion::event::Key::Char('\n') => break,
        termion::event::Key::Esc => break,
        _ => {
          stdout.lock().flush().unwrap();
          continue;
        }
      }
    }
  }

  answer
}
