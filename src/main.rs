use std::io::{stdin, stdout};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use tetris::Tetris;

struct TetrisTerm {
    tetris: Tetris,
}

impl TetrisTerm {
    fn run(&mut self) {
        let stdin = stdin();
        let mut _stdout: termion::raw::RawTerminal<std::io::Stdout> =
            stdout().into_raw_mode().unwrap();

        let (tx, rx) = channel::<&str>();

        let time_tx = tx.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(1000));
            time_tx.send("tick").unwrap();
        });

        thread::spawn(move || {
            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Left => tx.send("move_left").unwrap(),
                    Key::Right => tx.send("move_right").unwrap(),
                    Key::Down => tx.send("move_down").unwrap(),
                    Key::Char(' ') => tx.send("rotate").unwrap(),
                    Key::Char('s') => tx.send("init").unwrap(),
                    Key::Ctrl('q') => std::process::exit(0),
                    Key::Ctrl('c') => std::process::exit(0),
                    _ => (),
                }
            }
        });

        while let Ok(event) = rx.recv() {
            print!("{}", termion::clear::All);

            self.tetris.send(event).unwrap();

            print!("{}", termion::cursor::Goto(1, 1));
            print!("{}", self.tetris);
            print!("{:?}:{}\r\n", self.tetris.state, event);
        }

    }
}

fn main() {
    let tetris = Tetris::new();
    let mut tetri_term = TetrisTerm { tetris: tetris };
    tetri_term.run();
}
