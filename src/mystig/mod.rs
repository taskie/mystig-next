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
use super::loader::Loader;

pub struct Mystig {
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
    }

    fn draw(&self, display: &mut glium_sdl2::Display) -> () {
        match self.draw_lua() {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        }

        let frag = self.loader.get("basic.frag").unwrap();
        let vert = self.loader.get("basic.vert").unwrap();
        let program = glium::Program::from_source(display, vert, frag, None).unwrap();

        let vertex1 = shape::ColoredVertex2D {
            position: [0.0, 0.5],
            vert_color: [1.0, 0.0, 0.0, 1.0],
        };
        let vertex2 = shape::ColoredVertex2D {
            position: [-0.5, -0.5],
            vert_color: [0.0, 1.0, 0.0, 1.0],
        };
        let vertex3 = shape::ColoredVertex2D {
            position: [0.5, -0.5],
            vert_color: [0.0, 0.0, 1.0, 1.0],
        };
        let shape = vec![vertex1, vertex2, vertex3];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let mut target = display.draw();
        target.clear_color_and_depth((0.5, 0.5, 1.0, 0.0), 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }

    fn finished(&self) -> bool {
        false
    }
}
