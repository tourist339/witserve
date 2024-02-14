mod mod1;

wit_bindgen::generate!({
    world:"host",
    exports:{
        world:MyHost,
    },
});

struct MyHost;



impl Guest for MyHost{
    fn fib(k:u32)->u32{
        preproxy(k*2*3)
    }
}

