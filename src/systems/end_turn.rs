use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn: &mut TurnState) {
    let mut player = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_pos = amulet.iter(ecs).next().unwrap();

    let mut new_state = match turn {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => turn.clone(),
    };

    player.iter(ecs).for_each(|(health, pos)| {
        if health.current < 1 {
            new_state = TurnState::GameOver;
        }

        if pos == amulet_pos {
            new_state = TurnState::Victory;
        }
    });

    *turn = new_state;
}
