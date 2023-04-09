use std::io::{Write, stdout, Result, Stdout};
use crossterm::style::{Print};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    execute, 
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    cursor
};
use options::Options;

mod japanese;
mod text;
mod input;
mod options;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    introduce(&mut stdout);
    
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

fn introduce(stdout: &mut Stdout) {

    text::center("=^_^=", &stdout);
    text::center("Welcome to Yama-cli!", &stdout);
    text::center("Everthing you write will be in hiragana. You can toogle to katakana by using uppercase!", &stdout);

    let mut init_menu = Options::new();
    init_menu.option("Create new Deck".to_string(), create_deck);
    init_menu.option("Exit".to_string(), exit);
    init_menu.pick();
}

fn get_info(stdout: &mut Stdout) {
    text::left("Name ->", stdout);
    let name = input::read_line(stdout).unwrap();    

    text::left("Password ->", stdout);
    let password = input::read_line(stdout).unwrap();  
}

fn create_deck() {

}

fn exit() {

}

  
