extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{self, Parse, ParseStream};
use syn::{self, parse_macro_input, Error, Ident, LitStr};

mod lexer;
use lexer::{Lexer, ParseErr, Token};
mod parser;
use parser::{Change, Final, Init, Run};

struct Config {
    handler: Ident,
    result: Final,
}

impl Parse for Config {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let flag = input.parse::<LitStr>()?;
        input.parse::<syn::Token![,]>()?;
        Ok(Config {
            handler: input.parse()?,
            result: flag
                .value()
                .parse()
                .map_err(|e: ParseErr| Error::new(flag.span(), e.msg))?,
        })
    }
}

#[proc_macro]
pub fn bonk(input: TokenStream) -> TokenStream {
    let Config {
        handler,
        result:
            Final {
                tasks,
                statics,
                max_buffer_size,
            },
    } = parse_macro_input!(input as Config);
    let statics = statics.into_iter().map(|(k, v)| {
        let ident = format_ident!("CLASS_{}", v);
        quote! {
            static #ident: &[u8] = #k.as_bytes();
        }
    });
    let tasks = tasks.into_iter().enumerate().map(|(thread_id, task)| {
        let task = task.into_iter().map(
            |Run {
                 len,
                 inits,
                 changes,
             }| {
                let assignments = inits.into_iter().map(|Init { idx, value }| {
                    let value = value as u8;
                    quote! { buf[#idx] = #value; }
                });
                let body = quote! {
                    if <#handler as ::bonk::Bonk>::check(&mut bonker, &buf[0..#len]) {
                        return;
                    }
                };
                let loops = changes.into_iter().rev().fold(
                    body,
                    |acc,
                     Change {
                         class_id,
                         idx,
                         start,
                         end,
                     }| {
                        let class_ident = format_ident!("CLASS_{}", class_id);
                        let value_ident = format_ident!("c_{}", idx);
                        quote! {
                            for #value_ident in #class_ident[#start..#end].iter().copied() {
                                buf[#idx] = #value_ident;
                                #acc
                            }
                        }
                    },
                );
                quote! {
                    #(#assignments)*
                    #loops
                }
            },
        );
        quote! {
            let mut buf = [0u8; MAX_SIZE];
            let mut bonker = <#handler as ::bonk::Bonk>::new(#thread_id);
            #(#task)*
        }
    });
    let output = quote! {
        const MAX_SIZE: usize = #max_buffer_size;
        #(#statics)*
        #(#tasks)*
    };
    output.into()
}
