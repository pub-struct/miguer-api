#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::posts::{self};
use axum::{debug_handler, extract::Query, http::StatusCode};
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
pub async fn update(_auth: auth::JWT, State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
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
        ..Default::default()
    };
    let saved_post = new_post.insert(&_ctx.db).await.map_err(|db_err| {
        eprintln!("Database error creating post: {:?}", db_err);
        Error::InternalServerError
    })?;
    Ok((StatusCode::CREATED, Json(saved_post)).into_response())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/posts/")
        .add("/", get(index))
        .add("/", post(create))
        .add("/{id}", get(retrieve))
        .add("/{id}", delete(remove))
        .add("/{id}", patch(update))
}
