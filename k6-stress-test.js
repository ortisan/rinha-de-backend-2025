import http from 'k6/http';
import {check, sleep} from 'k6';
import { uuidv4 } from 'https://jslib.k6.io/k6-utils/1.4.0/index.js';


export const options = {
    // Define test scenarios
    scenarios: {
        // Warm-up phase with low load
        warm_up: {
            executor: 'ramping-vus',
            startVUs: 100,
            stages: [
                {duration: '30s', target: 10},
            ],
        },
        // Constant load phase
        // constant_load: {
        //     executor: 'constant-vus',
        //     vus: 50,
        //     duration: '1m',
        //     startTime: '30s',
        // },
        // // Stress test phase with increasing load
        // stress_test: {
        //     executor: 'ramping-vus',
        //     startVUs: 50,
        //     stages: [
        //         {duration: '1m', target: 100},
        //         {duration: '30s', target: 100},
        //         {duration: '30s', target: 0},
        //     ],
        //     startTime: '1m30s',
        // },
    },
    thresholds: {
        http_req_duration: ['p(95)<500'], // 95% of requests should complete within 500ms
        http_req_failed: ['rate<0.01'],   // Less than 1% of requests should fail
    },
};

export default function () {

    const getPayload = () => JSON.stringify({
        correlationId: uuidv4(),
        amount: Math.round(Math.random() * 100000) / 100
    });

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    // Send POST request to the payments endpoint
    const response = http.post('http://localhost:8000/payments/', getPayload(), params);

    // Check if the request was successful
    check(response, {
        'status is 200': (r) => r.status === 200,
    });

    // Add a small sleep to prevent overwhelming the server
    sleep(0.1);
}