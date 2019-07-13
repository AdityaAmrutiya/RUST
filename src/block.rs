use piston_window::{Context, G2d};
use piston_window::types::Color;

use super::draw::draw_rectangle;

const BW: f64 = 100.0;
const BH: f64 = 20.0;
const BLOCK_COLOR: Color = [0.6,0.9,0.9,1.0];

pub struct Block {
    pub x: f64, 
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub kill: bool,
}

impl Block {
    pub fn new(x: f64, y: f64) -> Block {
        Block { x,y,w: BW, h: BH, kill: false}
    }

    pub fn draw(&self, con: &Context, g: &mut G2d){
        draw_rectangle(BLOCK_COLOR, self.x, self.y, BW, BH, con, g);
    }

}