
extern crate proc_macro;

use syn::Token;
use syn::parse::{Parse, ParseStream};
use quote::quote;
use quote::{TokenStreamExt, ToTokens};
use proc_macro2::TokenStream;
use syn::parenthesized;
use syn::punctuated::Punctuated;

#[derive(Debug)]
struct Grammar {
    items: Vec<GrammarDef>
}

#[derive(Debug)]
struct GrammarDef {
    name: syn::Ident,
    eq_token: Token![=],
    variants: Punctuated<GrammarVariant, Token![|]>,
}

#[derive(Debug)]
struct GrammarVariant {
    name: syn::Ident,
    args: Option<GrammarArgs>,
}

#[derive(Debug)]
struct GrammarArgs {
    paren_token: syn::token::Paren,
    args: Punctuated<GrammarArg, Token![,]>,
}

#[derive(Debug)]
struct GrammarArg {
    name: syn::Type,
    rep_type: GrammarRepType,
}

#[derive(Debug)]
enum GrammarRepType {
    None,
    Optional(Token![?]),
    Repetition(Token![*]),
}


impl Parse for Grammar {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = vec!();
        while !input.is_empty() {
            items.push(input.parse()?)
        }
        Ok(Grammar{
            items
        })
    }
}

impl Parse for GrammarDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(GrammarDef {
            name: input.parse()?,
            eq_token: input.parse()?,
            variants: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}

impl Parse for GrammarVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(GrammarVariant {
            name: input.parse()?,
            args: input.parse().ok(),
        })
    }
}

impl Parse for GrammarArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        let args = content.parse_terminated(GrammarArg::parse)?;
        Ok(GrammarArgs {
            paren_token,
            args
        })
    }
}

impl Parse for GrammarArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(GrammarArg {
            name: input.parse()?,
            rep_type: input.parse()?,
        })
    }
}

impl Parse for GrammarRepType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(tok) = input.parse() {
            Ok(GrammarRepType::Optional(tok))
        } else if let Ok(tok) = input.parse() {
            Ok(GrammarRepType::Repetition(tok))
        } else {
            Ok(GrammarRepType::None)
        }
    }
}

impl ToTokens for Grammar {
   fn to_tokens(&self, tokens: &mut TokenStream) {
       tokens.append_all(self.items.iter());
   }
}

impl ToTokens for GrammarDef {
   fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let variants = self.variants.iter();
        tokens.append_all(quote!( pub enum #name ));
        tokens.append_all(quote!( { #(#variants),* } ));
    }
}

impl ToTokens for GrammarVariant {
   fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        self.args.to_tokens(tokens);
    }
}

impl ToTokens for GrammarArgs {
   fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.iter();
        tokens.append_all(quote!( (#(#args),*) ))
    }
}

impl ToTokens for GrammarArg {
   fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let ty = match &self.rep_type {
            GrammarRepType::None => quote!(Box<#name>),
            GrammarRepType::Optional(_tok) => quote!(Option<Box<#name>>),
            GrammarRepType::Repetition(_tok) => quote!(Vec<#name>),
        };
        tokens.append_all(ty);
    }
}

#[proc_macro]
pub fn grammar(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syntax_tree = syn::parse_macro_input!(input as Grammar);
    
    // check that each definition contains at most three variants
    for d in &syntax_tree.items {
        if d.variants.len() > 3 {
            return syn::Error::new_spanned(&d, "more than three variants aren't supported").to_compile_error().into();
        }
    }
    
    quote!(#syntax_tree).into()
}