use crate::mod1::exports::bar;
// use crate::mod1::example::host::runit::run;

wit_bindgen::generate!({
    world:"host",
    exports:{
        world:MyHost,
        "bar":MyHost
    },
});

struct MyHost;


impl bar::Guest for MyHost{
    fn getheader() -> u32{
        23
    }
}

impl Guest for MyHost{
    fn execute(k:u32)->u32{
        run(k*10)
    }

    fn pre(k:u32)->u32{
        k+3
    }
}

