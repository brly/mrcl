# mrcl

Mastodon REST API client written in Rust.

refer `https://github.com/tootsuite/documentation/blob/master/Using-the-API/API.md`.

# Getting Started

```rust
extern crate mrcl;

use mrcl::*;

fn getting_started() {
    let server = "server";
    let client_id = "client-id";
    let client_secret = "client-secred";
    let username = "mail address";
    let plane_password = "password";
    let scope = "scope";

    let mut c = Config::new(server, client_id, client_secret, username, plane_password, scope);
    let err = c.authenticate();

    let responses = statuses::get_timelines_home(&c, None, None, None, None).unwrap();

    for r in responses {
        println!("{:?}", r.content);
    }
}
```

# progress

## accounts

* [x] GET /api/v1/accounts/:id
* [x] GET /api/v1/accounts/verify_credentials
* [ ] PATCH /api/v1/accounts/update_credentials
* [x] GET /api/v1/accounts/:id/followers
* [x] GET /api/v1/accounts/:id/following
* [x] POST /api/v1/accounts/:id/follow
* [x] POST /api/v1/accounts/:id/unfollow
* [x] POST /api/v1/accounts/:id/block
* [x] POST /api/v1/accounts/:id/unblock
* [x] POST /api/v1/accounts/:id/mute
* [x] POST /api/v1/accounts/:id/unmute
* [x] GET /api/v1/accounts/relationships

## apps

* [x] POST /api/v1/apps

## blocks

* [x] GET /api/v1/blocks

## favourites

* [x] GET /api/v1/favourites

## follow_requests

* [x] GET /api/v1/follow_requests
* [x] POST /api/v1/follow_requests/:id/authorize
* [x] POST /api/v1/follow_requests/:id/reject

## follows

* [x] POST /api/v1/follows

## instance

* [x] GET /api/v1/instance

## media

* [ ] POST /api/v1/media

## mutes

* [x] GET /api/v1/mutes

## notifications

* [x] GET /api/v1/notifications
* [x] GET /api/v1/notifications/:id
* [x] POST /api/v1/notifications/clear

## reports

* [x] GET /api/v1/reports
* [x] POST /api/v1/reports

## search

* [x] GET /api/v1/search

## statuses

* [x] GET /api/v1/statuses/:id
* [x] GET /api/v1/statuses/:id/context
* [x] GET /api/v1/statuses/:id/card
* [x] GET /api/v1/statuses/:id/reblogged_by
* [x] GET /api/v1/statuses/:id/favourited_by
* [x] POST /api/v1/statuses
* [ ] DELETE /api/v1/statuses/:id
* [x] POST /api/v1/statuses/:id/reblog
* [x] POST /api/v1/statuses/:id/unreblog
* [x] POST /api/v1/statuses/:id/favourite
* [x] POST /api/v1/statuses/:id/unfavourite
* [x] GET /api/v1/timelines/home
* [x] GET /api/v1/timelines/public
* [x] GET /api/v1/timelines/tag/:hashtag

# License

MIT
