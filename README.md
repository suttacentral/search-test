# System Testing for SuttaCentral Search

## Overview

[SuttaCentral.net](https://SuttaCentral.net) allows its users to search for specific texts via a search bar the top of
the screen. When we type in a query a drop down list of suggestions appears. These results are provided by a third
party search engine, [Algolia](https://www.algolia.com/). We can then navigate directly to those texts, or go to the
search engine results page. These results are served via a POST request to an endpoint. For example, using curl:

`$ curl -X POST --json '["en"]' 'https://suttacentral.net/api/search/instant?limit=50&query=adze&language=en&restrict=all&matchpartial=false'`

Here we're using the search query "adze" and providing an array of language codes in the body.

The objective of this tool is to test the "instant search" API using a combination of a command line utility and
Github workflows to automate testing of the staging and production environments. The CLI can be run locally on the
developers machine against localhost, staging or production and reads test cases from a JSON file.

I'm writing the initial version of the CLI in Rust but will happily rewrite it in Python if required. Likewise
the workflow could be ported to another CI/CD environment (e.g. GitLabs CI).

This is not intended for unit or integration testing, nor is it intended to run on the staging or production servers.
It sits outside the system it is designed to test.