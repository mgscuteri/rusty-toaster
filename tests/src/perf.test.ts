import * as assert from "node:assert";
import { describe, test } from "node:test";
import axios from "axios";
import * as https from "https";
import {
  from,
  interval,
  lastValueFrom,
  map,
  mergeMap,
  of,
  range,
  switchMap,
  take,
  toArray,
} from "rxjs";
import { promiseHooks } from "node:v8";

//const baseURL = "https://localhost";
const baseURL = "https://192.168.0.16";

// const endpoint = "/a";
const endpoint = "albums/JellyFish/DSCF0461.JPG"; // The server URL

describe("performance", () => {
  const ax = axios.create({
    baseURL,
    httpsAgent: new https.Agent({
      rejectUnauthorized: false, // Ignore certificate validation
    }),
    validateStatus: () => true, // Allow any status codes for testing purposes
  });

  // test("Traffic with delays", async () => {
  //   const totalRequests = 10; // Number of requests
  //   const pauseInMilliseconds = 100; // Pause duration between requests

  //   // Observable to generate requests with pauses
  //   const requestObservable = interval(pauseInMilliseconds).pipe(
  //     take(totalRequests), // Limit to 25 requests
  //     mergeMap(() => {
  //       try {
  //         return from(ax.get("/")).pipe(map((p) => p.status)); // Use shared Axios instance
  //       } catch (error: any) {
  //         console.error(`Request failed:`, error.message);
  //         return of(0);
  //       }
  //     }),
  //     toArray()
  //   );

  //   // Wait for all requests to finish
  //   const responseCodes = await lastValueFrom(requestObservable);

  //   console.log("Response Codes:", responseCodes);

  //   // Assert that at least one response code is 429 (Too Many Requests)
  //   assert.ok(
  //     responseCodes.every((code) => code === 200),
  //     "Expected all response codes to be 200"
  //   );
  // });

  test("Traffic without delays", async () => {
    const endpoint = `${baseURL}/albums/JellyFish/DSCF0461.JPG`; // Replace with your actual endpoint
    const totalRequests = 5; // Number of requests

    // Create an array of Promises for simultaneous requests
    const requests = Array.from({ length: totalRequests }, async () => {
      try {
        const response = await axios.get(endpoint, {
          httpsAgent: new https.Agent({
            rejectUnauthorized: false, // Ignore certificate validation
          }),
        });
        return response.status; // Return the HTTP status code
      } catch (error: any) {
        console.error(`Request failed:`, error.message);
        return error.status ?? 0; // Use 0 for failed requests
      }
    });

    // Wait for all requests to complete
    const results: number[] = await Promise.all(requests);

    // Log the response codes for debugging purposes
    console.log("Response Codes:", results);

    // Assert that all response codes are 200
    assert.ok(
      results.every((code) => code === 200),
      "Expected all response codes to be 200"
    );
  });
});
