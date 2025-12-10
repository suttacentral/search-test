# Acceptance Testing for SuttaCentral Search

The `search-test` utility lets us test the "instant search" endpoint for SuttaCentral. Test suites are written as
TOML files and are run at the command line. Each test case makes a single request to the endpoint supplied in the
test suite TOML file and the response is checked and the test outcome printed to the screen. A typical request would
look like this in curl:

```
$ curl -X POST --json '["en"]' 'https://suttacentral.net/api/search/instant?limit=50&query=adze&language=en&restrict=all&matchpartial=false'
```

The search is defined in both the URL, with the query and a few other parameters, and the body of the request,
which is simply a list of languages to search in.

## Installing

Head on over to the releases page to download and install the utility:

https://github.com/suttacentral/search-test/releases

## Building

`search-test` is written entirely in Rust, so to build you'll need the toolchain installed:

https://rust-lang.org/tools/install/

On Linux Mint 22.1 I needed to add a single dependency:

```
sudo apt install libssl-dev
```

Building and running is then straightforward using `cargo`. E.g. `cargo run examples.toml`.

## Creating test cases

Each suite is contained in a single TOML file. It begins with a settings section, defaults section and then one or more
test cases.

### Settings

There are two possible settings:

- `endpoint` tells us where to send the requests. This would normally be on localhost but can be used for staging and
  production as needs be.
- `delay` is the time to wait between tests in milliseconds. This is optional and will default to zero milliseconds if
  not provided.

```toml
[settings]
endpoint = "http://localhost/api/search/instant"
delay = 100
```

### Defaults

This entire section can be omitted, but generally you will want to have some sensible defaults for your tests. If any
defaults aren't specified then each test case must include them. i.e. if you don't specify `limit`, then every test
case will have to include `limit`.

- `limit` is not specified by the SuttaCentral user directly and might be hardcoded in the site code.
- `restrict` : I've only tried "all" as I don't know what it does.
- `site-language` is the language selected by the SuttaCentral user and used throughout the site.
- `match-partial` will be true if the user selects "Match partial" when searching, otherwise false.
- `selected-languages` is an array of language codes equivalent to the items ticked by the user.

```toml
[defaults]
limit = 10
site-language = "en"
restrict = "all"
match-partial = false
selected-languages = ["en", "pli"]
```

### Test cases

The rest of the TOML file must contain at least one test case, and as many as you like. If all defaults are given
values,
the only required fields are

- `description` is a unique description for what this test case is for.
- `query` what you would type into the query field.

```toml
[[test-case]]
description = "Causes internal server error"
query = "by:sujato+\"the+Bamboo+Grove\"" 
```

Any of the other fields found in the defaults can be overridden:

```toml
[[test-case]]
description = "The most important sutta in Pāli"
query = "dhamma"
selected-languages = ["pli"]
```

### Great expectations!

The test cases above are great if you just want to know that they run successfully, or want to know how fast they
execute. However, `search-test` lets us provide an expected response from the server. There are four kinds of
expected response:

- `expected.sutta` for normal text hits
- `expected.dictionary` for dictionary hits
- `expected.suttaplex` for suttaplexes
- `expected.other` for any other kind of hits, a guide page for instance.

```toml
[[test-case]]
description = "The most important sutta in Pāli"
query = "dhamma"
selected-languages = ["pli"]
expected.sutta = "/an1.51-60/pli/ms"
```

You can only specify one of the above. If you want to check, say, that a query returns a sutta and a dictionary result
then you need to specify two test cases, even though a single request could return both.

### Ranking

We'd like to know that our results are ordered with the best results at the top. You can add a minimum rank to your
expected result. Naturally there needs to be an expected result to be ranked. The test suite won't run otherwise.

```toml
[[test-case]]
description = "Metta sutta is in the top three with partial match"
query = "metta"
match-partial = true
expected.sutta = "/snp5.1/en/sujato"
expected.min-rank = 3
```

### Result IDs

In order to specify expected results, each result has an ID, scraped from the JSON response.

- `expected.sutta` will typically be in three parts: `/snp5.1/en/sujato`
- `expected.dictionary` starts the same, with the end being the word: `/define/metta`
- `expected.suttaplex` is just the uid: `mn1`.
- `expected.volpage` takes the form `PTS SN ii 1`
- `expected.other` might take any form, but an examples would include `/sn-guide-sujato` and `/licencing`

Apart from suttaplexes, you can find the ids via the url of the search result. For example, given:

`https://suttacentral.net/mn1/en/sujato?lang=en&layout=plain&reference=none&notes=asterisk&highlight=false&script=latin`

the ID would be `/mn1/en/sujato`

Or for `https://suttacentral.net/define/metta?lang=en` you'd use `/define/metta`

### A complete TOML example

```toml
[settings]
endpoint = "http://localhost/api/search/instant"
delay = 10

[defaults]
limit = 10
site-language = "en"
restrict = "all"
match-partial = false
selected-languages = ["en", "pli"]

[[test-case]]
description = "Causes internal server error"
query = "by:sujato+\"the+Bamboo+Grove\""

[[test-case]]
description = "Search is successful"
query = "dhamma"

[[test-case]]
description = "The most important sutta in Pali"
query = "dhamma"
selected-languages = ["pli"]
expected.sutta = "/an1.51-60/pli/ms"

[[test-case]]
description = "This sutta is ranked too low"
query = "snake"
selected-languages = ["en", "pli"]
expected.sutta = "/an5.77/en/sujato"
expected.min-rank = 3
```

## Running test suites

With `search-test` on your path, it takes a single argument, the path to the test suite to be run:

```
$ search-test examples.toml 
Running tests against endpoint http://localhost/api/search/instant with 10ms delay

ERROR   55ms   Causes internal server error
  Expected status code to be 200 OK but got 502 Bad Gateway
PASSED  679ms  Search is successful
PASSED  650ms  The most important sutta in Pali
FAILED  388ms  This sutta is ranked too low
  Expected Text hit /an5.77/en/sujato to have minimum rank of 3 but it was found at rank 4
FAILED  356ms  Metta sutta should be in the top three but isn't in results at all
  Minium rank 3 expected for Text hit /snp5.1/en/sujato but it was not found
PASSED  1707ms Metta sutta is in the top three with partial match
PASSED  322ms  A pali term with diacritics
PASSED  363ms  Metta is in the dictionary
PASSED  335ms  Guide to The Linked Discourses
6 passed, 2 failed, 1 encountered an error
```
