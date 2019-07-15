use piston_window::{Context, G2d};
use piston_window::types::Color;
use super::rectangle::Rect;
use super::draw::draw_rectangle;
use super::game::{Direction}; 

const PADDLE_COLOR: Color = [0.9,0.9,0.9,1.0];
const SPEED: f64 = 10.0;
const PW: f64 = 40.0;
const PH: f64 = 10.0;

pub struct Paddle {
    pub bounds: Rect,
    v:f64,
    a:f64,
}

impl Paddle {
    pub fn new(x:f64, y:f64) -> Paddle {
        Paddle {
            bounds: Rect::new(x,y,PW,PH),
            v:0.,
            a:0.
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d){
        draw_rectangle(PADDLE_COLOR, self.bounds.origin.x, self.bounds.origin.y, self.bounds.w, self.bounds.h, con, g);
    }

    pub fn update(&mut self, dir: Direction, width:&f64){
        if self.bounds.origin.x < 0. {
            self.bounds.origin.x = 0.;
            self.v = 0.;
            return
        }
        if self.bounds.origin.x+self.bounds.w > *width {
            self.bounds.origin.x = *width - self.bounds.w;
            self.v = 0.;
            return
        }

        match dir {
            Direction::Left => (
                self.v -= 1.5
            ),
            Direction::Right => (
                self.v += 1.5
            ),
            _ => ( self.a = 0. )
        };
        if self.a == 0. {
            if self.v > 1. {
                self.v /= 2.
            } else if self.v < -1. {
                self.v /= 2. //apply breaks
            } else {
                self.v = 0.;
            }
        } 
        self.v += self.a;
        self.bounds.origin.x += self.v;
    }
}