import http from "k6/http";
import { sleep } from "k6";
import { check } from "k6";

export const options = {
  vus: 2,
  duration: "10s",
  thresholds: {
    "http_req_failed{phase:main}": ["rate<0.01"],
    "http_req_duration{phase:main}": ["p(95)<500"],
  },
};

const baseUrl = __ENV.SERVICE_BASE_URL || "http://localhost:8080";
const wiremockBaseUrl = __ENV.WIREMOCK_BASE_URL || "http://localhost:8081";

export function setup() {
  configureWireMock();
  waitForReadiness();
}

function configureWireMock() {
  waitForWireMock();

  const mapping = {
    request: {
      method: "GET",
      urlPath: "/external/message",
    },
    response: {
      status: 200,
      headers: {
        "Content-Type": "application/json",
      },
      jsonBody: {
        message: "from wiremock",
      },
    },
  };

  const reset = http.del(`${wiremockBaseUrl}/__admin/mappings`, null, {
    tags: { phase: "setup" },
    timeout: "2s",
  });

  if (reset.status < 200 || reset.status >= 300) {
    throw new Error(`could not reset WireMock mappings; status: ${reset.status}`);
  }

  const create = http.post(`${wiremockBaseUrl}/__admin/mappings`, JSON.stringify(mapping), {
    headers: {
      "Content-Type": "application/json",
    },
    tags: { phase: "setup" },
    timeout: "2s",
  });

  if (create.status < 200 || create.status >= 300) {
    throw new Error(`could not create WireMock mapping; status: ${create.status}`);
  }
}

function waitForWireMock() {
  const deadline = Date.now() + 30000;
  let lastStatus = "no response";

  while (Date.now() < deadline) {
    const response = http.get(`${wiremockBaseUrl}/__admin/mappings`, {
      tags: { phase: "setup" },
      timeout: "2s",
    });

    lastStatus = `${response.status}`;

    if (response.status === 200) {
      return;
    }

    sleep(1);
  }

  throw new Error(`WireMock admin API did not become ready; last status: ${lastStatus}`);
}

function waitForReadiness() {
  const deadline = Date.now() + 30000;
  let lastStatus = "no response";

  while (Date.now() < deadline) {
    const response = http.get(`${baseUrl}/ready`, {
      tags: { phase: "setup" },
      timeout: "2s",
    });

    lastStatus = `${response.status}`;

    if (response.status === 200 && response.json("status") === "ready") {
      return;
    }

    sleep(1);
  }

  throw new Error(`service readiness did not become ready for load test; last status: ${lastStatus}`);
}

export default function () {
  const response = http.get(`${baseUrl}/hello`, {
    tags: { phase: "main" },
  });

  check(response, {
    "status is 200": (r) => r.status === 200,
    "contains local message": (r) => r.json("message") === "hello world",
    "contains upstream message": (r) => r.json("upstream_message") === "from wiremock",
  });
}
