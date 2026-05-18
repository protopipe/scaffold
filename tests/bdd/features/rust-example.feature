@rust-example @blackbox
Feature: Rust example service
  The service composes its own response with a deterministic upstream response.

  Scenario: Hello endpoint includes the upstream message
    Given WireMock returns "from wiremock" for "/external/message"
    When I call the rust-example "/hello" endpoint
    Then the response status is 200
    And the JSON response field "message" is "hello world"
    And the JSON response field "upstream_message" is "from wiremock"

