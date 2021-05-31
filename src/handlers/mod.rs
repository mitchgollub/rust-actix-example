use actix_web::http::Uri;
use actix_web::HttpResponse;
use actix_web::{web, web::ServiceConfig};
use dynamodb::model::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};
use dynamodb::Endpoint;
use tracing::info;

pub fn app_config(config: &mut ServiceConfig) {
    let health_check = web::resource("/").route(web::get().to(health));
    let dynamo = web::resource("/dynamo").route(web::get().to(dynamo));

    config.service(health_check).service(dynamo);
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[tokio::main]
pub async fn dynamo() -> HttpResponse {
    let client = dynamodb::Client::from_conf(
        dynamodb::Config::builder()
            .endpoint_resolver(Endpoint::immutable(Uri::from_static(
                "http://localhost:8000",
            )))
            .build(),
    );

    let create_req = client
        .create_table()
        .set_table_name(Some(String::from("my_rust_table")))
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("k")
                .key_type(KeyType::Hash)
                .build(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("k")
                .attribute_type(ScalarAttributeType::S)
                .build(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .write_capacity_units(10)
                .read_capacity_units(10)
                .build(),
        )
        .send()
        .await
        .expect("Sending create table request");

    info!("Created DynamoDB table: {:?}", create_req);

    let req = client.list_tables().limit(10);
    let resp = req.send().await.expect("Sending Dynamo request");
    info!("Current DynamoDB tables: {:?}", resp.table_names);
    HttpResponse::Ok().finish()
}
