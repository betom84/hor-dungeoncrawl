use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    intention: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(intention.destination) {
        commands.add_component(intention.entity, intention.destination);

        if ecs
            .entry_ref(intention.entity)
            .expect("entity must exist")
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_move(intention.destination);
        }
    }

    commands.remove(*entity);
}
