use proc_macro2::Ident;
use quote::format_ident;

pub struct GenericExportNamer {
    pub base: Ident,
}
impl GenericExportNamer {
    pub fn model_name(&self) -> Ident {
        format_ident!("{}Model", self.base)
    }
    pub fn entity_name(&self) -> Ident {
        format_ident!("{}Entity", self.base)
    }
    pub fn column_name(&self) -> Ident {
        format_ident!("{}Column", self.base)
    }
    pub fn active_name(&self) -> Ident {
        format_ident!("{}ActiveModel", self.base)
    }
    pub fn relation_name(&self) -> Ident {
        format_ident!("{}Relation", self.base)
    }
}
