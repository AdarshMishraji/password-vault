use axum::http::HeaderMap;

#[derive(Default, Clone, Debug)]
pub struct GraphQLContext {
    pub session_token: Option<String>,
    pub headers: Option<HeaderMap>,
}
