use std::io::{Write, stdout, Result, Stdout};
use crossterm::style::{Print};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    execute, 
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    cursor
};

mod japanese;
mod text;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    introduce(&stdout);

    let kana = japanese::Kana::new(true);

    let mut string = String::new();

    loop {
        if let Event::Key(key_event) = read()? {
            let key_code = key_event.code;

            match key_code {
                KeyCode::Backspace => {
                    string.pop();
                    let string = kana.translate(&string);

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
                KeyCode::Char(char) => {   
                    string.push(char);

                    let string = kana.translate(&string);

                    execute!(
                        stdout,
                        Clear(ClearType::CurrentLine),
                        cursor::MoveToColumn(0),
                        Print(string)
                    )?;  
                }
                _ => {

                }
            };
        }

        stdout.flush()?;
    }
}

fn introduce(stdout: &Stdout) {
    text::center("=^_^=", &stdout);
    text::center("Welcome to Yama-cli!", &stdout);
    text::center("Everthing you write will be in hiragana. You can toogle to katakana by using uppercase!", &stdout);

}

  
