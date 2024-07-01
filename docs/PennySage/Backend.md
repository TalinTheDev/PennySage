**Language:** Rust
**Framework:** Axum
**DB:** SQLite w/ `sqlx`
**Authentication:** JWT

Possible API Returns:
1. (StatusCode::OK, Json(success: true, data))
2. (StatusCode::Err, Json(false, error))
3. (StatusCode::K, Json(success, data))