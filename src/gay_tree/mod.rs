use std::collections::HashSet;

use crate::gay_quad::Borders;

use egui::Vec2;
use macroquad::miniquad::gl::GL_TIME_ELAPSED;

pub struct point
{
    pub id: usize,
    pub pos:Vec2,
}

pub struct GayTree {
    pub boundary: Borders,
    pub capacity: usize,
    pub points: Option<Vec<point>>,
    pub nw: Option<Box<GayTree>>,
    pub ne: Option<Box<GayTree>>,
    pub sw: Option<Box<GayTree>>,
    pub se: Option<Box<GayTree>>,
}

impl GayTree {
    pub fn new(boundary: Borders, capacity: usize) -> GayTree {
        GayTree {
            boundary,
            capacity,
            points: None,
            nw: None,
            ne: None,
            sw: None,
            se: None,
        }
    }

    pub fn insert(&mut self, point: point) -> bool {
        if !self.boundary_contains_point(&point) {
            //println!("point position is: {} {}", point.pos.x, point.pos.y);
            //println!("my sw is: {} {}", self.boundary.SW.x, self.boundary.SW.y);
            //println!("my ne is: {} {}", self.boundary.NE.x, self.boundary.NE.y);
            return false;
        }

        if self.points.is_none() {
            self.points = Some(Vec::new());
        }

        if self.points.as_ref().unwrap().len() < self.capacity {
            self.points.as_mut().unwrap().push(point);
            return true;
        } else {
            if self.nw.is_none() {
                self.subdivide();
            }

            if self.nw.as_mut().unwrap().insert(point { id: (point.id), pos: (point.pos) }) {
                return true;
            }
            if self.ne.as_mut().unwrap().insert(point { id: (point.id), pos: (point.pos) }) {
                return true;
            }
            if self.sw.as_mut().unwrap().insert(point { id: (point.id), pos: (point.pos) }) {
                return true;
            }
            if self.se.as_mut().unwrap().insert(point { id: (point.id), pos: (point.pos) }) {
                return true;
            }
        }
        println!("not inserted");
        false
    }

    pub fn retrieve(&self, center: &Vec2, radius: f32) -> Vec<usize> {
        let mut result = Vec::new();

        if !self.intersects_circle(center, radius) {
            return result;
        }

        if(self.se.is_some()) {
            result.append(self.nw.as_ref().unwrap().retrieve(center, radius).as_mut());
            result.append(self.ne.as_ref().unwrap().retrieve(center, radius).as_mut());
            result.append(self.sw.as_ref().unwrap().retrieve(center, radius).as_mut());
            result.append(self.se.as_ref().unwrap().retrieve(center, radius).as_mut());
        }

        if self.points.is_some() {
            for point in self.points.as_ref().unwrap() {
                if self.contains_point_in_circle(&point.pos, center, radius) {
                    result.push(point.id);
                }
            }
        } 

        result
    }

    fn subdivide(&mut self) {
        let x = (self.boundary.NE.x + self.boundary.SW.x) / 2.0;
        let y = (self.boundary.NE.y + self.boundary.SW.y) / 2.0;
        let half_width = (self.boundary.NE.x - self.boundary.SW.x).abs() / 2.0;
        let half_height = (self.boundary.NE.y - self.boundary.SW.y).abs() / 2.0;

        self.nw = Some(Box::new(GayTree::new(
            Borders {
                NE: Vec2 { x, y: y + half_height },
                NW: self.boundary.NW,
                SW: Vec2 { x: x - half_width, y },
                SE: Vec2 { x, y},
            },
            self.capacity,
        )));
    
        self.ne = Some(Box::new(GayTree::new(
            Borders {
                NE: self.boundary.NE,
                NW: Vec2 { x, y: y+half_height },
                SW: Vec2 { x, y },
                SE: Vec2 { x: x + half_width, y},
            },
            self.capacity,
        )));
    
        self.sw = Some(Box::new(GayTree::new(
            Borders {
                NE: Vec2 { x, y },
                NW: Vec2 { x: x - half_width, y },
                SW: self.boundary.SW,
                SE: Vec2 { x, y: y - half_height },
            },
            self.capacity,
        )));
    
        self.se = Some(Box::new(GayTree::new(
            Borders {
                NE: Vec2 { x: x + half_width, y },
                NW: Vec2 { x, y},
                SW: Vec2 { x, y: y - half_height },
                SE: self.boundary.SE,
            },
            self.capacity,
        )));
    }
    
    fn boundary_contains_point(&self, point: &point) -> bool {
        self.boundary.NE.x >= point.pos.x
            && self.boundary.SW.x <= point.pos.x
            && self.boundary.NE.y >= point.pos.y
            && self.boundary.SW.y <= point.pos.y
    }
    
    fn check_intersection(circle: &Vec2, pos: &Vec2, radius: f32) -> bool
    {
        let dx = circle.x-pos.x;
        let dy = circle.y-pos.y;
        (dx*dy + dy*dy) < radius*radius
    }

    fn intersects_circle(&self, center: &Vec2, radius: f32) -> bool {
        Self::check_intersection(center, &self.boundary.NE, radius) ||
        Self::check_intersection(center, &self.boundary.SE, radius) ||
        Self::check_intersection(center, &self.boundary.NW, radius) ||
        Self::check_intersection(center, &self.boundary.SW, radius)
    }
    
    fn contains_point_in_circle(&self, point: &Vec2, center: &Vec2, radius: f32) -> bool {
        let dx = center.x - point.x;
        let dy = center.y - point.y;
    
        dx * dx + dy * dy <= radius * radius
    }
}    