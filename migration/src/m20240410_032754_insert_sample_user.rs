use bcrypt::{hash, DEFAULT_COST};
use chrono::Local;
use entity::user;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveModelTrait, Set},
};

#[derive(DeriveMigrationName)]
pub struct Migration {
    user: user::ActiveModel,
}

impl Default for Migration {
    fn default() -> Self {
        Self {
            user: user::ActiveModel {
                id: Set(1),
                username: Set("Admin".to_owned()),
                first_name: Set(Some("Admin".to_string()).to_owned()),
                last_name: Set(Some("Admin".to_string()).to_owned()),
                password: Set(hash("44332211".to_string(), DEFAULT_COST).unwrap()),
                created_at: Set(Some(Local::now().naive_local().to_owned())),
                updated_at: Set(Some(Local::now().naive_local().to_owned())),
                ..Default::default()
            },
        }
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        self.user.to_owned().insert(db).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        self.user.to_owned().delete(db).await?;
        Ok(())
    }
}
