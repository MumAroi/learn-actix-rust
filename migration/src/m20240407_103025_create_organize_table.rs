use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organize::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Organize::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Organize::Name).string().null())
                    .col(ColumnDef::new(Organize::Images).string().null())
                    .col(ColumnDef::new(Organize::CreatedBy).integer().null())
                    .col(ColumnDef::new(Organize::UpdatedBy).integer().null())
                    .col(ColumnDef::new(Organize::DeletedBy).integer().null())
                    .col(
                        ColumnDef::new(Organize::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP")
                            .null(),
                    )
                    .col(ColumnDef::new(Organize::UpdatedAt).timestamp().null())
                    .col(ColumnDef::new(Organize::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Organize::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Organize {
    Table,
    Id,
    Name,
    Images,
    CreatedBy,
    UpdatedBy,
    DeletedBy,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
