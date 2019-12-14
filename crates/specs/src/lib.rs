extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, Ident };
use syn::parse::{ Result, Parse, ParseStream };

fn impl_to_schema(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl Schema for #name {
            fn to_schema() {
                println!("Schema for {}", stringify!(#name));
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(Schema)]
pub fn to_schema_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_to_schema(&ast)
}

/*TODO:0

// WORK: Remove
mod keyword {
    syn::custom_keyword!(Feature);
    syn::custom_keyword!(Given);
    syn::custom_keyword!(When);
    syn::custom_keyword!(Then);
}

#[derive(Debug)]
struct BddSpecification {
    //Description: String,
}

impl Parse for BddSpecification {
    fn parse(input: ParseStream) -> Result<Self> {
        let _key1 = input.parse::<keyword::Feature>();
        let _id1 = input.parse::<Ident>();
        //let str1 = input.parse::<LitStr>();
        // input.step(|cursor| {
        //     let mut rest = *cursor;
        // });

        //input.parse::<Token![Feature]>();
        //let description = input.parse::<Token![Feature]>()?;

        // let lookahead = input.lookahead1();
        // if lookahead.peek(keyword::Feature) {
            
        // } else {
        //     //let error = lookahead.error();
        //     //compile_error!(concat!("Feature keyword required", 1));
        // }

        Ok(BddSpecification {
            
        })
    }
}

#[proc_macro]
pub fn spec(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as BddSpecification);

    //dbg!(input);

    TokenStream::new()
}
*/
