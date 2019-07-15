use super::game::Direction;

pub struct Point {
    pub x: f64,
    pub y: f64
}

pub struct Rect {
    pub origin: Point,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        let p = Point {x,y};
        Rect { origin:p,w,h }
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x > self.origin.x 
            && p.x < self.w+self.origin.x 
            && p.y > self.origin.y 
            && p.y < self.origin.y+self.h
    }

    pub fn center(&self) -> Point {
        Point {
            x: self.x_center(),
            y: self.origin.y+self.h/2.
        }
    }

    pub fn x_center(&self) -> f64 {
        self.origin.x+self.w/2.
    }

    pub fn intersects(&self, r: &Rect) -> Direction {
        /*
        check if self intersects r. return Direction::Empty if no intersect,
        otherwise return the direction of intersection. 
        so if self intersects from above, return Direction::Up
        */

        if !(
            self.origin.x > r.origin.x + r.w ||
            self.origin.x + self.w < r.origin.x ||
            self.origin.y > r.origin.y + r.h ||
            self.origin.y + self.h < r.origin.y
        ){
            let (xd,yd) = self.center_difference(r);
            //now we need to adjust based on width/height
            //if abs(xd) < abs(yd) but the width of the rects is much smaller than height, 
            //the collision may be in the y direction
            let y_axis = self.center().x > r.origin.x && xd < r.origin.x + r.w; //if xd between x bounds of r, then probably colliding from above or below
            if y_axis {
                if yd < 0. { return Direction::Up } 
                return Direction::Down
            } else {    
                if xd < 0. { return Direction::Left } //self's center is to the left of r's center 
                return Direction::Right
            }
        }
        Direction::Empty
    }

    pub fn center_difference(&self, r: &Rect) -> (f64, f64) { //should be a point?
        let self_c = self.center();
        let r_c = r.center();
        (self_c.x - r_c.x, self_c.y - r_c.y)
    }
}