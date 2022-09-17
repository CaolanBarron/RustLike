use core::panic;
use std::io::{stdout};
use backend::{player::Player, entity::{Entity, Movement}};
use crossterm::{event::{read, Event, KeyEvent, KeyCode}, ExecutableCommand, style::Print};
use frontend::map::Map;


//  Data concerning what entities are active in the game
struct RuntimeData {
    player: Player,
    entities: Vec<Box<dyn Entity>>,
}

fn gameplay_loop(mut rd: RuntimeData) {
    let mut map = Map{ writer: stdout(), size: (10,10) };
    map.display_base_map().expect("Could not display map");

    loop {
        get_input(&mut rd.player);
        map.refresh_player(&rd.player);
        map.refresh(&rd.entities);
    }
}

fn get_input(player: &mut Player) {
    // Keep looping until user input arrives and then run the corresponding function
    loop {
        let event = read().unwrap();


        if let Event::Key(KeyEvent{ code, modifiers: _, kind: _, state: _ }) = event {

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
}

fn main() {
    let rd = RuntimeData {
        player: Player::new("Stand in player".to_string(),(5,5)),
        entities: vec![],
    };

    gameplay_loop(rd);
}

#[cfg(test)]
mod game_data_tests {
    use backend::enemy::Enemy;

    use super::*;

    fn test_data() -> RuntimeData {
        RuntimeData { 
            player: Player::new("player test".to_string(), 10, (5,5)), 
            entities: (vec![Box::new(
                Enemy::new("test enemy".to_string(), 10, 4, 2, (4,4))
            )]) 
        }
    }

    fn test_map() -> Map {
        Map {
            size: (10, 10),
            writer: stdout(),
        }
    }

    #[test]
    fn test_map_populate() {
        let mut data = test_data();
        let mut map = test_map();

        map.display_base_map().expect("Could not display map");

        map.refresh(&data.entities);
        loop {
            get_input(&mut data.player);
        }
    }
}