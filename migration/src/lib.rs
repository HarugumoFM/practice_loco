#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20240721_091702_todos;
mod m20240721_092509_posts;
mod m20240721_092922_poms;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20240721_091702_todos::Migration),
            Box::new(m20240721_092509_posts::Migration),
            Box::new(m20240721_092922_poms::Migration),
        ]
    }
}