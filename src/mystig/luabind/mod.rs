use rlua::{self, Lua};

use loader::Loader;

pub struct Binder {
    pub lua: Lua,
}

impl Binder {
    pub fn new() -> Binder {
        Binder { lua: Lua::new() }
    }

    pub fn bind(&self) {
        self.lua_to_rust();
        self.rust_to_lua();
    }

    fn lua_to_rust(&self) {
        self.do_file("./assets/scripts/main.lua")
    }

    fn rust_to_lua(&self) -> rlua::Result<()> {
        let g = self.lua.globals();
        g.set(
            "fma",
            self.lua.create_function(|_, (a, b, c): (f64, f64, f64)| Ok(a * b + c)),
        )?;
        Ok(())
    }

    fn do_file(&self, path: &str) {
        let result = Loader::read_file(path);
        if let Ok(s) = result {
            if let Err(e) = self.lua.eval::<()>(s.as_str(), None) {
                println!("{:?}", e)
            }
        }
    }
}
