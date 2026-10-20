/*
*	Copyright (C) 2026 Kendall Tauser
*
*	This program is free software; you can redistribute it and/or modify
*	it under the terms of the GNU General Public License as published by
*	the Free Software Foundation; either version 2 of the License, or
*	(at your option) any later version.
*
*	This program is distributed in the hope that it will be useful,
*	but WITHOUT ANY WARRANTY; without even the implied warranty of
*	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*	GNU General Public License for more details.
*
*	You should have received a copy of the GNU General Public License along
*	with this program; if not, write to the Free Software Foundation, Inc.,
*	51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

use std::{net::Ipv4Addr, str::FromStr, sync::Arc};

use log::info;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use warp::{
    Filter,
    http::StatusCode,
    reply::{json, with_status},
};

use crate::config::FibServiceConfig;

#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    ok: bool,
}

impl Default for HealthResponse {
    fn default() -> Self {
        Self { ok: true }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct FibResponse {
    numbers: Vec<String>,
}

impl Default for FibResponse {
    fn default() -> Self {
        Self { numbers: vec![] }
    }
}

impl FibResponse {
    fn new(lookup: &Vec<String>, count: u32) -> Self {
        Self {
            numbers: lookup[0..count as usize].to_vec(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct FibErrorResponse {
    code: u16,
    message: String,
}

impl FibErrorResponse {
    fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code: code.as_u16(),
            message: message.into(),
        }
    }
}

/// I am making a conscious tradeoff here. I would prefer to precompute all supported values and
/// tradeoff the memory for not having consumers of the API perhaps needing to wait a long time
/// for a response if they choose to compute a large value.
fn generate_fib_table(max_number: u32) -> Vec<BigUint> {
    let mut table = vec![BigUint::from(0 as u32); max_number as usize];
    table[1] = BigUint::from(1 as usize);

    for idx in 2..max_number {
        let i = idx as usize;
        table[i] = &table[i - 1] + &table[i - 2];
    }

    table
}

/// Separate stringification into two steps. It seems like serialization is the most
/// expensive part of generating large responses, so precompute strings to trade off a
/// bit more memory for time.
fn generate_strings_table(table: Vec<BigUint>) -> Vec<String> {
    let mut stringified_table = vec![];
    for num in table.into_iter() {
        stringified_table.push(num.to_string());
    }

    stringified_table
}

#[test]
fn test_generate_fib_table() {
    let len = 50000;
    let table = generate_fib_table(len);

    // Some hard-coded checks for sanity.
    assert_eq!(table.len(), len as usize);
    assert_eq!(table[0], BigUint::from(0 as u32));
    assert_eq!(table[1], BigUint::from(1 as u32));
    assert_eq!(table[2], BigUint::from(1 as u32));
    assert_eq!(table[3], BigUint::from(2 as u32));
    assert_eq!(table[4], BigUint::from(3 as u32));
    assert_eq!(table[5], BigUint::from(5 as u32));

    // Inductive check.
    for i in 6..=len {
        let idx = i as usize;
        assert_eq!(table[idx], &table[idx - 1] + &table[idx - 2]);
    }
}

pub(super) async fn start_server<'a>(config: FibServiceConfig) {
    info!("pre-computing lookup table...");
    let table = Arc::new(generate_strings_table(generate_fib_table(
        config.max_supported_number,
    )));
    info!("lookup table generated and stringified...");
    let table_filter = warp::any().map(move || table.clone());
    let max_number_filter = warp::any().map(move || config.max_supported_number);

    let fibonacci = warp::get()
        .and(warp::path!("v1" / "fibonacci" / u32))
        .and(warp::path::end())
        .and(max_number_filter)
        .and(table_filter)
        .and_then(generate_fibs);

    let health = warp::get()
        .and(warp::path!("healthz").or(warp::path!("readyz")).unify())
        .and(warp::path::end())
        .map(|| with_status(json(&HealthResponse::default()), StatusCode::OK));

    let catchall = warp::any().map(|| {
        with_status(
            json(&FibErrorResponse::new(
                StatusCode::NOT_FOUND,
                "endpoint not found",
            )),
            StatusCode::NOT_FOUND,
        )
    });

    let routes = fibonacci
        .or(health)
        .or(catchall)
        .with(warp::cors().allow_any_origin().allow_methods(vec!["GET"]));

    info!("serving on {}:{}", &config.listen_host, &config.listen_port);
    warp::serve(routes)
        .run((
            Ipv4Addr::from_str(&config.listen_host).expect("invalid bind ip address"),
            config.listen_port,
        ))
        .await;
}

/// API function for generating results.
async fn generate_fibs(
    count: u32,
    max_number: u32,
    lookup: Arc<Vec<String>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // This behavior is at least explicit, I assume it is up to debate what the behavior should be.
    if count == 0 {
        return Ok(with_status(json(&FibResponse::default()), StatusCode::OK));
    }

    if count > max_number {
        let bad_req = StatusCode::BAD_REQUEST;
        return Ok(with_status(
            json(&FibErrorResponse::new(
                bad_req,
                format!("only {max_number} fibonacci numbers are supported."),
            )),
            bad_req,
        ));
    }

    Ok(with_status(
        json(&FibResponse::new(&lookup, count)),
        StatusCode::OK,
    ))
}
