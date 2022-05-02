[![github]](https://github.com/usagi/actix-web-tonic)&ensp;[![crates-io]](https://crates.io/crates/actix-web-tonic)&ensp;[![docs-rs]](https://docs.rs/actix-web-tonic)<br>
[![Build Status](https://travis-ci.org/usagi/actix-web-tonic.svg?branch=master)](https://travis-ci.org/usagi/actix-web-tonic)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

!(image1)[https://imgur.com/5qaPfr3.png]

# actix-web-tonic

- `actix-web-tonic` is the concurrent runner of `actix-web` + `tonic`.
- You can build a rich and fast server app using Web + gRPC so easily with the crate.

## Usage

1. Implement your `actix-web` server same as basically coding, and get the future:

```rust
let actix_future = HttpServer::new(|| { App::new().service(...
                   ...brabrabra...
                   .bind("[::1]:50000").unwrap()
                   .workers(8) // <- # of workers setting is here in actix-web.
                   .run(); // <- this type is `Service`, it's an `impl` of `Future`.
```

2. Implement your `tonic` server same as basically coding, and get the future:


```rust
let tonic_future = Server::builder().add_service(...
                   ...brabrabra...
                   ...serve(addr); // <- this type is a simple `Future` of `asyncl`.
```

3. Invoke in concurrent both of your actix-web future and your tonic future:

```rust
let tonic_worker_threads = 16; // <- # of workers setting is here in tonic.(=# of tokio workers)
let result = actix_web_tonic::invoke(actix_future, tonic_future, tonic_worker_threads);
```

And then, test your server:

```zsh
# gRPC:
$ grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' '[::1]:50001' helloworld.Greeter/SayHello
{
  "message": "Hello Tonic!"
}
```

```zsh
# Web:
$ curl '[::1]:50000/teapot' -v
*   Trying ::1:50000...
* TCP_NODELAY set
* Connected to ::1 (::1) port 50000 (#0)
> GET /teapot HTTP/1.1
> Host: [::1]:50000
> User-Agent: curl/7.68.0
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 418 I'm a teapot
< content-length: 23
< date: Sun, 01 May 2022 23:29:41 GMT
<
* Connection #0 to host ::1 left intact
I like a green tea.🍵%
```

- Want you all of the example?
  - => The full example is here: [examples/simple/](examples/simple/)

## Motivation and Notes

- I had try implements a single bin process actix-web + tonic service but it was little hard to code for every works. Then, I publish this crate. It will be a good little helps to the `future` of me, and ofcourse you who looks this crate.💕
- Of course, this crate is for servers. If your goal is not to provide a server, you want to complete an applications, you will also need crates for the client side to fight WASM, HTML/CSS and UI/UX.💪

## LICENSE

- [MIT](LICENSE.md)

## Author

- USAGI.NETWORK / Usagi Ito <https://github.com/usagi/>
