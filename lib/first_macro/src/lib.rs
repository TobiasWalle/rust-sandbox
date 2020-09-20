use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitStr};

mod keywords {
    syn::custom_keyword!(say);
    syn::custom_keyword!(loud);
}

struct Say {
    pub message: LitStr,
    pub loud: bool,
}

impl Parse for Say {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<keywords::say>()?;
        let lookahead = input.lookahead1();
        let loud = if lookahead.peek(keywords::loud) {
            input.parse::<keywords::loud>()?;
            true
        } else {
            false
        };
        let message: LitStr = input.parse()?;
        Ok(Say { loud, message })
    }
}

#[proc_macro]
pub fn communicate(item: TokenStream) -> TokenStream {
    let Say { loud, message } = parse_macro_input!(item as Say);
    if loud {
        (quote! {
            println!("{}", #message.to_uppercase());
        })
        .into()
    } else {
        (quote! {
            println!("{}", #message);
        })
        .into()
    }
}
