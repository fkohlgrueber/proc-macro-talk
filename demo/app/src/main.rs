#![allow(dead_code)]

use hello_world_macros::{
    function_macro, 
    DeriveMacro,
    attribute_macro,
};


function_macro!{
    fn test(){}
}


#[derive(DeriveMacro)]
struct Foo {
    bar: usize
}


#[attribute_macro(attribute argument tokens)]
fn attribute_item() {}


fn main() {
}
