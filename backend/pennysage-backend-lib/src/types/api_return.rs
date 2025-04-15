use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

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
    fn into_response(self) -> Response {
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

        // If a field is present, add it to the body of the response
        // If that fails, return an internal server error

        if let Some(message) = self.message {
            match res_builder.body(message.into()) {
                Ok(body) => return body,
                Err(_) => {
                    let mut response = Response::new("Internal Server Error".into());
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        }

        if let Some(json) = self.json {
            match res_builder.body(json.to_string().into()) {
                Ok(body) => return body,
                Err(_) => {
                    let mut response = Response::new("Internal Server Error".into());
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        }

        if let Some(error) = self.error {
            match res_builder.body(error.message.into()) {
                Ok(body) => return body,
                Err(_) => {
                    let mut response = Response::new("Internal Server Error".into());
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        }
        // If there are no fields set, return a default response with status code
        else {
            let mut response = Response::new("Nothing to say".into());
            *response.status_mut() = self.status;
            return response;
        }
    }
}

/// Represents an error in the API response
#[derive(Clone)]
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
