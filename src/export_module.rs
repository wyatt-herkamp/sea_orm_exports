use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;

use crate::{keywords, utils::GenericExportNamer};
#[derive(Clone)]
pub struct ExportModule {
    pub module: syn::Path,
    pub base_name: syn::Ident,
    pub has_relation: bool,
}
impl ExportModule {
    pub fn expand(self) -> TokenStream {
        let Self {
            module,
            base_name,
            has_relation,
        } = self;
        let renamer = GenericExportNamer { base: base_name };
        let relation = if has_relation {
            let rename_as = renamer.relation_name();
            quote! {
                #[allow(unused_imports)]
                pub use #module::Relation as #rename_as;
            }
        } else {
            quote! {}
        };
        let model_name = renamer.model_name();
        let entity_name = renamer.entity_name();
        let column_name = renamer.column_name();
        let active_name = renamer.active_name();
        quote! {
            #[allow(unused_imports)]
            pub use #module::Model as #model_name;
            #[allow(unused_imports)]
            pub use #module::Entity as #entity_name;
            #[allow(unused_imports)]
            pub use #module::Column as #column_name;
            #[allow(unused_imports)]
            pub use #module::ActiveModel as #active_name;
            #relation
        }
    }
}

impl Parse for ExportModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let module: syn::Path = input.parse::<syn::Path>()?;
        let mut has_relation = false;
        let _ = input.parse::<syn::Token![,]>()?;
        let base_name = input.parse::<syn::Ident>()?;
        let _ = input.parse::<syn::Token![,]>();

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keywords::has_relation) {
                input.parse::<keywords::has_relation>()?;
                has_relation = true;
            } else {
                return Err(lookahead.error());
            }
        }
        Ok(ExportModule {
            module,
            has_relation,
            base_name,
        })
    }
}
