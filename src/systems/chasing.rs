use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn chasing(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] map: &Map) {
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    let mut attackables = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_pos = player.iter(ecs).next().unwrap().0;
    let player_map_idx = map_idx(player_pos.x, player_pos.y).expect("Player must be on the map");

    let search_tarets = vec![player_map_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_tarets, map, 1024f32);

    movers.iter(ecs).for_each(|(entity, pos, _, fov)| {
        if !fov.visible_tiles.contains(player_pos) {
            return;
        }

        let idx = map_idx(pos.x, pos.y).unwrap();

        if let Some(dest_idx) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            let destination = if distance > 1.2 {
                map.index_to_point2d(dest_idx)
            } else {
                *player_pos
            };

            let mut should_move = true;
            attackables
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));
                    }

                    should_move = false;
                });

            if should_move {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    });
}
