mod actor;
mod input;
mod luabind;
mod scene;
mod shape;

use super::game::Game;
use super::loader::Loader;
use super::input::Button;
use glium;
use glium::Surface;
use glium_sdl2;
use rlua;
use rlua::{Function, Lua, Table};
use sdl2;
use sdl2::event::Event;
use nalgebra::{self,Vector3,Point3,Isometry3,Orthographic3,Perspective3};

pub struct Mystig {
    frame: i32,
    lua_binder: luabind::Binder,
    loader: Loader,
    point: (i32, i32),
    button_input: input::Input<Button>,
}

impl Mystig {
    pub fn new() -> Mystig {
        let mut loader = Loader::new();

        loader.load("mat.vert", "./assets/shaders/mat.vert");
        loader.load("basic.frag", "./assets/shaders/basic.frag");

        let mut button_input = input::Input::new(4);
        button_input.register("Z", 0);
        button_input.register("MouseLeft", 0);
        button_input.register("X", 1);
        button_input.register("MouseRight", 1);
        button_input.register("C", 2);
        button_input.register("L", 3);

        let lua_binder = luabind::Binder::new();
        lua_binder.bind();
        Mystig {
            frame: 0,
            lua_binder,
            loader,
            point: (0, 0),
            button_input,
        }
    }

    fn create_params(&self) -> Table {
        let mouse_t = self.lua_binder.lua.create_table();
        mouse_t.set("x", self.point.0);
        mouse_t.set("y", self.point.1);
        let t = self.lua_binder.lua.create_table();
        t.set("mouse", mouse_t);
        t.set("frame", self.frame);
        return t;
    }

    fn update_lua(&mut self) -> Result<Table, rlua::Error> {
        let globals = self.lua_binder.lua.globals();
        let update: Function = globals.get("update")?;
        update.call::<Table, Table>(self.create_params())
    }

    fn draw_lua(&self) -> Result<Table, rlua::Error> {
        let globals = self.lua_binder.lua.globals();
        let draw: Function = globals.get("draw")?;
        draw.call::<Table, Table>(self.create_params())
    }

    fn draw_with_table(&self, display: &mut glium_sdl2::Display, table: Table) -> () {
        let frag = self.loader.get("basic.frag").unwrap();
        let vert = self.loader.get("mat.vert").unwrap();
        let program = glium::Program::from_source(
            display, vert, frag, None).unwrap();

        use mystig::shape::HasXY;

        let model: Isometry3<f32> = Isometry3::new(Vector3::x(), nalgebra::zero());
        let eye = {
            if let Ok(eye_t) = table.get::<_, Table>("eye") {
                Point3::new(
                    eye_t.get::<_, f32>(1).unwrap_or(0.0),
                    eye_t.get::<_, f32>(2).unwrap_or(0.0),
                    eye_t.get::<_, f32>(3).unwrap_or(0.0))
            } else {
                Point3::new(0.0f32, 0.0, 1.0)
            }
        };
        let target = {
            if let Ok(target_t) = table.get::<_, Table>("target") {
                Point3::new(
                    target_t.get::<_, f32>(1).unwrap_or(0.0),
                    target_t.get::<_, f32>(2).unwrap_or(0.0),
                    target_t.get::<_, f32>(3).unwrap_or(0.0))
            } else {
                Point3::new(((self.point.0 as f32) - 320.0) / 640.0,
                            ((self.point.1 as f32) - 240.0) / 480.0,
                            0.0)
            }
        };
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
        let proj = {
            if let Ok(perspective_t) = table.get::<_, Table>("perspective") {
                Perspective3::new(
                    perspective_t.get::<_, f32>("aspect").unwrap_or(1.0),
                    perspective_t.get::<_, f32>("fovy").unwrap_or(1.0),
                    perspective_t.get::<_, f32>("znear").unwrap_or(1.0),
                    perspective_t.get::<_, f32>("zfar").unwrap_or(1.0))
            } else {
                Perspective3::new(640.0 / 480.0, 3.14 / 2.0, 1.0, 1000.0)
            }
        };
        let model_view = view * model;
        let mat_model_view = model_view.to_homogeneous();
        let mvp = proj.as_matrix() * mat_model_view;
        println!("{}", mvp);
        let mvp_fixed: [[f32; 4]; 4] = mvp.into();

        let mut target = display.draw();
        target.clear_color_and_depth((0.02, 0.02, 0.02, 1.0), 1.0);

        let vertex_buffer = {
            #[derive(Copy, Clone)]
            struct Vertex {
                position: [f32; 3],
            }

            implement_vertex!(Vertex, position);
            let scale = self.point.0 as f32;
            let mut vertices = Vec::new();
            for i in [-1.0f32, 1.0].iter() {
                for j in [-1.0f32, 1.0].iter() {
                    for k in [-1.0f32, 1.0].iter() {
                        vertices.push(Vertex { position: [*i, *j, *k] })
                    }
                }
            }
            glium::VertexBuffer::new(display,&vertices).unwrap()
        };

        {
            let uniforms = uniform!{
                mvp: mvp_fixed,
                my_color: [1.0f32, 0.0, 0.0, 1.0],
            };
            // building the index buffer
            let index_buffer = glium::IndexBuffer::new(
                display, glium::index::PrimitiveType::LinesList,
                &[0u16, 1, 1, 3, 3, 2, 2, 0, 0, 4, 1, 5, 2, 6, 3, 7, 4, 5, 5, 7, 7, 6, 6, 4]).unwrap();
            let params = glium::DrawParameters {
                blend: glium::draw_parameters::Blend::alpha_blending(),
                ..Default::default()
            };
            target
                .draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params)
                .unwrap();
        }

        target.finish().unwrap();
    }
}

impl Game for Mystig {
    fn process_event(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(keycode), ..
            } => {
                if let Some(button) = self.button_input.get_mut(keycode.name().as_str()) {
                    button.down()
                }
            },
            Event::KeyUp {
                keycode: Some(keycode), ..
            } => {
                if let Some(button) = self.button_input.get_mut(keycode.name().as_str()) {
                    button.up()
                }
            },
            Event::MouseButtonUp { x, y, .. } => {
                if let Some(button) = self.button_input.get_mut("MouseLeft") {
                    button.up()
                }
            },
            Event::MouseButtonDown { x, y, .. } => {
                if let Some(button) = self.button_input.get_mut("MouseLeft") {
                    button.down()
                }
            },
            Event::MouseMotion { x, y, .. } => {
                self.point = (x, y)
            },
            _ => {}
        }
    }

    fn update(&mut self) -> () {
        self.button_input.update();
        if let Some(button) = self.button_input.get("L") {
            if button.pressed() && button.just() {
                self.lua_binder.bind();
                println!("script reloaded.");
            }
        }
        match self.update_lua() {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        }
        self.frame += 1;
    }

    fn draw(&self, display: &mut glium_sdl2::Display) -> () {
        match self.draw_lua() {
            Ok(t) => self.draw_with_table(display, t),
            Err(e) => println!("{:?}", e),
        }
    }

    fn finished(&self) -> bool {
        false
    }
}
