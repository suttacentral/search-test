# Version 0.2.0

- Supports volpage searches.
- Extensive refactoring to support multiple parsers and improve the overall design.
- A subtle change in design means that a test without an `expect` will return success so long as something is returned,
  and we get a status code 200. If the response is not valid JSON the test will still return "PASSED".

# Version 0.1.0

Initial release. Functional, but does not support volpage and ref searches.