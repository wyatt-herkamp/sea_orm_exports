use sea_orm_exports::export_module;

pub mod user;
export_module!(user, User, has_relation);
