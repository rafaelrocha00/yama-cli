use std::collections::HashMap;
use std::fmt::write;
use std::io::{Write, stdout, Result};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    execute, 
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    cursor
};
use std::char;


fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();

    let hiragana_map = create_translator();
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
                    let char = translate_input(key_code);
                   
                    if char == 'a' || char == 'e' || char == 'i' || char == 'o' || char == 'u' || char == 'n' {                 
                        let last_char = string.chars().last().unwrap_or('\0');
                        
                        let mut sylab: String =  String::new();

                        if last_char != '\0' {
                            sylab.push(last_char);
                        }

                        sylab.push(char);
                        let hiragana = hiragana_map.get(&sylab).unwrap_or(&sylab);
                        
                        string.pop();
                        string.push(hiragana.chars().last().unwrap());
                        string = string.trim().to_string();
                        
                        execute!(
                            stdout,
                            Clear(ClearType::CurrentLine),
                            cursor::MoveToColumn(0),
                            Print(&string)
                        )?;

                    } else {
                        write!(stdout, "{}", char)?;
                        string.push(char);
                    }
                    

                }
            };
        }

        stdout.flush()?;
    }
}


fn create_translator() -> HashMap<String, String> {
    let mut romanji_to_japanese: HashMap<String, String> = HashMap::new();
    
    romanji_to_japanese.insert("a".to_string(), "あ".to_string());
    romanji_to_japanese.insert("i".to_string(), "い".to_string());
    romanji_to_japanese.insert("u".to_string(), "う".to_string());
    romanji_to_japanese.insert("e".to_string(), "え".to_string());
    romanji_to_japanese.insert("o".to_string(), "お".to_string());

    romanji_to_japanese.insert("ka".to_string(), "か".to_string());
    romanji_to_japanese.insert("ki".to_string(), "き".to_string());
    romanji_to_japanese.insert("ku".to_string(), "く".to_string());
    romanji_to_japanese.insert("ke".to_string(), "け".to_string());
    romanji_to_japanese.insert("ko".to_string(), "こ".to_string());

    romanji_to_japanese.insert("sa".to_string(), "さ".to_string());
    romanji_to_japanese.insert("shi".to_string(), "し".to_string());
    romanji_to_japanese.insert("su".to_string(), "す".to_string());
    romanji_to_japanese.insert("se".to_string(), "せ".to_string());
    romanji_to_japanese.insert("so".to_string(), "そ".to_string());

    romanji_to_japanese.insert("ta".to_string(), "た".to_string());
    romanji_to_japanese.insert("chi".to_string(), "ち".to_string());
    romanji_to_japanese.insert("tsu".to_string(), "つ".to_string());
    romanji_to_japanese.insert("te".to_string(), "て".to_string());
    romanji_to_japanese.insert("to".to_string(), "と".to_string());

    romanji_to_japanese.insert("na".to_string(), "な".to_string());
    romanji_to_japanese.insert("ni".to_string(), "に".to_string());
    romanji_to_japanese.insert("nu".to_string(), "ぬ".to_string());
    romanji_to_japanese.insert("ne".to_string(), "ね".to_string());
    romanji_to_japanese.insert("no".to_string(), "の".to_string());

    romanji_to_japanese.insert("ha".to_string(), "は".to_string());
    romanji_to_japanese.insert("hi".to_string(), "ひ".to_string());
    romanji_to_japanese.insert("fu".to_string(), "ふ".to_string());
    romanji_to_japanese.insert("he".to_string(), "へ".to_string());
    romanji_to_japanese.insert("ho".to_string(), "ほ".to_string());
    
    romanji_to_japanese.insert("ma".to_string(), "ま".to_string());
    romanji_to_japanese.insert("mi".to_string(), "み".to_string());
    romanji_to_japanese.insert("mu".to_string(), "む".to_string());
    romanji_to_japanese.insert("me".to_string(), "め".to_string());
    romanji_to_japanese.insert("mo".to_string(), "も".to_string());

    romanji_to_japanese.insert("ya".to_string(), "や".to_string());
    romanji_to_japanese.insert("yu".to_string(), "ゆ".to_string());
    romanji_to_japanese.insert("yo".to_string(), "よ".to_string());

    romanji_to_japanese.insert("ra".to_string(), "ら".to_string());
    romanji_to_japanese.insert("ri".to_string(), "り".to_string());
    romanji_to_japanese.insert("ru".to_string(), "る".to_string());
    romanji_to_japanese.insert("re".to_string(), "れ".to_string());
    romanji_to_japanese.insert("ro".to_string(), "ろ".to_string());

    romanji_to_japanese.insert("wa".to_string(), "わ".to_string());
    romanji_to_japanese.insert("wo".to_string(), "を".to_string());
    
    romanji_to_japanese.insert("n".to_string(), "ん".to_string());

    romanji_to_japanese.insert("ga".to_string(), "が".to_string());
    romanji_to_japanese.insert("gi".to_string(), "ぎ".to_string());
    romanji_to_japanese.insert("gu".to_string(), "ぐ".to_string());
    romanji_to_japanese.insert("ge".to_string(), "げ".to_string());
    romanji_to_japanese.insert("go".to_string(), "ご".to_string());
    
    romanji_to_japanese.insert("za".to_string(), "ざ".to_string());
    romanji_to_japanese.insert("ji".to_string(), "じ".to_string());
    romanji_to_japanese.insert("zu".to_string(), "ず".to_string());
    romanji_to_japanese.insert("ze".to_string(), "ぜ".to_string());
    romanji_to_japanese.insert("zo".to_string(), "ぞ".to_string());
    
    romanji_to_japanese.insert("da".to_string(), "だ".to_string());
    romanji_to_japanese.insert("ji".to_string(), "ぢ".to_string());
    romanji_to_japanese.insert("zu".to_string(), "づ".to_string());
    romanji_to_japanese.insert("de".to_string(), "で".to_string());
    romanji_to_japanese.insert("do".to_string(), "ど".to_string());
    
    romanji_to_japanese.insert("ba".to_string(), "ば".to_string());
    romanji_to_japanese.insert("bi".to_string(), "び".to_string());
    romanji_to_japanese.insert("bu".to_string(), "ぶ".to_string());
    romanji_to_japanese.insert("be".to_string(), "べ".to_string());
    romanji_to_japanese.insert("bo".to_string(), "ぼ".to_string());
    
    romanji_to_japanese.insert("pa".to_string(), "ぱ".to_string());
    romanji_to_japanese.insert("pi".to_string(), "ぴ".to_string());
    romanji_to_japanese.insert("pu".to_string(), "ぷ".to_string());
    romanji_to_japanese.insert("pe".to_string(), "ぺ".to_string());
    romanji_to_japanese.insert("po".to_string(), "ぽ".to_string());

    return romanji_to_japanese
}

fn translate_input(key_code: KeyCode) -> char {
    match key_code {
        KeyCode::Char(c @ 'a'..='z') => return c,
        _ => return '\0'
    }
}