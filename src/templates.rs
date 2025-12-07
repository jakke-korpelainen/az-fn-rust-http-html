use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use reqwest::StatusCode;

use crate::posts::BlogPost;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(askama::Template, Debug)]
#[template(path = "home.html")]
pub struct HomeTemplate {}

#[derive(askama::Template, Debug)]
#[template(path = "posts.html")]
pub struct BlogPostsTemplate {
    pub posts: Vec<BlogPost>,
}

#[derive(askama::Template, Debug)]
#[template(path = "post.html")]
pub struct BlogPostTemplate {
    pub post: Option<BlogPost>,
}

#[derive(askama::Template, Debug)]
#[template(path = "404.html")]
pub struct NotFoundTemplate {}
