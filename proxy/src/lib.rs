mod bindings;

use bindings::exports::component::proxy::proxy::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn middleware(k:u32) -> u32 {
        k*3
    }
}
