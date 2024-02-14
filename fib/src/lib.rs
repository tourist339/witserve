mod bindings;

use bindings::exports::user::fib::run::Guest;

// Bring the imported add function into scope
use crate::bindings::component::proxy::proxy::middleware;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn run(k:u32) -> u32{
        middleware(k) * 2
    }
}
