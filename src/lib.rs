mod expand_derive;
mod export_module;
pub(crate) mod utils;
use export_module::ExportModule;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};
pub(crate) mod keywords {
    syn::custom_keyword!(has_relation);
}
/// ```rust, ignore
/// #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, SeaORMExports)]
/// #[sea_orm(table_name = "blog-comments")]
/// #[exports(BlogComment, has_relation)]
/// pub struct Model {
///    #[sea_orm(primary_key, auto_increment = true)]
///    pub id: i64,
///    /// A random string that identifies the comment
///    #[sea_orm(unique)]
///    pub key: String,
///    pub creator: i64,
///    pub content: String,
///    pub blog_post: i64,
///    pub created: DateTimeWithTimeZone,
/// }
/// /// The generated code is
/// pub type BlogCommentModel = Model;
/// pub type BlogCommentEntity = Entity;
/// pub type BlogCommentColumn = Column;
/// pub type BlogCommentActiveModel = ActiveModel;
/// pub type BlogCommentRelation = Relation;
/// ```
#[proc_macro_derive(SeaORMExports, attributes(exports))]
pub fn sea_orm_exports(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let result = expand_derive::expand(input);
    match result {
        Ok(ok) => ok.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// ```rust, ignore
/// export_module!(crate::user, User, has_relation)
///
/// // expands to:
/// pub use crate::user::Model as UserModel;
/// pub use crate::user::Entity as UserEntity;
/// pub use crate::user::Column as UserColumn;
/// pub use crate::user::ActiveModel as UserActiveModel;
/// pub use crate::user::Relation as UserRelation;
/// ```
#[proc_macro]
pub fn export_module(input: TokenStream) -> TokenStream {
    let input: ExportModule = parse_macro_input!(input as export_module::ExportModule);
    input.expand().into()
}
