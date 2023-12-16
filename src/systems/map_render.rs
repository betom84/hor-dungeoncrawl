use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let tile_idx = map_idx(x, y);
            if tile_idx.is_none() {
                continue;
            }

            let pt = Point::new(x, y);
            if !player_fov.visible_tiles.contains(&pt) && !map.revealed_tiles[tile_idx.unwrap()] {
                continue;
            }

            if let Some(tile) = map.tiles.get(tile_idx.expect("can't be none")) {
                let offset = Point::new(camera.left_x, camera.top_y);
                let glyph = theme.tile_to_render(*tile);

                let tint = if player_fov.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARK_GRAY
                };

                draw_batch.set(pt - offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
