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
