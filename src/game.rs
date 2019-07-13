use piston_window::*;
use piston_window::types::Color;

use super::paddle::{Paddle};
use super::draw::{draw_rectangle};
use super::block::Block;
use super::ball::Ball;

use std::collections::HashSet;

const MAX_SPEED: f64 = 6.0;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Empty,
}

pub struct Game {
    paddle: Paddle,
    direction: Direction,
    pub keys: HashSet<Key>,
    pub width: f64,
    pub height: f64,
    blocks: Vec<Block>,
    ball: Ball,
}

impl Game {
    pub fn new(width: f64, height: f64) -> Game {
        let mut bvec: Vec<Block> = Vec::new();
        for j in 0..4 {
            for i in 0..4 {
                bvec.push(Block::new((i as f64)*105.0, (j as f64)*25.0+40.));
            }
        }
        Game {
            paddle: Paddle::new(200.0,380.0),
            direction: Direction::Empty,
            keys: HashSet::new(),
            width,
            height,
            blocks: bvec,
            ball: Ball::new(200.0,250.0,0.5,1.0),
        }

    }

    pub fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::Left => Direction::Left,
            Key::Right => Direction::Right,
            _ => Direction::Empty,
        };

        self.direction = dir;
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.paddle.draw(con,g);
        for b in self.blocks.iter() {
            b.draw(con,g);
        }
        self.ball.draw(con,g);
    }

    pub fn update(&mut self, delta_time: f64){
        //let (px,py) = self.ball.get_pos();
        for key in self.keys.clone() {
            self.key_pressed(key.clone());
        }
        if !(self.ball.x > self.paddle.x + self.paddle.w ||
            self.ball.x + self.ball.w < self.paddle.x ||
            self.ball.y > self.paddle.y + self.paddle.h ||
            self.ball.y + self.ball.h < self.paddle.y
        ) { //collision has happened
            let xd = (self.ball.x+self.ball.w/2.0 - (self.paddle.x+self.paddle.w/2.0))/self.paddle.w/2.0;
            self.ball.set_vel(xd*MAX_SPEED, -self.ball.get_vel().1);
        } else { //ball wont collide with paddle and blocks at the same time
            let ballxcenter = (self.ball.x+self.ball.w/2.0);

            let mut  i = 0;
            while i != self.blocks.len() {
                let block = &mut self.blocks[i];
                if !(
                    self.ball.x > block.x + block.w ||
                    self.ball.x + self.ball.w < block.x ||
                    self.ball.y > block.y + block.h ||
                    self.ball.y + self.ball.h < block.y
                ){
                    if (
                        ballxcenter > block.x &&
                        ballxcenter < block.x+block.w 
                    ){
                        self.ball.reverse_y();
                    } else {
                        self.ball.reverse_x();
                    }
                    self.blocks.remove(i);
                    
                } else {
                    i+=1;
                }
            }
        }
        
        self.paddle.update(self.direction.clone(), &self.width);
        self.direction = Direction::Empty;
        self.ball.update(&self.width, &self.height);
    }

}