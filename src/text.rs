use std::io::Stdout;
use std::io::{Write};

use crossterm::cursor::{MoveToNextLine, MoveToColumn};
use crossterm::execute;
use crossterm::{terminal, cursor, QueueableCommand, ExecutableCommand, style::Stylize};

pub fn center(string: &str,  mut stdout: &Stdout) {

    let text_width = string.len() as u16;
    let (terminal_width, _) = terminal::size().unwrap();

    let (_, cursor_y) = cursor::position().unwrap();
    let center_x = (terminal_width - text_width) / 2;

    stdout.queue(cursor::MoveTo(center_x, cursor_y)).unwrap();

    println!("{}", string.dim());

    stdout.execute(cursor::MoveToColumn(0)).unwrap();
}

pub fn left(string: &str, mut stdout: &Stdout) {
    execute!(stdout, MoveToNextLine(1)).unwrap();
    println!("{}", string.dim());
}
