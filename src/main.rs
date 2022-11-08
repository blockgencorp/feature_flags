use feature_flags::{Request, Response};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use std::collections::HashSet;

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let feature = event.payload.feature;
    let vars: Vec<(String, String)> = dotenv::vars().collect();
    let mut flags = HashSet::new();
    for (key, flag) in vars {
        if let Ok(true) = flag.parse::<bool>() {
            flags.insert(key);
        }
    }
    let flag = flags.contains(&feature);
    let resp = Response { feature, flag };
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
