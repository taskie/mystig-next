mod actor;
mod scene;
mod luabind;

use super::game::Game;
use rlua::{Error, Function, Lua, Table};

pub struct Mystig {
    lua: Lua,
}

impl Mystig {
    pub fn new() -> Mystig {
        let binder = luabind::Binder::new();
        binder.bind();
        Mystig { lua: binder.lua }
    }

    fn update_lua(&mut self) -> Result<(), Error> {
        let globals = self.lua.globals();
        let mys: Table = globals.get("Mys")?;
        let update: Function = mys.get("update")?;
        update.call::<_, ()>(())
    }

    fn draw_lua(&self) -> Result<(), Error> {
        let globals = self.lua.globals();
        let mys: Table = globals.get("Mys")?;
        let draw: Function = mys.get("draw")?;
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

    fn draw(&self) -> () {
        match self.draw_lua() {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        }
    }

    fn finished(&self) -> bool {
        false
    }
}
