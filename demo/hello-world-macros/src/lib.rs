
extern crate proc_macro;
use proc_macro::TokenStream;


#[proc_macro]
pub fn function_macro(input: TokenStream) -> TokenStream {
    // print input
    eprintln!("Function Macro called! Input:\n{}", input.to_string());
    // return input
    input
}


#[proc_macro_derive(DeriveMacro)]
pub fn derive_macro(input: TokenStream) -> TokenStream {
    // print input
    eprintln!("Derive Macro called! Input:\n{}", input.to_string());
    // return a token stream containing a trivial function
    "fn derive_macro_output(){}".parse().unwrap()
}


#[proc_macro_attribute]
pub fn attribute_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    // print input parameters
    eprintln!(
        "Attribute Macro called! \nAttr:\n{}\nItem:\n{}", 
        attr.to_string(), 
        item.to_string()
    );
    // return a token stream containing a trivial function
    "fn attr_macro_output(){}".parse().unwrap()
}
