extern crate proc_macro;

use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, LitStr, Token};

mod lexer;
use lexer::{Lexer, ParseErr, Token};
mod parser;
use parser::Final;

struct Config {
    handler: Ident,
    result: Final,
}

impl Parse for Config {
    fn parse(input: ParseStream) -> Result<Self> {
        let flag = input.parse::<LitStr>()?;
        input.parse::<Token![,]>()?;
        let handler: Ident = input.parse()?;
        let result = flag
            .value()
            .parse()
            .map_err(|e: ParseErr| syn::Error::new(flag.span(), e.msg))?;
        Ok(Config { handler, result })
    }
}

#[proc_macro]
pub fn bonk(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Config { handler, result: Final {
        tasks,
        statics,
        max_buffer_size,
    }} = parse_macro_input!(input as Config);
    let statics = statics.map.into_iter().map(|(k, v)| {
        let ident = format_ident!("CLASS{}", v);
        quote! {
            static #ident: &[u8] = #k.as_bytes();
        }
    });
    let tasks = tasks.into_iter().enumerate().map(|(thread_id, task)| {
        let task = task.into_iter().map(|run| {
            let len = run.len();
            let inits = run.inits.into_iter().map(|init| {
                let value = init.value as u8;
                let idx = init.idx;
                quote! {
                    buf[#idx] = #value;
                }
            });
            let body = quote! {
                if <#handler as ::bonk::Bonk>::check(&mut bonker, &buf[0..#len]) {
                    return;
                }
            };
            let changes = run.changes.into_iter().rev().fold(body, |acc, change| {
                let class_ident = format_ident!("CLASS{}", change.class_id);
                let value_ident = format_ident!("c{}", change.idx);
                let idx = change.idx;
                let start = change.span.start;
                let end = change.span.end;
                quote! {
                    for #value_ident in #class_ident[#start..#end].iter().copied() {
                        buf[#idx] = #value_ident;
                        #acc
                    }
                }
            });
            quote! {
                #(#inits)*
                #changes
            }
        });
        quote! {
            // (|| {
            let mut buf = [0u8; MAX_SIZE];
            let mut bonker = <#handler as ::bonk::Bonk>::new(#thread_id);
            #(#task)*
            // })();
        }
    });
    let output = quote! {
        const MAX_SIZE: usize = #max_buffer_size;
        #(#statics)*
        #(#tasks)*
    };
    output.into()
}
