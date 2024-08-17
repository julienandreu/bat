use std::time::Duration;

use bevy::{
    input::gamepad::{GamepadRumbleIntensity, GamepadRumbleRequest},
    prelude::*,
};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};

use crate::player::{Action, Player};

use super::JoinedPlayers;

pub fn join(
    mut commands: Commands,
    mut joined_players: ResMut<JoinedPlayers>,
    mut evw_rumble: EventWriter<GamepadRumbleRequest>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
) {
    for gamepad in gamepads.iter() {
        // Join the game when both bumpers (L+R) on the controller are pressed
        // We drop down the Bevy's input to get the input from each gamepad
        if button_inputs.pressed(GamepadButton::new(
            gamepad,
            GamepadButtonType::Start,
        )) {
            // Make sure a player cannot join twice
            if !joined_players.0.contains_key(&gamepad) {
                info!("Player {} has joined the game!", gamepad.id);

                evw_rumble.send(GamepadRumbleRequest::Add {
                    gamepad,
                    duration: Duration::from_millis(500),
                    intensity: GamepadRumbleIntensity {
                        strong_motor: 0.25,
                        weak_motor: 0.5,
                    },
                });

                evw_rumble.send(GamepadRumbleRequest::Add {
                    gamepad,
                    duration: Duration::from_millis(1000),
                    intensity: GamepadRumbleIntensity::MAX,
                });

                let input_map = InputMap::new([
                    (Action::Jump, GamepadButtonType::South),
                    (Action::Disconnect, GamepadButtonType::Select),
                ])
                // Make sure to set the gamepad or all gamepads will be used!
                .with_gamepad(gamepad);
                let player = commands
                    .spawn(InputManagerBundle::with_map(input_map))
                    .insert(Player { gamepad })
                    .id();

                // Insert the created player and its gamepad to the hashmap of joined players
                // Since uniqueness was already checked above, we can insert here unchecked
                joined_players
                    .0
                    .insert_unique_unchecked(gamepad, player);
            }
        }
    }
}

pub fn jump(action_query: Query<(&ActionState<Action>, &Player)>) {
    // Iterate through each player to see if they jumped
    for (action_state, player) in action_query.iter() {
        if action_state.just_pressed(&Action::Jump) {
            info!("Player {} jumped!", player.gamepad.id);
        }
    }
}

pub fn disconnect(
    mut commands: Commands,
    action_query: Query<(&ActionState<Action>, &Player)>,
    mut joined_players: ResMut<JoinedPlayers>,
) {
    for (action_state, player) in action_query.iter() {
        if action_state.pressed(&Action::Disconnect) {
            let player_entity =
                *joined_players.0.get(&player.gamepad).unwrap();

            // Despawn the disconnected player and remove them from the joined player list
            commands.entity(player_entity).despawn();
            joined_players.0.remove(&player.gamepad);

            info!("Player {} has disconnected!", player.gamepad.id);
        }
    }
}
