use std::io::{Write, stdout, Result};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    execute, 
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    cursor
};
use yama_cli::japanese::Hiragana;
use std::char;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();

    let hiragana = Hiragana::new();
    let mut string = String::new();

    loop {
        if let Event::Key(key_event) = read()? {
            let key_code = key_event.code;

            match key_code {
                KeyCode::Backspace => {
                    string.pop();

                    execute!(
                        stdout,
                        Clear(ClearType::CurrentLine),
                        cursor::MoveToColumn(0),
                        Print(&string)
                    )?;
                },
                KeyCode::Enter => {
                    writeln!(stdout)?;
                    execute!(stdout, cursor::MoveToColumn(0))?;
                    string.clear();
                },
                KeyCode::Esc => {
                    execute!(
                        stdout,
                        Clear(ClearType::CurrentLine),
                        cursor::MoveToColumn(0)
                    )?;

                    disable_raw_mode()?;
                    return Ok(())
                },
                _ => {                    
                    let char = key_to_char(key_code);
                    string.push(char);

                    let string = hiragana.translate(&string);

                    execute!(
                        stdout,
                        Clear(ClearType::CurrentLine),
                        cursor::MoveToColumn(0),
                        Print(string)
                    )?;     
                }
            };
        }

        stdout.flush()?;
    }
}


fn key_to_char(key_code: KeyCode) -> char {
    match key_code {
        KeyCode::Char(c @ 'a'..='z') => return c,
        _ => return '\0'
    }
}