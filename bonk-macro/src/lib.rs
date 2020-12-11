#![feature(proc_macro_span)]
extern crate proc_macro;

mod lexer;
mod parser;

use lexer::{Lexer, ParseErr, Token};
use parser::{Change, Final, Init, Partition, Run};
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::{format_ident, quote};
use syn::parse::{self, Parse, ParseStream};
use syn::{parse_macro_input, Error, Ident, LitStr, Token};

struct Config {
    handler: Ident,
    result: Final,
    abort: bool,
    threaded: bool,
}

fn make_span(src: &str, idx: usize, old_span: Span) -> Span {
    let mut literal = Literal::string(src);
    literal.set_span(old_span);
    literal.subspan(idx + 1..idx + 2).unwrap_or(old_span)
}

impl Parse for Config {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let literal = input.parse::<LitStr>()?;
        let pattern = literal.value();
        input.parse::<Token![,]>()?;
        let handler = input.parse::<Ident>()?;
        let num_threads = num_cpus::get();
        let abort = false;
        let threaded = true;
        let result =
            Final::new(&pattern, num_threads, Partition::Weak).map_err(|e: ParseErr| {
                Error::new(make_span(&pattern, e.offset, literal.span()), e.msg)
            })?;

        Ok(Config {
            handler,
            result,
            abort,
            threaded,
        })
    }
}

#[proc_macro]
pub fn bonk(input: TokenStream) -> TokenStream {
    let Config {
        handler,
        abort,
        threaded,
        result: Final {
            tasks,
            statics,
            max_size,
        },
    } = parse_macro_input!(input as Config);

    let statics = statics.into_iter().map(|(k, v)| {
        let ident = format_ident!("CLASS_{}", v);
        quote! {
            static #ident: &[u8] = #k.as_bytes();
        }
    });
    let num_threads = tasks.len();
    let tasks = tasks.into_iter().enumerate().map(|(thread_id, task)| {
        let task = task.into_iter().map(
            |Run {
                 len,
                 inits,
                 changes,
             }| {
                let assignments = inits
                    .into_iter()
                    .map(|Init { buf_idx, val }| quote! { buf[#buf_idx] = #val; });
                let body = if abort {
                    quote! {
                        if <#handler as ::bonk::Bonk>::check(&mut bonker, &buf[0..#len]) {
                            ::std::process::exit(0);
                        }
                    }
                } else {
                    quote! {
                        if flag.load(::std::sync::atomic::Ordering::Relaxed)
                            || <#handler as ::bonk::Bonk>::check(&mut bonker, &buf[0..#len]) {
                            flag.store(true, ::std::sync::atomic::Ordering::Relaxed);
                            return;
                        }
                    }
                };
                let loops = changes.into_iter().rev().fold(
                    body,
                    |acc,
                     Change {
                         class_id,
                         buf_idx,
                         lower,
                         upper,
                     }| {
                        let class_ident = format_ident!("CLASS_{}", class_id);
                        let value_ident = format_ident!("c_{}", buf_idx);
                        quote! {
                            for #value_ident in #class_ident[#lower..#upper].iter().copied() {
                                buf[#buf_idx] = #value_ident;
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
        let output = quote! {
            let mut buf = [0u8; MAX_SIZE];
            let mut bonker = <#handler as ::bonk::Bonk>::new(#thread_id);
            #(#task)*
        };
        let output = if abort {
            output
        } else {
            let flag_ident = format_ident!("flag_{}", thread_id);
            quote! {
                let mut flag = #flag_ident;
                #output
            }
        };
        let output = if threaded {
            let thread_ident = format_ident!("t_{}", thread_id);
            quote! {
                let #thread_ident = ::std::thread::spawn(move || {
                    #output
                });
            }
        } else {
            output
        };
        let output = if abort {
            output
        } else {
            let flag_ident = format_ident!("flag_{}", thread_id);
            quote! {
                let #flag_ident = flag.clone();
                #output
            }
        };
        output
    });
    let output = quote! {
        const MAX_SIZE: usize = #max_size;
        #(#statics)*
        #(#tasks)*
    };
    let output = if abort {
        output
    } else {
        quote! {
            let flag = ::std::sync::Arc::new(::std::sync::atomic::AtomicBool::new(false));
            #output
        }
    };
    let output = if threaded {
        let joins = (0..num_threads).map(|thread_id| {
            let thread_ident = format_ident!("t_{}", thread_id);
            quote! { #thread_ident.join().unwrap(); }
        });
        quote! {
            #output
            #(#joins)*
        }
    } else {
        output
    };
    output.into()
}
