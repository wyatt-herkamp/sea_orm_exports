# Sea_ORM_Exports

Rename the sea_orm generated types

## This project contains two macros

First is a derive macro `SeaORMExports`

The `exports` attribute is required

```rust
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, SeaORMExports)]
#[sea_orm(table_name = "blog-comments")]
#[exports(BlogComment, has_relation)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// A random string that identifies the comment
    #[sea_orm(unique)]
    pub key: String,
    pub creator: i64,
    pub content: String,
    pub blog_post: i64,
    pub created: DateTimeWithTimeZone,
}
/// The generated code is
pub type BlogCommentModel = Model;
pub type BlogCommentEntity = Entity;
pub type BlogCommentColumn = Column;
pub type BlogCommentActiveModel = ActiveModel;
pub type BlogCommentRelation = Relation;
```

Next is proc_macro `export_module!`

```rust
export_module!(users, User, has_relation);

/// Generated Code
#[allow(unused_imports)]
pub use users::ActiveModel as UserActiveModel;
#[allow(unused_imports)]
pub use users::Column as UserColumn;
#[allow(unused_imports)]
pub use users::Entity as UserEntity;
#[allow(unused_imports)]
pub use users::Model as UserModel;
#[allow(unused_imports)]
pub use users::Relation as UserRelation;
```

I hate writing the same thing everytime so this is macro do the work for you.
