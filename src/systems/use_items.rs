use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesDungeonMap)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut healings_to_apply = Vec::<(Entity, i32)>::new();

    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healings_to_apply.push((activate.used_by, healing.amount));
                }

                if let Ok(_m) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }
            }

            commands.remove(activate.item);
            commands.remove(*entity);
        });

    for healing in healings_to_apply.iter() {
        if let Ok(mut used_by) = ecs.entry_mut(healing.0) {
            if let Ok(health) = used_by.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + healing.1);
            }
        }
    }
}
