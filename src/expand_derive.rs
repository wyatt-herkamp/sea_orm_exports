use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::DeriveInput;
use syn::Result;

use crate::utils::GenericExportNamer;

pub struct Exports {
    pub has_relation: bool,
    pub base_name: Ident,
}
impl Parse for Exports {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let mut has_relation = false;
        let _ = input.parse::<syn::Token![,]>();
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(crate::keywords::has_relation) {
                input.parse::<crate::keywords::has_relation>()?;
                has_relation = true;
            } else {
                return Err(lookahead.error());
            }
        }
        Ok(Exports {
            base_name: name,
            has_relation,
        })
    }
}

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let args: Option<Result<Exports>> = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("exports"))
        .map(|v| v.parse_args());
    let args = match args {
        Some(Ok(ok)) => ok,
        Some(Err(err)) => return Err(err),
        None => return Err(syn::Error::new_spanned(input, "No exports attribute")),
    };
    let renamer = GenericExportNamer {
        base: args.base_name.clone(),
    };
    let model_name = renamer.model_name();
    let entity_name = renamer.entity_name();
    let column_name = renamer.column_name();
    let active_name = renamer.active_name();

    let relation = if args.has_relation {
        let relation_name = renamer.relation_name();
        quote! {
            pub type #relation_name = Relation;
        }
    } else {
        quote! {}
    };
    let result = quote! {
        pub type #model_name = Model;
        pub type #entity_name = Entity;
        pub type #column_name = Column;
        pub type #active_name = ActiveModel;
        #relation
    };
    Ok(result)
}
