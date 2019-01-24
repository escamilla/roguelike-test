extern crate termion;

use std::io::{Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

const WALL_TOP_LEFT: char = '+';
const WALL_TOP_RIGHT: char = '+';
const WALL_BOTTOM_LEFT: char = '+';
const WALL_BOTTOM_RIGHT: char = '+';
const WALL_HORIZONTAL: char = '-';
const WALL_VERTICAL: char = '|';
const PLAYER: char = '@';
const EMPTY_SPACE: char = ' ';

struct Player {
    x: u16,
    y: u16,
}

struct Room {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

fn write_char(stdout: &mut RawTerminal<Stdout>, x: u16, y: u16, ch: char) {
    write!(
        stdout,
        "{}{}{}",
        termion::cursor::Goto(x, y),
        ch,
        termion::cursor::Hide
    )
    .unwrap();
}

impl Player {
    fn draw(&self, stdout: &mut RawTerminal<Stdout>) {
        write_char(stdout, self.x, self.y, PLAYER);
    }
}

impl Room {
    fn draw(&self, stdout: &mut RawTerminal<Stdout>) {
        for dx in 0..self.width {
            for dy in 0..self.height {
                let ch = if dx == 0 && dy == 0 {
                    WALL_TOP_LEFT
                } else if dx == self.width - 1 && dy == 0 {
                    WALL_TOP_RIGHT
                } else if dx == 0 && dy == self.height - 1 {
                    WALL_BOTTOM_LEFT
                } else if dx == self.width - 1 && dy == self.height - 1 {
                    WALL_BOTTOM_RIGHT
                } else if dx == 0 || dx == self.width - 1 {
                    WALL_VERTICAL
                } else if dy == 0 || dy == self.height - 1 {
                    WALL_HORIZONTAL
                } else {
                    EMPTY_SPACE
                };
                write_char(stdout, self.x + dx, self.y + dy, ch);
            }
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    let mut player = Player { x: 1, y: 1 };

    let room = Room {
        x: 4,
        y: 2,
        width: 8,
        height: 4,
    };

    write!(
        stdout,
        "{}{}Press q to exit. Use the vi keys (hjkl) to move around.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('h') => player.x -= 1,
            Key::Char('j') => player.y += 1,
            Key::Char('k') => player.y -= 1,
            Key::Char('l') => player.x += 1,
            Key::Char('y') => {
                player.x -= 1;
                player.y -= 1;
            }
            Key::Char('u') => {
                player.x += 1;
                player.y -= 1;
            }
            Key::Char('b') => {
                player.x -= 1;
                player.y += 1;
            }
            Key::Char('n') => {
                player.x += 1;
                player.y += 1;
            }
            Key::Left => player.x -= 1,
            Key::Right => player.x += 1,
            Key::Up => player.y -= 1,
            Key::Down => player.y += 1,
            _ => (),
        }

        if player.y < 1 {
            player.y = 1
        }
        if player.x < 1 {
            player.x = 1
        }

        let (columns, rows): (u16, u16) = termion::terminal_size().unwrap();
        if player.y > rows {
            player.y = rows
        }
        if player.x > columns {
            player.x = columns
        }

        write!(stdout, "{}", termion::clear::All).unwrap();
        room.draw(&mut stdout);
        player.draw(&mut stdout);
        stdout.flush().unwrap();
    }

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    )
    .unwrap();
}
