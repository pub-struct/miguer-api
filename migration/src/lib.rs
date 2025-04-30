#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250423_224330_posts;
mod m20250430_022410_post_views;
mod m20250430_023018_add_tags_to_posts;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250423_224330_posts::Migration),
            Box::new(m20250430_022410_post_views::Migration),
            Box::new(m20250430_023018_add_tags_to_posts::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}