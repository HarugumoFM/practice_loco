use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Poms::Table)
                    .col(pk_auto(Poms::Id))
                    .col(string(Poms::Title))
                    .col(string_null(Poms::Author))
                    .col(text_null(Poms::Content))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Poms::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Poms {
    Table,
    Id,
    Title,
    Author,
    Content,
    
}


