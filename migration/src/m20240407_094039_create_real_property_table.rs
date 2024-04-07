use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RealProperty::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RealProperty::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RealProperty::Title).string().null())
                    .col(ColumnDef::new(RealProperty::Description).text().null())
                    .col(ColumnDef::new(RealProperty::Address).text().null())
                    .col(ColumnDef::new(RealProperty::Images).text().null())
                    .col(ColumnDef::new(RealProperty::Lat).string().null())
                    .col(ColumnDef::new(RealProperty::Long).string().null())
                    .col(ColumnDef::new(RealProperty::Long).string().null())
                    .col(ColumnDef::new(RealProperty::OpenedAt).timestamp().null())
                    .col(ColumnDef::new(RealProperty::OrganizeId).integer().null())
                    .col(
                        ColumnDef::new(RealProperty::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP")
                            .null(),
                    )
                    .col(ColumnDef::new(RealProperty::UpdatedAt).timestamp().null())
                    .col(ColumnDef::new(RealProperty::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RealProperty::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RealProperty {
    Table,
    Id,
    Title,
    Description,
    Address,
    Images,
    Lat,
    Long,
    OpenedAt,
    OrganizeId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
