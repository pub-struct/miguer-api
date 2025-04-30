#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::{
    _entities::post_views,
    _entities::posts::{self},
};
use axum::{
    debug_handler,
    extract::{Query, Request},
    http::{header, HeaderMap, StatusCode},
};
use chrono::{Duration, Utc};
use loco_rs::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    page: u64,
    page_size: u64,
}
#[derive(Debug, Deserialize, Validate, Clone)]
pub struct CreatePostData {
    #[validate(length(
        min = 5,
        message = "Voce precisa escrever um artigo com pelo menos 5 caracteres"
    ))]
    pub content: String,
    #[validate(length(
        min = 5,
        message = "Voce precisa escrever um titulo com pelo menos 5 caracteres"
    ))]
    pub title: String,
    #[validate(length(min = 1, message = "Voce precisa escrever pelo menos uma tag"))]
    tags: Vec<String>,
}
#[derive(Debug, Deserialize, Validate, Clone)]
pub struct UpdatePostData {
    #[validate(length(
        min = 5,
        message = "Voce precisa escrever um artigo com pelo menos 5 caracteres"
    ))]
    pub content: Option<String>,
    #[validate(length(
        min = 5,
        message = "Voce precisa escrever um titulo com pelo menos 5 caracteres"
    ))]
    pub title: Option<String>,
}
#[debug_handler]
pub async fn index(
    State(_ctx): State<AppContext>,
    Query(params): Query<QueryParams>,
) -> Result<Response> {
    let posts = posts::Model::all(&_ctx.db, params.page, params.page_size).await?;
    format::json(posts)
}

#[debug_handler]
pub async fn retrieve(State(_ctx): State<AppContext>, Path(id): Path<i32>) -> Result<Response> {
    let post = posts::Model::by_id(id).await.one(&_ctx.db).await?;
    let post = post.unwrap();
    format::json(post)
}

#[debug_handler]
pub async fn remove(
    _auth: auth::JWT,
    State(_ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let post = posts::Model::by_id(id).await.one(&_ctx.db).await?;
    let post = post.unwrap();
    post.delete(&_ctx.db).await?;
    format::text("Deleted")
}

#[debug_handler]
pub async fn update(
    _auth: auth::JWT,
    State(_ctx): State<AppContext>,
    Path(id): Path<i32>,
    JsonValidate(payload): JsonValidate<UpdatePostData>,
) -> Result<Response> {
    let post_model = posts::Model::by_id(id).await.one(&_ctx.db).await?;
    let mut post: posts::ActiveModel = post_model.unwrap().into_active_model();
    if let Some(content) = payload.content {
        post.content = Set(content);
    }
    if let Some(title) = payload.title {
        post.title = Set(title);
    }
    let _ = post.patch(&_ctx.db).await.map_err(|model_err| {
        tracing::error!(error.message = %model_err, "Failed to patch post model");
        Error::InternalServerError
    });
    format::text("Post Updated")
}

#[debug_handler]
pub async fn create(
    _auth: auth::JWT,
    State(_ctx): State<AppContext>,
    JsonValidate(post): JsonValidate<CreatePostData>,
) -> Result<Response> {
    let new_post = posts::ActiveModel {
        title: Set(post.title),
        content: Set(post.content),
        views: Set(0),
        tags: Set(post.tags),
        ..Default::default()
    };

    let saved_post = new_post.insert(&_ctx.db).await.map_err(|db_err| {
        eprintln!("Database error creating post: {:?}", db_err);
        println!("{:?}", db_err);
        Error::InternalServerError
    })?;
    Ok((StatusCode::CREATED, Json(saved_post)).into_response())
}
#[debug_handler]
pub async fn increase_views(
    State(_ctx): State<AppContext>,
    Path(id): Path<i32>,
    req: Request,
) -> Result<Response> {
    let post_model = posts::Model::by_id(id)
        .await
        .one(&_ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let cutoff = Utc::now() - Duration::hours(6);
    println!("{:?}", req.headers());
    // let ip = headers
    //     .get()
    //     .and_then(|value| value.to_str().ok())
    //     .and_then(|x_forwarded| x_forwarded.split(',').next())
    //     .unwrap_or_else(|| String::from("default-ip"));
    // let already_viewed = post_model
    //     .find_related(post_views::Entity)
    //     .filter(post_views::Column::IpAddress.eq(ip));

    format::text("Views Increased")
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/posts/")
        .add("/", get(index))
        .add("/", post(create))
        .add("/{id}", get(retrieve))
        .add("/{id}", delete(remove))
        .add("/{id}", patch(update))
        .add("/{id}/views", post(increase_views))
}
