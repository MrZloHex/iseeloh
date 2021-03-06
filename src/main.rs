use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

use termion::screen::*;

#[derive(Clone)]
struct Location {
    x: usize,
    y: usize
}

#[derive(Clone)]
struct Player {
    location: Location,
    direction: f32
}

impl Player {
    pub fn new() -> Player {
        Player {
            location: Location { x:0, y:0 },
            direction: 0.0
        }
    }
}

#[derive(Clone)]
enum TileType {
    Empty,
    Player(Player),
    Wall
}

struct Map {
    field: Vec<Vec<TileType>>,
    max_x: usize,
    max_y: usize,
}

impl Map {
    pub fn generate_map(map_s: String) -> Map {
        let mut max_x: usize = 0;
        let mut max_y: usize = 0;
        let mut x: usize = 0;
        for ch in map_s.bytes() {
            if ch == 10 {
                if max_x < x { max_x = x }
                x = 0;
                max_y += 1;
            } else {
                x += 1;
            }
        }
        drop(x);

        
        let mut field: Vec<Vec<TileType>> = vec![vec![TileType::Empty; max_y]; max_x];


        let mut x: usize = 0;
        let mut y: usize = 0;
        for ch in map_s.bytes() {
            println!("Char: {} X: {} Y: {}", ch as char, x, y);
            match ch {
                35 => field[x][y] = TileType::Wall,
                88 => field[x][y] = TileType::Player(Player::new()),
                32 => field[x][y] = TileType::Empty,
                10 => {
                    x = 0;
                    y += 1;
                    continue;
                },
                _ => {
                    eprintln!("ERROR: unknown char in map at: {}:{}", x, y);
                    std::process::exit(1);
                }
            }
            x += 1;
        }


            
        Map {
            field,
            max_x,
            max_y
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: not provided filepath to map");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => {
            eprintln!("ERROR: couldn't open {}: {}", display, why);
            std::process::exit(1);
        },
        Ok(file) => file,
    };

    let mut map_s = String::new();
    match file.read_to_string(&mut map_s) {
        Err(why) => {
            eprintln!("ERROR: couldn't read {}: {}", display, why);
            std::process::exit(1);
        },
        Ok(_) => {}
    }


    let map = Map::generate_map(map_s);





    // let stdin = stdin();
    // let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    // write!(stdout,
    //        "{}{}q to exit. Click, click, click!",
    //        termion::clear::All,
    //        termion::cursor::Goto(1, 1))
    //         .unwrap();
    // stdout.flush().unwrap();

    // for c in stdin.events() {
    //     let evt = c.unwrap();
    //     match evt {
    //         Event::Key(Key::Char('q')) => break,
    //         Event::Mouse(me) => {
    //             match me {
    //                 MouseEvent::Press(_, x, y) => {
    //                     write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
    //                 }
    //                 _ => (),
    //             }
    //         }
    //         _ => {}
    //     }
    //     stdout.flush().unwrap();
    // }
        
}
