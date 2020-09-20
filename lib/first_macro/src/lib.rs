use proc_macro::TokenStream;
use quote::quote;
use std::iter::FromIterator;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, LitStr, Token};

mod keywords {
    syn::custom_keyword!(say);
    syn::custom_keyword!(loud);
}

struct Message {
    pub message: LitStr,
    pub loud: bool,
}

struct Conversation {
    pub messages: Vec<Message>,
}

impl Parse for Message {
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
        Ok(Self { loud, message })
    }
}

impl Parse for Conversation {
    fn parse(input: ParseStream) -> Result<Self> {
        let result = Punctuated::<Message, Token![.]>::parse_terminated(input)?;
        Ok(Conversation {
            messages: result.into_iter().collect(),
        })
    }
}

fn create_message(Message { message, loud }: Message) -> TokenStream {
    if loud {
        (quote! {
            println!("{}!!!", #message.to_uppercase());
        })
        .into()
    } else {
        (quote! {
            println!("{}", #message);
        })
        .into()
    }
}

#[proc_macro]
pub fn communicate(item: TokenStream) -> TokenStream {
    let Conversation { messages } = parse_macro_input!(item as Conversation);
    TokenStream::from_iter(messages.into_iter().map(|message| create_message(message)))
}
