use sea_orm::entity::prelude::*;
use sea_orm_exports::SeaORMExports;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SeaORMExports)]
#[exports(User, has_relation)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    pub username: String,
    pub email: String,
    pub email_verified_at: Option<DateTimeWithTimeZone>,
    pub password: String,
    pub require_password_change: bool,
    pub password_changed_at: DateTimeWithTimeZone,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub banned: bool,
    pub created: DateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
