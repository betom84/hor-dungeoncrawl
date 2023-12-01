use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn: &mut TurnState) {
    *turn = match turn {
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        _ => TurnState::AwaitingInput,
    }
}
