mod actor;
mod scene;
mod luabind;
mod shape;

use super::game::Game;
use rlua;
use rlua::{Function, Lua, Table};
use glium;
use glium::Surface;
use glium_sdl2;
use nalgebra as na;
use super::loader::Loader;

pub struct Mystig {
    frame: i32,
    lua: Lua,
    loader: Loader,
}

impl Mystig {
    pub fn new() -> Mystig {
        let mut loader = Loader::new();

        loader.load("basic.vert", "./assets/shaders/basic.vert");
        loader.load("basic.frag", "./assets/shaders/basic.frag");

        let binder = luabind::Binder::new();
        binder.bind();
        Mystig {
            frame: 0,
            lua: binder.lua,
            loader,
        }
    }

    fn update_lua(&mut self) -> Result<(), rlua::Error> {
        let globals = self.lua.globals();
        let update: Function = globals.get("update")?;
        update.call::<_, ()>(())
    }

    fn draw_lua(&self) -> Result<(), rlua::Error> {
        let globals = self.lua.globals();
        let draw: Function = globals.get("draw")?;
        draw.call::<_, ()>(())
    }
}

impl Game for Mystig {
    fn update(&mut self) -> () {
        match self.update_lua() {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        }
        self.frame += 1;
    }

    fn draw(&self, display: &mut glium_sdl2::Display) -> () {
        match self.draw_lua() {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        }

        let frag = self.loader.get("basic.frag").unwrap();
        let vert = self.loader.get("basic.vert").unwrap();
        let program = glium::Program::from_source(display, vert, frag, None).unwrap();

        use mystig::shape::HasXY;

        let model_view_projection = {
            let model = na::Isometry3::new(na::Vector3::x(), na::zero());
            let eye    = na::Point3::new(0.0f32, 1.0 * f32::sin(self.frame as f32 / 40.0),  1.0);
            let target = na::Point3::new(1.0f32 * f32::sin(self.frame as f32 / 70.0), 0.0, -1.0);
            let view   = na::Isometry3::look_at_rh(&eye, &target, &na::Vector3::y());
            let projection = na::Perspective3::new(
                16.0f32 / 9.0f32, 3.14f32 / 2.0f32, 0.0f32, 1000.0f32);
            let model_view = view * model;
            let mat_model_view = model_view.to_homogeneous();
            projection.as_matrix() * mat_model_view
        };
        let mvp_array: &[[f32; 4]; 4] = model_view_projection.as_ref();

        let mut target = display.draw();
        target.clear_color_and_depth((0.02, 0.02, 0.02, 1.0), 1.0);

        let xys = [
            shape::Vertex2D::from_xy(0.0f32 + 100.0, 100.0),
            shape::Vertex2D::from_xy(320.0f32, 240.0),
            shape::Vertex2D::from_xy(640.0f32 - 100.0, 480.0 - 100.0),
        ];
        for i in 1i32..4 {
            let s = shape::circle_line(xys[i as usize - 1],
                                       i as f32 * 100.0 * (1.0 + f32::sin(self.frame as f32 / 50.0)));
            let vertex_buffer = glium::VertexBuffer::new(display, &s).unwrap();
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
            let uniforms = uniform! {
                z: 0.0f32,
                mvp_matrix: *mvp_array,
                my_color: [1.0 / (i as f32), 0.0, 0.0, 0.5f32],
            };
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();

            let s = shape::circle_fill(xys[i as usize - 1],
                                       i as f32 * 100.0 * (1.0 + f32::sin(self.frame as f32 / 50.0)));
            let vertex_buffer = glium::VertexBuffer::new(display, &s).unwrap();
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
            let params = glium::DrawParameters {
                blend: glium::draw_parameters::Blend::alpha_blending(),
                .. Default::default()
            };
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniforms,
                    &params,
                )
                .unwrap();
        }

        {
            let s = shape::rect_line(shape::Vertex2D::from_xy(0.0f32, 240.0), 320.0f32, 240.0f32);
            let vertex_buffer = glium::VertexBuffer::new(display, &s).unwrap();
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
            let uniforms = uniform! {
                z: 0.0f32,
                mvp_matrix: *mvp_array,
                my_color: [0.0, 0.0, 1.0, 0.5f32],
            };

            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();

            let s = shape::rect_fill(shape::Vertex2D::from_xy(0.0f32, 240.0), 320.0f32, 240.0f32);
            let vertex_buffer = glium::VertexBuffer::new(display, &s).unwrap();
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
            let uniforms = uniform! {
                z: 0.0f32,
                mvp_matrix: *mvp_array,
                my_color: [0.0, 0.0, 1.0, 0.1f32],
            };
            let params = glium::DrawParameters {
                blend: glium::draw_parameters::Blend::alpha_blending(),
                .. Default::default()
            };

            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniforms,
                    &params,
                )
                .unwrap();
        }

        target.finish().unwrap();
    }

    fn finished(&self) -> bool {
        false
    }
}
