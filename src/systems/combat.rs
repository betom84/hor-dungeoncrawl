use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[read_component(Damage)]
#[read_component(Carried)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect::<Vec<(Entity, Entity, Entity)>>();

    victims.iter().for_each(|(message, attacker, victim)| {
        let mut total_damage: i32 = 0;

        total_damage += if let Ok(e) = ecs.entry_ref(*attacker) {
            if let Ok(d) = e.get_component::<Damage>() {
                d.0
            } else {
                0
            }
        } else {
            0
        };

        total_damage += <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, damage)| damage.0)
            .sum::<i32>();

        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= total_damage;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }

        commands.remove(*message);
    });
}
