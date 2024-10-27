use axum::{debug_handler, extract::Path, response::IntoResponse};
use contentful::{ContentfulClient, QueryBuilder};
use std::{env, error::Error};

use crate::templates::{self, HtmlTemplate};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BlogPost {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

fn create_client() -> ContentfulClient {
    let contentful_space_id = match env::var("CONTENTFUL_SPACE_ID") {
        Ok(contentful_space_id) => contentful_space_id,
        Err(_) => panic!("CONTENTFUL_SPACE_ID is not set"),
    };
    
    let contentful_token = match env::var("CONTENTFUL_TOKEN") {
        Ok(contentful_token) => contentful_token,
        Err(_) => panic!("CONTENTFUL_TOKEN is not set"),
    };
    
    let contentful_client = ContentfulClient::new(&contentful_token, &contentful_space_id);
    contentful_client
}

pub async fn get_post(post: String) -> Result<BlogPost, Box<dyn Error>> {
    let client = create_client();

    // post in path may be a slug or an id
    let builder = QueryBuilder::new()
        .content_type_is("blogPost")
        .field_equals("fields.slug", &post);

    let result = match client.get_entries::<BlogPost>(Some(builder)).await {
        Ok(entries) => {
            if let Some(entry) = entries.first() {
                entry.clone()
            } else {
                return Err("No entries found".into());
            }
        }
        Err(error) => {
            return Err(error);
        }
    };
    Ok(result)
}

pub async fn get_posts() -> Result<Vec<BlogPost>, Box<dyn Error>> {
    let client = create_client();
    let builder = QueryBuilder::new().content_type_is("blogPost");
    let result = match client.get_entries::<BlogPost>(Some(builder)).await {
        Ok(posts) => {
            posts
        }
        Err(error) => {
            return Err(error);
        }
    };

    Ok(result)
}

#[debug_handler]
pub async fn posts_route() -> impl IntoResponse {
    let posts = match get_posts().await {
        Ok(posts) => posts,
        Err(_) => Vec::new(),
    };

    HtmlTemplate(templates::BlogPostsTemplate { posts })
}

#[debug_handler]
pub async fn post_route(Path(post): Path<String>) -> impl IntoResponse {
    let post = match get_post(post).await {
        Ok(post) => Some(post),
        Err(_) => None,
    };

    HtmlTemplate(templates::BlogPostTemplate { post })
}
