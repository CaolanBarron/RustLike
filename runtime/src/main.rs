use backend::{entity::{Entity, Character}, player::Player};
use core::panic;
use crossterm::{event::{read, Event, KeyCode, KeyEvent}, terminal::disable_raw_mode};
use frontend::{map::Map, UI};
use fundamentals::{position::Position, pos};
use runtime_data::RuntimeData;
use std::io::stdout;
mod runtime_data;

fn main() {
    let mut rd = RuntimeData::new(Player::new("Stand in player".to_string(), 60, 8));
    let mut ui = UI::build((0,0), 0);
    rd.generate_level(
        ui.map(),
        ui.map().start_position,
        ui.map().end_position,
    );

    for _ in 0..3 {
        rd.add_enemy();
    }
    gameplay_loop(ui, rd);
}

fn gameplay_loop(mut ui: UI, mut rd: RuntimeData) {
    ui.init_term();
    ui.map().display_base_level(&rd.level,&mut stdout())
        .expect("Could not display level");
    loop {
        get_input(&mut rd);
        ui.map().refresh_entities(&rd.entities, &mut stdout());
        ui.map().refresh_player(&rd.player, *rd.level.get(&rd.player.previous_position()).unwrap(), &mut stdout())
    }
}

fn get_input(rd: &mut RuntimeData) {
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
            KeyCode::Left => {
                let p = rd.player.position() + pos!(-1,0);
                if Player::walkable(rd.level.get(&p)) {
                    rd.player.move_left()
                }
            }
            KeyCode::Right => {
                let p = rd.player.position() + pos!(1,0);
                if Player::walkable(rd.level.get(&p)) {
                    rd.player.move_right()
                }
            }

            KeyCode::Up => {
                let p = rd.player.position() + pos!(0,-1);
                if Player::walkable(rd.level.get(&p)) {
                    rd.player.move_up()
                }
            }

            KeyCode::Down => {
                let p = rd.player.position() + pos!(0,1);
                if Player::walkable(rd.level.get(&p)) {
                    rd.player.move_down()
                }
            }

            KeyCode::Esc => disable_raw_mode().expect("Shouldnt crash"),
            _ => (),
        }
    }
}
