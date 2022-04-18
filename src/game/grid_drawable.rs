use core::ops::{Sub};

use crate::engine;
use super::grid::Grid;

pub struct GridDrawable {
    pub object: engine::Object,
    pub position: cgmath::Point2<i32>,
    pub size_muliplier: f32,
}

impl GridDrawable {

    pub fn new(object: engine::Object, position: cgmath::Point2<i32>, size_muliplier: f32) -> GridDrawable {
        GridDrawable {
            object, 
            position,
            size_muliplier,
        }
    }
    
    pub fn update_object(&mut self, grid: &Grid, direction: Option<cgmath::Vector2<i32>>, transition: Option<f32>){

        let (mut position, size) = grid.to_screen(self.position, self.size_muliplier);

        if let (Some(direction), Some(transition)) = (direction, transition) {
            let (position2, _size2) = grid.to_screen(self.position.sub(direction), self.size_muliplier);

            position = position2 + (position - position2) * transition;
        }

        self.object.position = position;
        self.object.size = size;
    }
}