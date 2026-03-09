use axum::{routing::post, Router, Json, response::IntoResponse};
use axum::http::StatusCode;
use solana_sdk::{pubkey::Pubkey, system_instruction};
use std::str::FromStr;
use base64::{engine::general_purpose, Engine as _};
use crate::models::*;
use crate::utils::error_response;

pub fn routes() -> Router {
    Router::new().route("/send/sol", post(send_sol))
}

async fn send_sol(Json(payload): Json<SendSolRequest>) -> impl IntoResponse {
    let from = match Pubkey::from_str(&payload.from) {
        Ok(pk) => pk,
        Err(_) => return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "success": false, "error": "Invalid from address" }))
        ).into_response(),
    };

    let to = match Pubkey::from_str(&payload.to) {
        Ok(pk) => pk,
        Err(_) => return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "success": false, "error": "Invalid to address" }))
        ).into_response(),
    };

    let instr = system_instruction::transfer(&from, &to, payload.lamports);

    let accounts = instr.accounts.iter().map(|a| {
        serde_json::json!({
            "pubkey": a.pubkey.to_string(),
            "is_signer": a.is_signer,
            "is_writable": a.is_writable
        })
    }).collect::<Vec<_>>();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "data": {
                "program_id": instr.program_id.to_string(),
                "accounts": accounts,
                "instruction_data": general_purpose::STANDARD.encode(instr.data),
            }
        }))
    ).into_response()
}