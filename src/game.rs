use piston_window::*;

use super::paddle::{Paddle};
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
        for key in self.keys.clone() {
            self.key_pressed(key.clone());
        }
        let dir = self.ball.bounds.intersects(&self.paddle.bounds);
        match dir {
            Direction::Empty => self.check_blocks(),
            _ => {
                let xd = (self.ball.bounds.x_center() - self.paddle.bounds.x_center())/self.paddle.bounds.w/2.0;
                self.ball.set_vel(xd*MAX_SPEED, -self.ball.get_vel().1);
            }
        };
        
        self.paddle.update(self.direction.clone(), &self.width);
        self.direction = Direction::Empty;
        self.ball.update(&self.width, &self.height);
    }

    pub fn check_blocks(&mut self){
        let mut i = 0;
        while i != self.blocks.len() {
            let block = &mut self.blocks[i];
            let dir = self.ball.bounds.intersects(&block.bounds);
            match dir {
                Direction::Up | Direction::Down => {self.ball.reverse_y(); self.blocks.remove(i);},
                Direction::Left | Direction::Right => {self.ball.reverse_x(); self.blocks.remove(i);},
                Direction::Empty => i+=1
            };
        }
    }
}