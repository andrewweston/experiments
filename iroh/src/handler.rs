use std::sync::Arc;

use axum::extract::State;
// use anyhow::Result;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::node::Node;

#[derive(Deserialize)]
pub struct JoinRequest {
    ticket: String,
}

#[derive(Deserialize, Serialize)]
pub struct JoinResponse {
    ticket: String,
}

#[axum::debug_handler]
pub async fn join_doc(
    State(node): State<Arc<Node>>, Json(req): Json<JoinRequest>,
) -> Result<Json<JoinResponse>, error::Error> {
    node.join_doc(&req.ticket).await?;
    Ok(Json(JoinResponse { ticket: req.ticket }))
}

mod error {
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Json, Response};
    use serde_json::json;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("{0:#}")]
        Other(anyhow::Error),
    }

    impl From<anyhow::Error> for Error {
        fn from(err: anyhow::Error) -> Self {
            Error::Other(err)
        }
    }

    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            let (status, error_message) = match self {
                Error::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()),
            };
            let body = Json(json!({
                "error": error_message,
            }));
            (status, body).into_response()
        }
    }
}
