use std::collections::HashMap;
use std::io::{Write, stdout, Result};
use crossterm::{
    execute, 
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, ClearType, Clear},
    cursor, style::{SetForegroundColor, SetBackgroundColor, Color}
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();

    loop {
        if let Event::Key(key_event) = read()? {
            let key_code = key_event.code;
            let mut string = String::new();

            match key_code {
                KeyCode::Backspace => {
                    write!(stdout, "\u{8} \u{8}")?;
                    string.pop();

                },
                KeyCode::Enter => {
                    writeln!(stdout)?;
                    execute!(stdout, cursor::MoveToColumn(0))?;
                    string.clear();
                },
                KeyCode::Esc => {
                    break;
                },
                _ => {
                    string.push_str("--");
                    write!(stdout, "{}", string)?;
                }
            };
        }

        stdout.flush()?;
    }

    disable_raw_mode()?;
    Ok(())
}


fn createTranslator() -> HashMap<&'static str, &'static str> {
    let mut romanji_to_japanese = HashMap::new();
    
    romanji_to_japanese.insert("a", "あ");
    romanji_to_japanese.insert("i", "い");
    romanji_to_japanese.insert("u", "う");
    romanji_to_japanese.insert("e", "え");
    romanji_to_japanese.insert("o", "お");

    romanji_to_japanese.insert("ka", "か");
    romanji_to_japanese.insert("ki", "き");
    romanji_to_japanese.insert("ku", "く");
    romanji_to_japanese.insert("ke", "け");
    romanji_to_japanese.insert("ko", "こ");

    romanji_to_japanese.insert("sa", "さ");
    romanji_to_japanese.insert("shi", "し");
    romanji_to_japanese.insert("su", "す");
    romanji_to_japanese.insert("se", "せ");
    romanji_to_japanese.insert("so", "そ");

    romanji_to_japanese.insert("ta", "た");
    romanji_to_japanese.insert("chi", "ち");
    romanji_to_japanese.insert("tsu", "つ");
    romanji_to_japanese.insert("te", "て");
    romanji_to_japanese.insert("to", "と");

    romanji_to_japanese.insert("na", "な");
    romanji_to_japanese.insert("ni", "に");
    romanji_to_japanese.insert("nu", "ぬ");
    romanji_to_japanese.insert("ne", "ね");
    romanji_to_japanese.insert("no", "の");

    romanji_to_japanese.insert("ha", "は");
    romanji_to_japanese.insert("hi", "ひ");
    romanji_to_japanese.insert("fu", "ふ");
    romanji_to_japanese.insert("he", "へ");
    romanji_to_japanese.insert("ho", "ほ");
    
    romanji_to_japanese.insert("ma", "ま");
    romanji_to_japanese.insert("mi", "み");
    romanji_to_japanese.insert("mu", "む");
    romanji_to_japanese.insert("me", "め");
    romanji_to_japanese.insert("mo", "も");

    romanji_to_japanese.insert("ya", "や");
    romanji_to_japanese.insert("yu", "ゆ");
    romanji_to_japanese.insert("yo", "よ");

    romanji_to_japanese.insert("ra", "ら");
    romanji_to_japanese.insert("ri", "り");
    romanji_to_japanese.insert("ru", "る");
    romanji_to_japanese.insert("re", "れ");
    romanji_to_japanese.insert("ro", "ろ");

    romanji_to_japanese.insert("wa", "わ");
    romanji_to_japanese.insert("wo", "を");
    
    romanji_to_japanese.insert("n", "ん");

    return romanji_to_japanese
}