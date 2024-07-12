import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate } from 'k6/metrics';

export const options = {
  stages: [
    { duration: '30s', target: 200 }, // Ramp up to 1000 users over 2 minutes
    { duration: '30s', target: 200 }, // Stay at 1000 users for 3 minutes
    { duration: '30s', target: 0 },    // Ramp down to 0 users over 2 minutes
  ],
  thresholds: {
    http_req_duration: ['p(95)<500'], // 95% of requests must complete below 500ms
    'errors{endpoint:rust}': ['rate<0.05'],  // Errors must be less than 5% for Rust endpoint
    'errors{endpoint:python}': ['rate<0.05'], // Errors must be less than 5% for Python endpoint
  },
};

const errorRate = new Rate('errors');

export default function () {
  const endpoints = [
    { url: 'http://34.86.178.8:8082/ihatethismovie', tag: 'rust' },
    { url: 'http://34.86.68.125:8000/ihatethismovie', tag: 'python' },
  ];

  endpoints.forEach(endpoint => {
    // Include tags in the request options
    const res = http.get(endpoint.url, { tags: { backend: endpoint.tag } });
    const result = check(res, {
      [`${endpoint.tag} status is 200`]: (r) => r.status === 200,
    });
  
    if (!result) {
      errorRate.add(1, { endpoint: endpoint.tag });
    }
  
    sleep(1);
  });
}