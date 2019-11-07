extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;


#[proc_macro_attribute]
pub fn parse(attr: TokenStream, item: TokenStream) -> TokenStream {

    // parse item as a syn struct
    let strukt = syn::parse_macro_input!(item as syn::ItemStruct);
    let name = &strukt.ident;
    
    
    // extract name and type for each struct element
    let mut items = vec!();
    if let syn::Fields::Named(nf) = &strukt.fields {
        for field in &nf.named {
            items.push(
                (field.ident.as_ref().unwrap(), field.ty.to_token_stream().to_string())
            )
        }
    }

    // parse groups from input. A group is delimited by curly braces
    let pattern: String = syn::parse_macro_input!(attr as syn::LitStr).value();
    let parts = pattern.split("{}").collect::<Vec<_>>();

    // panic if number of brace pairs in pattern doesn't match number of struct elements
    assert!(parts.len() == items.len()+1);

    // generate the regex pattern string
    let mut regex_str = String::new();
    for (pattern_prefix, item) in parts.iter().zip(items.iter()) {
        let regex_for_type = get_regex_for_type(&item.1);
        regex_str.push_str(&format!("{}({})", pattern_prefix, regex_for_type));
    }
    regex_str.push_str(parts.last().unwrap());

    // generate initializers for each struct element
    let mut inits = vec!();
    for (idx, item) in items.iter().enumerate() {
        let name = &item.0;
        inits.push(quote!( #name: cap[#idx+1].parse().unwrap()));
    }

    
    quote!(
        #strukt

        impl #name {
            fn get_regex() -> &'static pat_parse::__imp::Regex {
                use pat_parse::__imp::Regex;
                pat_parse::__imp::lazy_static! {
                    static ref RE: Regex = Regex::new(#regex_str).unwrap();
                }
                &RE
            }
            
            pub fn from_str(s: &str) -> Option<Self> {
                Self::get_regex().captures(s).map(|cap| Self { #(#inits),* })
            }

            pub fn from_str_multiple(s: &str) -> Vec<Self> {
                Self::get_regex().captures_iter(s).map(|cap| Self { #(#inits),* }).collect()
            }
        }
    ).into()
}


fn get_regex_for_type(ty: &str) -> &'static str {
    match ty {
        "usize" => r"\d+",
        "f64" => r"[0-9]*\.?[0-9]*",
        "char" => r".",
        "bool" => r"true|false",
        t => panic!(format!("Regex for type '{}' is unknown", t))
    }
}