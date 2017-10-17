use rlua::Lua;

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
        self.do_file("")
    }

    fn rust_to_lua(&self) {}

    fn do_file(&self, path: &str) {
        let s = "Mys = { update = function () end, draw = function () end }";
        if let Err(e) = self.lua.eval::<()>(s, None) {
            println!("{:?}", e)
        }
    }
}
