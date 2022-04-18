use ggez::{Context};
use ggez::graphics::{self, DrawMode, FillOptions};

use super::Object;
use super::Mesh;


pub fn draw(ctx: &mut Context, objects: Vec<&Object>) {
    
    for object in objects {

        let mesh = match object.mesh {
            Mesh::Rectangle => graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::Fill(FillOptions::DEFAULT),
                graphics::Rect::new(0.0, 0.0, object.size.x, object.size.y),
                graphics::Color::new(object.color[0], object.color[1], object.color[2], object.color[3]))
                .unwrap(),

            Mesh::Circle => graphics::Mesh::new_circle(
                ctx,
                DrawMode::Fill(FillOptions::DEFAULT),
                cgmath::point2(object.size.x / 2.0, object.size.y / 2.0),
                object.size.x / 2.0,
                0.2,
                graphics::Color::new(object.color[0], object.color[1], object.color[2], object.color[3]))
                .unwrap(),

            Mesh::RoundedRectangle => graphics::Mesh::new_rounded_rectangle(
                ctx,
                DrawMode::Fill(FillOptions::DEFAULT),
                graphics::Rect::new(0.0, 0.0, object.size.x, object.size.y),
                10.0,
                graphics::Color::new(object.color[0], object.color[1], object.color[2], object.color[3]))
                .unwrap(),
        };
        
    
        let params = graphics::DrawParam::new();
    
        let params2 = params
            .dest(object.position)
            .rotation(0.0)
            .scale(cgmath::Vector2 {x:1.0, y:1.0});
    
    
        graphics::draw(ctx, &mesh, params2).unwrap();
    }
}