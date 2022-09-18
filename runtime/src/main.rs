use backend::{
    entity::Movement,
    player::Player,
};
use runtime_data::RuntimeData;
use core::panic;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
};
use frontend::map::{Map};
use fundamentals::position::Position;
use std::{io::stdout};

mod runtime_data;

fn gameplay_loop(mut rd: self::runtime_data::RuntimeData) {
    let mut map = Map::new((10, 10), stdout());
    rd.generate_level(
        &map,
        Position::new(0, 0),
        Position::new(map.size.0.into(), map.size.1.into()),
    );
    map.display_base_level(rd.level).expect("Could not display level");
    loop {
        get_input(&mut rd.player);
        map.refresh_player(&rd.player);
        map.refresh(&rd.entities);
    }
}

fn get_input(player: &mut Player) {
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
            KeyCode::Left => player.move_left(),
            KeyCode::Right => player.move_right(),
            KeyCode::Up => player.move_up(),
            KeyCode::Down => player.move_down(),
            KeyCode::Esc => panic!("Escaping"),
            _ => (),
        }
    }
}

fn main() {
    let rd = RuntimeData::new(Player::new("Stand in player".to_string(), (5, 5)));
    gameplay_loop(rd);
}

#[cfg(test)]
mod game_data_tests {
    use super::*;

    fn test_data() -> RuntimeData {
        RuntimeData::new(Player::new("player test".to_string(), (5, 5)))
    }

    fn test_map() -> Map {
        Map::new((10, 10), stdout())
    }

    #[test]
    fn level_display_test() {
        let mut map = test_map();
        let mut data = test_data();

        data.generate_level(
            &map,
            Position::new(0, 0),
            Position::new(map.size.0.into(), map.size.1.into()),
        );

        map.display_base_level(data.level);
        // loop{}
    }
}
