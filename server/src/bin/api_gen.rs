use std::fs;

use librero_server::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let openapi = ApiDoc::openapi();
    let json = serde_json::to_string_pretty(&openapi).unwrap();
    fs::write("../openapi.json", json).expect("Unable to write file");
    println!("OpenAPI JSON file generated: openapi.json");
}
