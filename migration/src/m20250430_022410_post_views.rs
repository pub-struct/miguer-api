use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "post_views",
            &[
                ("id", ColType::PkAuto),
                ("ip_address", ColType::String),
                ("user_agent", ColType::TextNull),
                ("location", ColType::StringNull),
                ("referer", ColType::StringNull),
                ("device_type", ColType::StringNull),
            ],
            &[("posts", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "post_views").await
    }
}
