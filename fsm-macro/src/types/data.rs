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
        let initial: String;
        let mut final_: Vec<String> = Vec::new();
        let mut states: Vec<String> = Vec::new();
        let mut transitions: Vec<Transition> = Vec::new();

        if input.peek(kw::initial) {
            let _ = input.parse::<syn::Ident>()?;
            let _ = input.parse::<syn::Token![=]>()?;
            let ident = input.parse::<syn::Ident>()?;
            initial = ident.to_string();
        } else {
            return Err(input.error("expected `initial = initial_state_name`"));
        }

        // parse states
        while input.peek(syn::Ident) {
            if input.peek(kw::end) {
                break;
            }
            let ident = input.parse::<syn::Ident>()?;
            states.push(ident.to_string());
        }

        // parse final states
        if input.peek(kw::end) {
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

        // parse transitions
        while input.peek(syn::Ident) {
            let from = input.parse::<syn::Ident>()?;
            let _ = input.parse::<syn::Token![->]>()?;
            let to = input.parse::<syn::Ident>()?;
            let _ = input.parse::<syn::Token![:]>()?;
            let event = input.parse::<syn::Ident>()?;
            transitions.push(Transition {
                from: from.to_string(),
                to: to.to_string(),
                event: event.to_string(),
            });
        }

        Ok(FSMMacroInput {
            initial,
            final_,
            states,
            transitions,
        })
    }
}
