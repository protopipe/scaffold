use cucumber::{given, then, when, World};
use futures::FutureExt;
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::convert::Infallible;
use std::env;
use std::time::Duration;

#[derive(Debug, World)]
#[world(init = Self::new)]
struct ApiWorld {
    client: reqwest::Client,
    service_base_url: String,
    wiremock_base_url: String,
    response_status: Option<StatusCode>,
    response_json: Option<Value>,
}

impl ApiWorld {
    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            client: reqwest::Client::new(),
            service_base_url: env::var("SERVICE_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            wiremock_base_url: env::var("WIREMOCK_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8081".to_string()),
            response_status: None,
            response_json: None,
        })
    }

    async fn cleanup(&mut self) {
        wait_for_http(
            &self.client,
            &format!("{}/__admin/mappings", self.wiremock_base_url),
        )
        .await;

        let mappings_response = self
            .client
            .delete(format!("{}/__admin/mappings", self.wiremock_base_url))
            .send()
            .await
            .expect("reset WireMock mappings");

        assert!(
            mappings_response.status().is_success(),
            "WireMock mapping reset failed with status {}",
            mappings_response.status()
        );

        let requests_response = self
            .client
            .delete(format!("{}/__admin/requests", self.wiremock_base_url))
            .send()
            .await
            .expect("reset WireMock request journal");

        assert!(
            requests_response.status().is_success(),
            "WireMock request journal reset failed with status {}",
            requests_response.status()
        );

        self.response_status = None;
        self.response_json = None;
    }
}

#[given(expr = "WireMock returns {string} for {string}")]
async fn wiremock_returns(world: &mut ApiWorld, message: String, path: String) {
    let mapping = json!({
        "request": {
            "method": "GET",
            "urlPath": path
        },
        "response": {
            "status": 200,
            "headers": {
                "Content-Type": "application/json"
            },
            "jsonBody": {
                "message": message
            }
        }
    });

    let mapping_url = format!("{}/__admin/mappings", world.wiremock_base_url);
    let response = world
        .client
        .post(mapping_url)
        .json(&mapping)
        .send()
        .await
        .expect("create WireMock mapping");

    assert!(
        response.status().is_success(),
        "WireMock mapping failed with status {}",
        response.status()
    );
}

#[when(expr = "I call the rust-example {string} endpoint")]
async fn call_endpoint(world: &mut ApiWorld, path: String) {
    wait_for_http(&world.client, &format!("{}/health", world.service_base_url)).await;

    let url = format!("{}{}", world.service_base_url, path);
    let response = world.client.get(url).send().await.expect("call service");

    world.response_status = Some(response.status());
    world.response_json = Some(response.json::<Value>().await.expect("parse JSON response"));
}

#[then(expr = "the response status is {int}")]
async fn response_status_is(world: &mut ApiWorld, expected: u16) {
    let status = world.response_status.expect("response status should exist");
    assert_eq!(status.as_u16(), expected);
}

#[then(expr = "the JSON response field {string} is {string}")]
async fn json_response_field_is(world: &mut ApiWorld, field: String, expected: String) {
    let body = world
        .response_json
        .as_ref()
        .expect("response JSON should exist");
    assert_eq!(
        body.get(field).and_then(Value::as_str),
        Some(expected.as_str())
    );
}

async fn wait_for_http(client: &reqwest::Client, url: &str) {
    let mut last_error = None;

    for _ in 0..30 {
        match client.get(url).send().await {
            Ok(response) if response.status().is_success() => return,
            Ok(response) => last_error = Some(format!("status {}", response.status())),
            Err(error) => last_error = Some(error.to_string()),
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    panic!("service at {url} did not become ready: {last_error:?}");
}

#[tokio::main]
async fn main() {
    let features = env::var("CUCUMBER_FEATURES").unwrap_or_else(|_| "features".to_string());
    ApiWorld::cucumber()
        .max_concurrent_scenarios(1)
        .before(|_feature, _rule, _scenario, world| {
            async move {
                world.cleanup().await;
            }
            .boxed_local()
        })
        .after(
            |_feature, _rule, _scenario, _scenario_finished, _optional_world| {
                async move {
                    // Cleanup runs before each scenario so failed scenario state
                    // remains available in logs for diagnosis.
                }
                .boxed_local()
            },
        )
        .run(features)
        .await;
}
