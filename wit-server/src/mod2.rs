use crate::exports::preproxy;

wit_bindgen::generate!({
    world:"second",
    exports:{
        world:MySecondHost,
    },
});

struct MySecondHost;

impl Guest for MySecondHost{
    fn count() -> u32 {
        96
    }
}

