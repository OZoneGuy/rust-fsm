mod types;

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

use crate::types::data::FSMMacroInput;

#[proc_macro]
pub fn fsm(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as FSMMacroInput);

    let states = input.states.iter().map(|s| {
        let state = format_ident!("{}", s);
        quote! {
            pub struct #state;
        }
    });

    let mut transition_map: HashMap<String, Vec<(String, String)>> = HashMap::new();
    input.transitions.iter().for_each(|t| {
        let (from, to, event) = (&t.from, &t.to, &t.event);
        transition_map
            .entry(from.clone())
            .or_insert_with(Vec::new)
            .push((event.clone(), to.clone()));
    });

    // create FSM<T> impls for the different states
    // each contain a function named after each event
    // that transitions to the next state
    // Each transition takes a callback function
    // let transition_impls = transition_map.iter().map(|(from, transitions)| {
    //     let from = format_ident!("{}", from);
    //     let transitions = transitions.iter().map(|(event, to)| {
    //         let event_ident = format_ident!("{}", event);
    //         let to = format_ident!("{}", to);
    //         let event_fn = format_ident!("{}_fn", event_ident);
    //         let end_fn = if input.final_.contains(event) {
    //             quote! {
    //                 pub fn end(self) {}
    //                 pub fn end_fn(self, f: impl Fn(&str, &str, &str)) {
    //                     f(stringify!(#from), stringify!(#to), stringify!(#event_ident));
    //                     self.end()
    //                 }
    //             }
    //         } else {
    //             quote! {}
    //         };
    //         quote! {
    //             pub fn #event_ident(self) -> FSM<#to> {
    //                 FSM {
    //                     _state: std::marker::PhantomData,
    //                 }
    //             }
    //             pub fn #event_fn(self, f: impl Fn(&str, &str, &str)) -> FSM<#to> {
    //                 f(stringify!(#from), stringify!(#to), stringify!(#event_ident));
    //                 self.#event_ident()
    //             }
    //             #end_fn
    //         }
    //     });

    //     quote! {
    //         impl FSM<#from> {
    //             #(#transitions)*
    //         }
    //     }
    // });

    let transition_impls = input.states.iter().map(|from_state| {
        let from_ident = format_ident!("{}", from_state);
        let mut funcs = vec![];
        if let Some(t) = transition_map.get(from_state) {
            funcs = t.iter().map(|(event, to)| {
            let event_ident = format_ident!("{}", event);
            let to_ident = format_ident!("{}", to);
            let event_fn = format_ident!("{}_fn", event_ident);

            quote! {
                pub fn #event_ident(self) -> FSM<#to_ident> {
                    let mut history = self.history;
                    history.push(Transition{
                        from: stringify!(#from_ident).to_string(),
                        to: stringify!(#to_ident).to_string(),
                        event: stringify!(#event_ident).to_string(),
                    });

                    return FSM {
                        _state: std::marker::PhantomData,
                        history,
                    };
                }
                pub fn #event_fn(self, f: impl Fn(&str, &str, &str)) -> FSM<#to_ident> {
                    f(stringify!(#from_ident), stringify!(#to_ident), stringify!(#event_ident));
                    return self.#event_ident();
                }
            }
            }).collect();
        };
        if input.final_.contains(from_state) {
            let end_fn = quote! {
                pub fn end(self) {}
                pub fn end_fn(self, f: impl Fn(&str)) {
                    f(stringify!(#from_ident));
                    self.end()
                }
            };
            funcs.push(end_fn);
        };
        quote! {
            impl FSM<#from_ident> {
                #(#funcs)*
            }
        }
    });

    let initial = format_ident!("{}", input.initial);
    let out = quote! {
        #(#states)*

        #[derive(PartialEq, Debug)]
        pub struct Transition {
            pub from: String,
            pub to: String,
            pub event: String,
        }
        pub struct FSM<T = #initial> {
            _state: std::marker::PhantomData<T>,
            pub history: Vec<Transition>,
        }

        impl FSM {
            pub fn new() -> FSM<#initial> {
                FSM {
                    _state: std::marker::PhantomData,
                    history: vec![],
                }
            }
        }

        #(#transition_impls)*
    };

    out.into()
}
