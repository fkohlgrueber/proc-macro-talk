
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn function_macro(input: TokenStream) -> TokenStream {
    eprintln!("Function Macro called! Input:\n{}", input.to_string());
    input
}

#[proc_macro_derive(DeriveMacro)]
pub fn derive_macro(input: TokenStream) -> TokenStream {
    eprintln!("Derive Macro called! Input:\n{}", input.to_string());
    "fn test_2(){}".parse().unwrap()
}

#[proc_macro_attribute]
pub fn attribute_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("Attribute Macro called! \nAttr:\n{}\nItem:\n{}", attr.to_string(), item.to_string());
    "fn test_3(){}".parse().unwrap()
}
