use backend::{entity::{Entity, Character}, player::Player};
use core::panic;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use frontend::Map::Map;
use fundamentals::{position::Position, pos};
use runtime_data::RuntimeData;
use std::io::stdout;
mod runtime_data;

fn main() {
    let mut rd = RuntimeData::new(Player::new("Stand in player".to_string(), 5, 5));
    let map = Map::new((10, 10), stdout());
    rd.generate_level(
        &map,
        Position::new(0, 0),
        Position::new(map.size.0.into(), map.size.1.into()),
    );

    gameplay_loop(map, rd);
}

fn gameplay_loop(mut map: Map, mut rd: RuntimeData) {
    map.display_base_level(&rd.level)
        .expect("Could not display level");

    loop {
        get_input(&mut rd, &map);
        map.refresh_entities(&rd.entities);
        map.refresh_player(&rd.player, *rd.level.get(&rd.player.previous_position()).unwrap())
    }
}

fn get_input(rd: &mut RuntimeData, map: &Map) {
    // Keep looping until user input arrives and then run the corresponding function
    let event = read().unwrap();

    if let Event::Key(KeyEvent {
        code,
        modifiers: _,
        kind: _,
        state: _,
    }) = event
    {
        match code {
            KeyCode::Left => rd.player.move_left(),
            KeyCode::Right => rd.player.move_right(),
            KeyCode::Up => rd.player.move_up(),
            KeyCode::Down => rd.player.move_down(),
            KeyCode::Esc => panic!("Escaping"),
            _ => (),
        }
    }
}

fn detect_collision(space: char) -> bool {
    if space != '\u{2592}' {
        return false;
    }
    true
}
