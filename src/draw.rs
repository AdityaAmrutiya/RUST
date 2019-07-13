use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn draw_rectangle(
    color: Color,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    con: &Context,
    g: &mut G2d,
){
    //let gui_x = x;
    //let gui_y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            width,
            height,
        ],
        con.transform,
        g,
    );
}