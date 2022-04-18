use core::{ops::{AddAssign, Mul, Sub}, slice::Iter};

use super::{grid::Grid};
use super::grid_drawable::GridDrawable;

use crate::engine::{self, Mesh};
use super::settings;


pub struct Snake {
    pub body: Vec<SnakeBody>,
    direction: SnakeDirection,
}

pub struct SnakeBody {
    pub object: GridDrawable,
    direction: SnakeDirection,
}

#[derive(Clone, Copy)]
pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
    None,
}

const INITIAL_DIRECTION: SnakeDirection = SnakeDirection::Right;

const SNAKE_OBJECT_TEMPLATE: engine::Object = engine::Object {
    position: cgmath::Point2 { x: 0.0, y: 0.0 },
    size: cgmath::Point2 { x: 1.0, y: 1.0 },
    angle: 0.0,
    color: [0.1, 0.4, 0.92, 1.0],
    mesh: SNAKE_BODY_FULL_MESH,
};

const SNAKE_HEAD_MESH: Mesh = Mesh::Rectangle;
const SNAKE_HEAD_SIZE_MULTIPLIER: f32 = 1.1;
const SNAKE_HEAD_COLOR: [f32; 4] = [0.2, 0.7, 1.0, 1.0];

const SNAKE_BODY_MESH: Mesh = Mesh::Rectangle;
const SNAKE_BODY_SIZE_MULTIPLIER: f32 = 1.0;
const SNAKE_BODY_ALPHA : f32 = 0.9;
const SNAKE_BODY_VARIATION: f32 = 0.3;

const SNAKE_BODY_FULL_MESH: Mesh = Mesh::RoundedRectangle;
const SNAKE_BODY_FULL_SIZE_MULTIPLIER: f32 = 1.3;
const SNAKE_BODY_FULL_ALPHA : f32 = 0.9;


impl Snake {

    pub fn new(position_start: cgmath::Point2<i32>, length: i32) -> Snake {

        let mut body = Vec::<SnakeBody>::new();
        
        {
            let mut head_object = SNAKE_OBJECT_TEMPLATE;
            head_object.mesh = SNAKE_HEAD_MESH;
            head_object.color = SNAKE_HEAD_COLOR;

            let head = SnakeBody {
                object: GridDrawable::new(
                    head_object,
                    position_start,
                    SNAKE_HEAD_SIZE_MULTIPLIER),
                direction: SnakeDirection::Right,
            };
            
            body.push(head);
        }
        

        let mut new_snake = Snake { 
            body,
            direction: INITIAL_DIRECTION,
        };

        for _ in 0..length-1 { new_snake.create_body(); }
        
        new_snake
    }

    pub fn create_body(&mut self){

        let head = self.body.first().unwrap();

            let position = head.object.position.sub(head.direction.to_vector());

            let mut new_object = SNAKE_OBJECT_TEMPLATE;

            new_object.color[3] = SNAKE_BODY_ALPHA;
            new_object.color = new_object.color.map(|c| c - SNAKE_BODY_VARIATION / 2.0 + SNAKE_BODY_VARIATION * rand::RandomRange::gen_range(0.0, 1.0));

            let new_body = SnakeBody {
                object: GridDrawable::new(
                    new_object,
                    position, 
                    SNAKE_BODY_FULL_SIZE_MULTIPLIER),
                direction: SnakeDirection::None,
            };

            self.body.insert(1, new_body);
    }

    pub fn get_objects(&self) -> Vec<&GridDrawable> {
        let mut vec = Vec::new();

        for obj in self.body.iter() {
            vec.push(&obj.object);
        }

        vec
    }

    pub fn update_objects(&mut self, grid: &Grid, transition: Option<f32>){

        for obj in self.body.iter_mut() {
            obj.update_object(grid, transition);
        }
    }
    

    pub fn update(&mut self) -> bool {

        self.move_body();
        self.check_body_full();

        self.check_collision()
    }

    fn move_body(&mut self){

        let mut forward_direction = self.direction;


        for body_part in &mut self.body {

            let direction_buffer = body_part.direction;

            body_part.direction = forward_direction;
            forward_direction = direction_buffer;


            let position = &mut body_part.object.position;

            position.add_assign(body_part.direction.to_vector());

            
            if settings::ALLOW_WALL_TELEPORT == true {
                if position.x < 0 { position.x += settings::GRID_SIZE.x; }
                if position.y < 0 { position.y += settings::GRID_SIZE.y; }

                position.x = position.x % settings::GRID_SIZE.x;
                position.y = position.y % settings::GRID_SIZE.y;
            }
        }
    }

    fn check_body_full(&mut self){

        for body_part in self.body.iter_mut().skip(1) {

            if let SnakeDirection::None = body_part.direction {
                body_part.object.object.color[3] = SNAKE_BODY_FULL_ALPHA;
                body_part.object.object.mesh = SNAKE_BODY_FULL_MESH;
                body_part.object.size_muliplier = SNAKE_BODY_FULL_SIZE_MULTIPLIER;
            }
            else {
                body_part.object.object.color[3] = SNAKE_BODY_ALPHA;
                body_part.object.object.mesh = SNAKE_BODY_MESH;
                body_part.object.size_muliplier = SNAKE_BODY_SIZE_MULTIPLIER;
            }
        }
    }

    fn check_collision(&self) -> bool {

        let position = &self.body[0].object.position;

        if position.x >= settings::GRID_SIZE.x
        || position.x < 0
        || position.y >= settings::GRID_SIZE.y
        || position.y < 0 {
            return true;
        }


        for object in self.body.iter().skip(1) {

            if *position == object.object.position {
                return true;
            }
        }
            
        false
    }


    pub fn change_direction(&mut self, new_direction: SnakeDirection) -> bool {

        if self.body[0].direction.to_vector().mul(-1) != new_direction.to_vector(){
            self.direction = new_direction;
            return true;
        }

        false
    }

    pub fn check_food(&self, food: Iter<&GridDrawable>) -> (bool, usize) {

        let position = &self.body[0].object.position;
        
        for (i, f) in food.enumerate() {
            if *position == f.position { return (true, i); }
        }

        (false, 0)
    }
}

impl SnakeBody {
    pub fn update_object(&mut self, grid: &Grid, transition: Option<f32>) {
        self.object.update_object(grid, Some(self.direction.to_vector()), transition);
    }
}

impl SnakeDirection {
    pub fn to_vector(&self) -> cgmath::Vector2<i32>{

        let mut vector = cgmath::vec2(0, 0);
        
        match self {
            SnakeDirection::Up => vector.y -= 1,
            SnakeDirection::Down => vector.y += 1,
            SnakeDirection::Left => vector.x -= 1,
            SnakeDirection::Right => vector.x += 1,
            _ => (),
        };

        vector
    }
}