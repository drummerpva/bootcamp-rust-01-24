use piston_window::{rectangle, types::Color, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    return (game_coord as f64) * BLOCK_SIZE;
}
pub fn to_coord_u32(game_coord: i32) -> u32 {
    return to_coord(game_coord) as u32;
}

pub fn draw_block(color: Color, x: i32, y: i32, context: &Context, graphic_buffer: &mut G2d) {
    let graphic_coord_x = to_coord(x);
    let graphic_coord_y = to_coord(y);
    rectangle(
        color,
        [graphic_coord_x, graphic_coord_y, BLOCK_SIZE, BLOCK_SIZE],
        context.transform,
        graphic_buffer,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    context: &Context,
    graphic_buffer: &mut G2d,
) {
    let graphic_coord_x = to_coord(x);
    let graphic_coord_y = to_coord(y);
    rectangle(
        color,
        [
            graphic_coord_x,
            graphic_coord_y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        context.transform,
        graphic_buffer,
    );
}
