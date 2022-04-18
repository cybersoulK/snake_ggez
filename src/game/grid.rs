use cgmath::{ElementWise};

use crate::engine;

pub struct Grid {

    grid_to_screen: cgmath::Point2<f32>,
    offset: cgmath::Point2<f32>,

    pub object: engine::Object,
}

const GRID_OBJECT_TEMPLATE: engine::Object = engine::Object {
    position: cgmath::Point2 { x: 0.0, y: 0.0 },
    size: cgmath::Point2 { x: 0.0, y: 0.0 },
    angle: 0.0,
    color: [0.95, 0.95, 0.95, 0.9],
    mesh: engine::Mesh::Rectangle,
};

impl Grid {
    pub fn new(screen_size: cgmath::Point2<i32>, grid_size: cgmath::Point2<i32>) -> Grid {
   
        let grid_to_screen;
        let offset;

        let mut object = GRID_OBJECT_TEMPLATE;
        {
            let screen_size = screen_size.map(|c| c as f32);
            let grid_size = grid_size.map(|c| c as f32);
            
            let screen_ratio_test = screen_size.div_element_wise(grid_size);
            

            let grid_to_screen_ratio = if screen_ratio_test.x < screen_ratio_test.y { screen_ratio_test.x } else { screen_ratio_test.y };
            grid_to_screen = cgmath::point2(grid_to_screen_ratio, grid_to_screen_ratio);

            offset = screen_size.sub_element_wise(grid_size.mul_element_wise(grid_to_screen)) / 2.0;


            object.position = offset;
            object.size = grid_size.mul_element_wise(grid_to_screen);
        }

        Grid {
            grid_to_screen,
            offset,

            object,
        }
    }

    pub fn to_screen(&self, grid_position: cgmath::Point2<i32>, object_size: f32) -> (cgmath::Point2<f32>, cgmath::Point2<f32>){

        let grid_position = grid_position.map(|c| c as f32);
        
        let screen_position = 
            (grid_position.mul_element_wise(self.grid_to_screen))
            .add_element_wise(self.offset)
            .add_element_wise(self.grid_to_screen * (1.0 - object_size) / 2.0);
        
        let size = self.grid_to_screen * object_size;

        (screen_position, size)
    }
}