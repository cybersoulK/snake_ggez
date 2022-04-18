use core::slice::Iter;

use crate::engine;

use super::grid::Grid;
use super::grid_drawable::GridDrawable;

use super::settings;


pub struct Food {
    pub object : GridDrawable,
}

pub struct FoodHandler {
    pub food: Vec<Food>,
}

const FOOD_OBJECT_TEMPLATE: engine::Object = engine::Object {
    position: cgmath::Point2 { x: 0.0, y: 0.0 },
    size: cgmath::Point2 { x: 0.0, y: 0.0 },
    angle: 0.0,
    color: [0.9, 0.5, 0.1, 0.80],
    mesh: engine::Mesh::Circle,
};

const FOOD_SIZE_MULTIPLIER: f32 = 0.7;

impl Food {
    pub fn new(position: cgmath::Point2<i32>) -> Food {

        Food { 
            object: GridDrawable::new(FOOD_OBJECT_TEMPLATE, position, FOOD_SIZE_MULTIPLIER),
         }
    }

    pub fn update_object(&mut self, grid: &Grid) {
        self.object.update_object(grid, None, None);
    }
}

impl FoodHandler {
    pub fn new() -> FoodHandler {
        FoodHandler {
            food: Vec::new(),
        }
    }

    pub fn get_objects(&self) -> Vec<&GridDrawable> {
        let mut vec = Vec::new();

        for obj in self.food.iter() {
            vec.push(&obj.object);
        }

        vec
    }

    pub fn update_objects(&mut self, grid: &Grid){

        for obj in self.food.iter_mut() {
            obj.update_object(grid);
        }
    }

    pub fn is_low(&self) -> bool {

        if (self.food.len() as i32) < settings::FOOD_VECTOR_SIZE {
            return true;
        }

        false
    }

    pub fn spawn(&mut self, snake_objects: Iter<&GridDrawable>) {

        let new_position = settings::GRID_SIZE
                .map(|c| (c as f32 * rand::RandomRange::gen_range(0.0, 1.0)).floor() as i32);


        for obj in snake_objects.chain(self.get_objects().iter()) {
            if new_position == obj.position { return; }
        } 

        let new_food = Food::new(new_position);
        self.food.push(new_food);
    }

    pub fn remove(&mut self, index: usize) {
        self.food.remove(index);
    }
}