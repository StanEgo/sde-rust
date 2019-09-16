extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use syn::{ parse_macro_input, Token, Ident, Lit, LitStr };
use syn::parse::{ Result, Parse, ParseStream };

// WORK: Remove
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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
        let key1 = input.parse::<keyword::Feature>();
        let id1 = input.parse::<Ident>();
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
    let input = parse_macro_input!(input as BddSpecification);

    //dbg!(input);

    TokenStream::new()
}
