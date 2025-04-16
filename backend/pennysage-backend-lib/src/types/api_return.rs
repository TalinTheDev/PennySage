use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

/// Represents an API response with a required status code field and other optional fields
#[derive(Clone)]
pub struct ApiResponse {
    pub status: StatusCode,
    pub message: Option<String>,
    pub cookies: Option<Vec<String>>,
    pub json: Option<serde_json::Value>,
    pub error: Option<ApiError>,
}

impl ApiResponse {
    /// Creates and returns a new `ApiResponse` instance with the given status code
    /// and initializes all other fields to `None`.
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            message: None,
            cookies: None,
            json: None,
            error: None,
        }
    }

    /// Sets the message for the API response
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    /// Sets the cookies (as a [`Vec<String>`]) for the API response
    pub fn cookies(mut self, cookies: Vec<String>) -> Self {
        self.cookies = Some(cookies);
        self
    }

    /// Sets the JSON data for the API response
    pub fn json(mut self, json: serde_json::Value) -> Self {
        self.json = Some(json);
        self
    }

    /// Sets an error value (of type [`ApiError`] for the API response
    pub fn error(mut self, error: ApiError) -> Self {
        self.error = Some(error);
        self
    }
}

impl IntoResponse for ApiResponse {
    /// Converts a `ApiResponse` instance into an axum response
    fn into_response(mut self) -> Response {
        // res_builder needs to be re-assigned every time it is used because
        // all those functions (eg. header, body, status), consume the builder
        // and return it again
        let mut res_builder = Response::builder().header("Content-Type", "application/json");
        res_builder = res_builder.status(self.status);

        // If cookies are present in the response, loop through them and
        // add them to the headers
        if let Some(cookies) = self.cookies {
            for cookie in cookies {
                res_builder = res_builder.header("Set-Cookie", cookie);
            }
        }
        let body = json!({
            "message": self.message.get_or_insert("Nothing to say".to_string()),
            "data": self.json.get_or_insert(json!(null)),
            "error": self.error.get_or_insert(ApiError::new(json!(null).to_string())),
        });
        match res_builder.body(body.to_string().into()) {
            Ok(res) => res,
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(
                    json!({
                        "message": "Internal server error",
                        "json": json!(null),
                        "error": ApiError::new(e.to_string())
                    })
                    .to_string()
                    .into(),
                )
                .unwrap(),
        }
    }
}

/// Represents an error in the API response
#[derive(Clone, Serialize)]
pub struct ApiError {
    /// Holds the error message
    pub message: String,
}

impl ApiError {
    /// Creates and returns a new `ApiError` instance with the given message
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
