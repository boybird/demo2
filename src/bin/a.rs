extern crate frank_jwt;
#[macro_use]
extern crate serde_json;
use frank_jwt::{decode, encode, validate_signature, Algorithm, ValidationOptions};
use std::path::Path;

fn main() {
    dotenv::dotenv();
    let mut payload = json!({
        "key1": "val1",
        "key2": "val2"
    });

    let mut header = json!({});
    let secret = dotenv::var("secret").unwrap_or("secret123".to_owned());

    let p1 = json!(1);
    let header = json!({});
    let jwt1 = encode(header, &secret, &p1, Algorithm::HS256).unwrap();
    let maybe_res = decode(
        &jwt1,
        &secret,
        Algorithm::HS256,
        &ValidationOptions::dangerous(),
    );
    println!("{:#?}", maybe_res);
    assert!(maybe_res.is_ok());
}
