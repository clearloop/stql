# StQL

A QL-service implementation of [sonata](https://github.com/clearloop/sonata), and now we are using embeded database [sled](https://github.com/spacejam/sled) for the beta version.

StQL is kind like [GraphQL](http://graphql.org), we hash the sonata code and select the result from k-v database, it's more like a standard.

StQL aims to `serverless query engine`, which means that you don't need to write complex routes in network request, we hash them, and the result is unique, the bad face of StQL is that it doesn't have indexes, you can write a cache server if you need to handle the big data.

## Usage

For example, we got a `user` table here.

| query             | sha256(key)                                                      | result(value)            |
|-------------------|------------------------------------------------------------------|--------------------------|
| user              | 04f8996da763b7a969b1028ee3007569eaf3a635486ddab211d512c85b9df8fb | user info bytes          |
| (username user)   | 6afedbbb9255c16284c28d126608793c85e41808eda3891f295239054fc88763 | username of user         |
| (articles user)   | 50a05394b870e1733514098fc2945b2a5b410a9fd146600bcf33307108f3e0fb | user article indexs      |
| (articles user 0) | 7ed9aefa632e3100575499a5f561a2f18bdf998659faeb8c672b2b8cf436e3f8 | the No.0 article of user |

What about the ciper request? 

We can sign the `server-returned` token with your `query` with `ed25519`.

### Set

For a http request template:

| Request      | body                    | result                                                           |
|--------------|-------------------------|------------------------------------------------------------------|
| POST `/user` | {"query": "(user ...)"} | e7280aa76daecbcc09565500ae86058762289f49b7cbf7c8d983ee9628c8d811 |

### Get

For a http request template:

| Request                                                                 | result |
|-------------------------------------------------------------------------|--------|
| GET `/e7280aa76daecbcc09565500ae86058762289f49b7cbf7c8d983ee9628c8d811` | (...)  |

the `/e7280aa76daecbcc09565500ae86058762289f49b7cbf7c8d983ee9628c8d811` is the hash of `(user ...)`

## Operation

The unbelievable part of StQL is, the query in sonata can __opreate__ in server-side, kind like smart-contract!

For example: 

```
use stql::StQL;

let s = StQL::new("/usr/local/etc/stql").use("sled");

let set_query = r#"(foo "I'm foo's content!")"#;
let hash = StQL.run(query);
// hash: 2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae

let get_query = r#"(foo)"#
let foo = StQL.run(get_query);
// foo: I'm foo's content!

let get_query_by_hash = r#"(2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae)"
let foo_content = StQL.run(get_query);
// foo_content = I'm foo's content!
```
