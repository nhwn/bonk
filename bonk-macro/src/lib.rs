extern crate proc_macro;

use quote::{format_ident, quote};
use syn::{parse_macro_input, LitStr};

mod lexer;
use lexer::{Lexer, ParseErr, Token};
mod parser;
use parser::Final;

#[proc_macro]
pub fn bonk(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let Final {
        tasks,
        statics,
        max_buffer_size,
    } = input.value().parse().unwrap();
    let statics = statics.map.into_iter().map(|(k, v)| {
        let ident = format_ident!("CLASS{}", v);
        quote! {
            static #ident: &[u8] = #k.as_bytes();
        }
    });
    let tasks = tasks.into_iter().map(|task| {
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
                let s = unsafe { ::std::str::from_utf8_unchecked(&buf[0..#len]) };
                if s == "SKY-BRUH-1337" {
                    println!("{}", s);
                    return;
                }
            };
            let changes = run.changes.into_iter().rev().fold(body, |acc, change| {
                let idx = change.buffer_idx;
                let class_ident = format_ident!("CLASS{}", change.class_id);
                let value_ident = format_ident!("c{}", idx);
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
            (|| {
                let mut buf = [0u8; MAX_SIZE];
                #(#task)*
            })();
        }
    });
    let output = quote! {
        const MAX_SIZE: usize = #max_buffer_size;
        #(#statics)*
        #(#tasks)*
    };
    output.into()
}
