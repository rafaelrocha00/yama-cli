use std::io::{Stdout, Write};
use crossterm::{event::{Event, read, KeyCode}, execute, cursor::{self, MoveToNextLine, MoveToColumn, MoveTo}, terminal::{ClearType, Clear}};


pub fn read_line(mut stdout: &Stdout) -> Result<String, &'static str> {
    execute!(std::io::stdout(), MoveToNextLine(1)).unwrap();
    execute!(stdout, MoveToColumn(0)).unwrap();

    let mut line = String::new();
    let mut pos = 0;

   loop {
    if let Event::Key(key_event) = read().unwrap() {
        let key_code = key_event.code;

        match key_code {
            KeyCode::Backspace => {
                if line.is_empty() || pos == 0 {
                    continue;
                }

                line.remove(pos - 1);
                pos -= 1;
            },
            KeyCode::Enter => {
                println!("");
                clear();
                return Ok(line);
            },
            KeyCode::Esc => {
            },
            KeyCode::Char(char) => {   
                line.insert(pos, char);
                pos += 1;
            }
            _ => {

            }
        };

        execute!(stdout, MoveTo(0, cursor::position()?.1), Clear(ClearType::FromCursorDown)).unwrap();
        print!("{}", &line);
        stdout.flush().unwrap();
        execute!(stdout, MoveToColumn(pos as u16)).unwrap();
    }
   }
}

fn clear() {
    execute!(
        std::io::stdout(),
        Clear(ClearType::CurrentLine),
        cursor::MoveToColumn(0),
    ).unwrap();
}

fn reset_cursor() {
    execute!(std::io::stdout(), MoveToNextLine(1)).unwrap();
}

fn clear_line() {
    execute!(std::io::stdout(), Clear(ClearType::CurrentLine), cursor::MoveToColumn(0)).unwrap();
}