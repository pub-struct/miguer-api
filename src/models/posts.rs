pub use super::_entities::posts::{ActiveModel, Entity, Model};
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
pub type Posts = Entity;

#[derive(Serialize, Deserialize)]
pub struct Pagination<T> {
    pub count: u64,
    pub current_page: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
    pub total_pages: u64,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl Model {
    pub async fn all(
        db: &DatabaseConnection,
        page: u64,

        page_size: u64,
    ) -> ModelResult<Pagination<Self>> {
        if page == 0 {
            return Err(ModelError::Any("Page number must be greater than 0".into()));
        }
        if page_size == 0 {
            return Err(ModelError::Any("Page size must be greater than 0".into()));
        }
        println!("Fetching posts page {} with size {}", page, page_size); // Log input

        let query: Select<Posts> = Posts::find();
        println!("Base Query: {:?}", query);

        let paginator = query.paginate(db, page_size);

        let total_items = match paginator.num_items().await {
            Ok(count) => {
                println!("Total items found: {}", count); // Log total items
                count
            }
            Err(e) => {
                eprintln!("Error fetching total items: {:?}", e); // Log error
                return Err(ModelError::from(e));
            }
        };

        let total_pages = match paginator.num_pages().await {
            Ok(pages) => {
                println!("Total pages found: {}", pages); // Log total pages
                pages
            }
            Err(e) => {
                eprintln!("Error fetching total pages: {:?}", e); // Log error
                return Err(ModelError::from(e));
            }
        };

        // If total_items is 0, we expect an empty result set
        if total_items == 0 {
            println!("No items found in the database according to paginator.");
        }

        let results = match paginator
            .fetch_page(page - 1) // Adjust to 0-based index
            .await
        {
            Ok(res) => {
                println!("Fetched {} items for page {}", res.len(), page); // Log fetched count
                res
            }
            Err(e) => {
                eprintln!("Error fetching page data: {:?}", e); // Log error
                return Err(ModelError::from(e));
            }
        };

        let current_page_results = results;
        let pagination_data = Pagination {
            count: total_items,
            current_page: page,
            total_pages,
            next: None, // Consider calculating next/previous URLs if needed
            previous: None,
            results: current_page_results,
        };
        Ok(pagination_data)
    }
    pub async fn by_id(id: i32) -> Select<Posts> {
        Posts::find_by_id(id)
    }
}

impl ActiveModel {
    pub async fn remove(self, db: &DatabaseConnection) -> ModelResult<()> {
        self.delete(db).await?;
        Ok(())
    }
    pub async fn patch(self, db: &DatabaseConnection) -> ModelResult<()> {
        self.update(db).await?;
        Ok(())
    }
}

impl Entity {}
