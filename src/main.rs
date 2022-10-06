use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Request {
    feature: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    feature: String,
    flag: bool,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let feature = event.payload.feature;
    let flags = HashSet::from([
        "POST_RATE_ADJUSTMENT_ACTION".to_string(),
        "POST_PRICE_RULE_ACTION".to_string(),
        // "POST_PRICE_RULE_ACTION".to_string(),
    ]);
    let flag = flags.contains(&feature);
    let resp = Response {
        feature,
        flag,
    };
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
