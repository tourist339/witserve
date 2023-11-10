wit_bindgen::generate!({
    world:"host",
    exports:{
        world:MyHost,
    },
});

struct MyHost;

impl Guest for MyHost{
    fn run(k:u32)->u32{
        print!("Hey");
        k
    }
}