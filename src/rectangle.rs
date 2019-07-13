use super::game::Direction;

pub struct Rectangle {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Rectangle { x,y,w,h }
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x > self.x 
            && p.x < self.w+self.x 
            && p.y > self.y 
            && p.y < self.y+self.h
    }

    pub fn intersects(&self, r: Rectangle) -> {
        /*
        check if self intersects r. return Direction::Empty if no intersect,
        otherwise return the direction of intersection. 
        so if self intersects from above, return Direction::Up
        */

        if !(
            self.x > r.x + r.w ||
            self.x + self.w < r.x ||
            self.y > r.y + r.h ||
            self.y + self.h < r.y
        ){
            let (xn,yn) = self.normal(r);
            if xn < 0 { //self's center is to the left of r's center 
                
            } 
            if self.y > r.y+r.h { return Direction::Down } 
            if self.x < r.x { return Direction::Up } 
            if self.x > r.x { return Direction::Up } 
        }
        Direction::Empty
    }

    pub fn normal(&self, r: Rectangle){
        (
            (self.x+self.w/2.0) - (r.x+r.w/2.0), 
            (self.y+self.h/2.0)-(r.y+r.h/2.0)
        )
    }
}