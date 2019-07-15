use piston_window::{Context, G2d};
use piston_window::types::Color;
use super::rectangle::Rect;
use super::draw::draw_rectangle;
use super::game::Game;

const PADDLE_COLOR: Color = [0.8,0.2,0.2,1.0];
const BALL_SIZE: f64 = 10.0;

pub struct Ball {
    pub bounds: Rect,
    xv: f64,
    yv: f64
}

impl Ball {
    pub fn new(x:f64, y:f64, xv:f64, yv:f64) -> Self {
        let b = Rect::new(x,y,BALL_SIZE,BALL_SIZE);
        Ball {
            bounds:b,
            xv,
            yv
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d){
        draw_rectangle(PADDLE_COLOR, self.bounds.origin.x, self.bounds.origin.y, self.bounds.w, self.bounds.h, con, g);
    }

    pub fn update(&mut self, w: &f64, h: &f64){
        if self.bounds.origin.x > *w || self.bounds.origin.x < 0.0 {
            self.xv = -self.xv;
        }
        if self.bounds.origin.y > *h || self.bounds.origin.y < 0.0 {
            self.yv = -self.yv;
        }
        self.bounds.origin.x += self.xv;
        self.bounds.origin.y += self.yv;
    }

    pub fn reverse_dir(&mut self) {
        self.reverse_x();
        self.reverse_y();
    }

    pub fn reverse_y(&mut self) {
        self.yv = -self.yv;
    }

    pub fn reverse_x(&mut self) {
        self.xv = -self.xv;
    }

    pub fn get_pos(&self) -> (f64, f64) {
        (self.bounds.origin.x,self.bounds.origin.y)
    }

    pub fn get_vel(&self) -> (f64, f64) {
        (self.xv, self.yv)
    }

    pub fn set_vel(&mut self, xv:f64, yv:f64) {
        self.xv = xv;
        self.yv = yv;
    }

}