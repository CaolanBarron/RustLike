use backend::{
    entity::{Character, Mapable},
    player::Player,
};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal::disable_raw_mode,
};
use frontend::UI;
use fundamentals::{pos, position::Position};
use runtime_data::RuntimeData;
use std::io::stdout;
mod runtime_data;

fn main() {
    let mut rd = RuntimeData::new(Player::new("Stand in player".to_string(), 60, 8));
    let mut ui = UI::build((0, 0), 0);
    rd.generate_level(ui.map(), ui.map().start_position, ui.map().end_position);

    for _ in 0..4 {
        rd.add_entity();
    }
    gameplay_loop(ui, rd);
}

fn gameplay_loop(mut ui: UI, mut rd: RuntimeData) {
    ui.init_term();
    ui.map()
        .display_base_level(&rd.level, &mut stdout())
        .expect("Could not display level");
    loop {
        ui.map().refresh_entities(&rd.entities, &mut stdout());
        ui.map().refresh_player(
            &rd.player,
            *rd.level.get(&rd.player.previous_position()).unwrap(),
            &mut stdout(),
        );

        //Check player surroundings
        check_player_surroundings(&mut rd, &ui);
        get_input(&mut rd);
    }
}

fn check_player_surroundings(rd: &mut RuntimeData, ui: &UI) {
    match rd.player.look(&rd.entities) {
        Some(input) => {
            ui.character_portrait().clear_portrait();
            ui.dialogue_box().clear_box();
            ui.dialogue_box().draw_title(&input[rd.targeted].name());
            ui.dialogue_box()
                .draw_options(&input[rd.targeted].dialogue(), &rd.choice);
            ui.character_portrait()
                .draw_portrait(&input[rd.targeted].avatar());
            ui.map().target_character(&input[rd.targeted].position())
        }

        None => {
            ui.dialogue_box().clear_box();
            ui.character_portrait().clear_portrait();
            rd.targeted = 0;
            rd.choice = 0;
        }
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
            KeyCode::Esc => disable_raw_mode().expect("Shouldnt crash"),

            KeyCode::Left => {
                rd.targeted = 0;
                let p = rd.player.position() + pos!(-1, 0);

                if Player::walkable(rd.level.get(&p))
                    && !rd.entities.iter().any(|x| x.position() == p)
                {
                    rd.player.move_left()
                }
            }
            KeyCode::Right => {
                rd.targeted = 0;
                let p = rd.player.position() + pos!(1, 0);

                if Player::walkable(rd.level.get(&p))
                    && !rd.entities.iter().any(|x| x.position() == p)
                {
                    rd.player.move_right()
                }
            }

            KeyCode::Up => {
                rd.targeted = 0;
                let p = rd.player.position() + pos!(0, -1);

                if Player::walkable(rd.level.get(&p))
                    && !rd.entities.iter().any(|x| x.position() == p)
                {
                    rd.player.move_up()
                }
            }

            KeyCode::Down => {
                rd.targeted = 0;
                let p = rd.player.position() + pos!(0, 1);

                if Player::walkable(rd.level.get(&p))
                    && !rd.entities.iter().any(|x| x.position() == p)
                {
                    rd.player.move_down()
                }
            }

            KeyCode::Char('n') => {
                rd.choice = 0;
                let mut range = 0;
                match rd.player.look(&rd.entities) {
                    Some(input) => range = input.len(),
                    None => return (),
                }

                rd.targeted = (rd.targeted + 1) % range;
            }

            KeyCode::Char('i') => {
                rd.player.inventory_choice = (rd.choice + 1) % 3;
            }

            KeyCode::Char('1') => panic!("One"),
            KeyCode::Char('2') => panic!("Two"),
            KeyCode::Char('3') => panic!("Three"),
            KeyCode::Char('4') => panic!("Four"),
            KeyCode::Char('5') => panic!("Five"),

            KeyCode::Enter => match rd.player.look(&rd.entities) {
                Some(input) => panic!("Interaction not made"),
                None => todo!(),
            },

            _ => (),
        }
    }
}
