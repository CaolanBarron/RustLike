use core::panic;
use std::io::{stdout};
use backend::{player::Player, entity::Entity};
use crossterm::{event::{read, Event, KeyEvent, KeyCode}, ExecutableCommand, style::Print};
use frontend::map::Map;


//  Data concerning what entities are active in the game
struct RuntimeData {
    player: Player,
    entities: Vec<Box<dyn Entity>>,
}

fn gameplay_loop(rd: RuntimeData) {
    let mut map = Map{entities: vec![], writer: stdout()};
    map.display_map().expect("Could not display map");

    loop {
        get_input();
    }
}

fn get_input() {
    // Keep looping until user input arrives and then run the corresponding function
    loop {
        let event = read().unwrap();


        if let Event::Key(KeyEvent{ code, modifiers: _, kind: _, state: _ }) = event {
            if code == KeyCode::Esc {
                panic!("Escaping");
            }
        }
        
    }
}

fn main() {
    let rd = RuntimeData {
        player: todo!(),
        entities: todo!(),
    };

    gameplay_loop(rd);
}