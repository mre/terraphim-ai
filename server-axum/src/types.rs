use terraphim_pipeline::{Document};

use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};

/// Query type for searching documents in the `RoleGraph`.
/// It contains the search term, skip and limit parameters.
#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub(crate) struct SearchQuery {
    pub(crate) search_term: String,
    pub(crate) skip: Option<usize>,
    pub(crate) limit: Option<usize>,
    pub (crate) role: Option<String>,
}

/// Create article schema
#[derive(Deserialize, Serialize, Debug, ToSchema, Clone)]
pub(crate) struct Article {
    pub(crate) id: Option<String>,
    pub(crate) stub: Option<String>,
    pub(crate) title: String,
    pub(crate) url: String,
    pub(crate) body: String,
    pub(crate) description: Option<String>,
    pub(crate) tags: Option<Vec<String>>,
}

impl From<Article> for Document {
    fn from(val: Article) -> Self {
        // If the ID is not provided, generate a new one
        let id = match val.id {
            Some(id) => id,
            None => ulid::Ulid::new().to_string(),
        };

        Document {
            id,
            title: val.title,
            body: Some(val.body),
            description: val.description,
        }
    }
}