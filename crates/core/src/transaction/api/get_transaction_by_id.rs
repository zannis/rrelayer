use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};

use crate::shared::{not_found, HttpError};
use crate::{
    app_state::AppState,
    transaction::{
        get_transaction_by_id,
        types::{Transaction, TransactionId},
    },
};

/// API endpoint to retrieve a transaction by its ID.
pub async fn get_transaction_by_id_api(
    State(state): State<Arc<AppState>>,
    Path(id): Path<TransactionId>,
    headers: HeaderMap,
) -> Result<Json<Transaction>, HttpError> {
    state.validate_allowed_passed_basic_auth(&headers)?;

    let transaction = get_transaction_by_id(&state.cache, &state.db, id)
        .await?
        .ok_or(not_found("Transaction not found".to_string()))?;

    state.validate_auth_basic_or_api_key(&headers, &transaction.from, &transaction.chain_id)?;

    Ok(Json(transaction))
}
