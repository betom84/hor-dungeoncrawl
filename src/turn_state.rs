// use crate::prelude::*;

//#[derive(Copy, Clone, Debug, PartialEq)]
#[derive(Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}
