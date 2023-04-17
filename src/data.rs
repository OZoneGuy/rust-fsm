use syn::parse::{Parse, ParseStream};

mod kw {
    syn::custom_keyword!(initial);
    syn::custom_keyword!(end);
}

pub struct FSMMacroInput {
    pub initial: String,
    pub final_: Vec<String>,
    pub states: Vec<String>,
    pub transitions: Vec<Transition>,
}

#[derive(Debug)]
pub struct Transition {
    pub from: String,
    pub to: String,
    pub event: String,
}

impl Parse for FSMMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut initial: Option<String> = None;
        let mut final_: Vec<String> = Vec::new();
        let mut states: Vec<String> = Vec::new();
        let mut transitions: Vec<Transition> = Vec::new();

        let mut init_span = None;

        while !input.is_empty() {
            // parse initial state
            if input.peek(kw::initial) {
                if initial.is_some() {
                    return Err(input.error("initial state already defined"));
                }
                let kw = input.parse::<syn::Ident>()?;
                let eq = input.parse::<syn::Token![=]>()?;
                let ident = input.parse::<syn::Ident>()?;
                if cfg!(feature = "nightly") {
                    init_span = kw.span().join(eq.span).unwrap().join(ident.span());
                } else {
                    init_span = Some(kw.span());
                }
                initial = Some(ident.to_string());
            }

            // parse final states
            if input.peek(kw::end) {
                if final_.len() > 0 {
                    return Err(input.error("final state already defined"));
                }
                let _ = input.parse::<syn::Ident>()?;
                let _ = input.parse::<syn::Token![=]>()?;
                let ident = input.parse::<syn::Ident>()?;
                final_.push(ident.to_string());
                while input.peek(syn::Token![,]) {
                    let _ = input.parse::<syn::Token![,]>()?;
                    let ident = input.parse::<syn::Ident>()?;
                    final_.push(ident.to_string());
                }
            }

            // parse state or transition
            if input.peek(syn::Ident) {
                // parse the state name
                let ident = input.parse::<syn::Ident>()?;

                // parse transition
                if input.peek(syn::Token![->]) {
                    let _ = input.parse::<syn::Token![->]>()?;
                    let to = input.parse::<syn::Ident>()?;
                    let _ = input.parse::<syn::Token![:]>()?;
                    let event = input.parse::<syn::Ident>()?;
                    transitions.push(Transition {
                        from: ident.to_string(),
                        to: to.to_string(),
                        event: event.to_string(),
                    });
                } else {
                    // push state to list
                    states.push(ident.to_string());
                }
            };
        }

        if initial.is_none() {
            return Err(input.error("initial state not defined"));
        }

        if final_.len() == 0 {
            return Err(input.error("final state not defined"));
        }

        if states.len() == 0 {
            return Err(input.error("no states defined"));
        }

        if transitions.len() == 0 {
            return Err(input.error("no transitions defined"));
        }

        let init_state = initial.clone().unwrap();
        if !states.contains(&init_state) {
            return Err(syn::Error::new(
                init_span.unwrap(),
                "initial state not defined",
            ));
        }

        Ok(FSMMacroInput {
            initial: initial.unwrap(),
            final_,
            states,
            transitions,
        })
    }
}
