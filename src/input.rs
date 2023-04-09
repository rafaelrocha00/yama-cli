use std::io::{Stdout, Write};
use crossterm::{event::{Event, read, KeyCode}, execute, cursor::{self, MoveToNextLine, MoveToColumn}, terminal::{ClearType, Clear}};


pub fn read_line(mut stdout: &Stdout) -> Result<String, &'static str> {
    execute!(std::io::stdout(), MoveToNextLine(1)).unwrap();
    execute!(stdout, MoveToColumn(0)).unwrap();

    let mut line = String::new();

   loop {
    if let Event::Key(key_event) = read().unwrap() {
        let key_code = key_event.code;
        let mut end = false;

        match key_code {
            KeyCode::Backspace => {
                line.pop();

            },
            KeyCode::Enter => {
                end = true;
            },
            KeyCode::Esc => {
            },
            KeyCode::Char(char) => {   
                line.push(char);
            }
            _ => {

            }
        };

        clear();
        print!("{}", line);
        stdout.flush().unwrap();


        if end {
            println!("");
            clear();
            return Ok(line)
        }
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