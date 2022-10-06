use aws_sdk_lambda::Client;
use aws_sdk_lambda::types::Blob;
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

pub fn feature_flag_sync(feature: &str) -> bool {
    use tokio::runtime::Runtime;
    let rt  = Runtime::new().unwrap();
    let flag = rt.block_on(async {
        dbg!(feature_flag(feature).await)
    });
    flag
}

pub async fn feature_flag(feature: &str) -> bool {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let payload = serde_json::to_string(&Request {
        feature: feature.to_string()
    }).unwrap();
    let output = client.invoke().function_name("feature_flags").payload(Blob::new(payload)).send().await.unwrap();
    let json = String::from_utf8(output.payload().unwrap().clone().into_inner()).unwrap();
    dbg!(&json);
    let resp: Response = serde_json::from_str(&json).unwrap();
    resp.flag
}

#[test]
fn test_feature_flag() {
    assert_eq!(feature_flag_sync("POST_RATE_ADJUSTMENT_ACTION"), true);
    assert_eq!(feature_flag_sync("POST_MARGIN_ADJUSTMENT_ACTION"), false);
}
