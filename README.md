# Experiment with Actix

This is a cargo workspace for experimenting with [Actix](https://github.com/actix/actix)

## i-am-alive

A simple actor from [Implementing an Actor](https://github.com/actix/actix/tree/8d1081fc6f8d76a3ca5c3e8338e423bca4c097ca#implementing-an-actor)

```
$ cargo run --bin i-am-alive
   Compiling i-am-alive v0.1.0 (/home/wink/prgs/rust/myrepos/expr-actix/i-am-alive)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/i-am-alive`
I am alive!
```

## calculator

This is based on [Handle Messages](https://github.com/actix/actix/tree/8d1081fc6f8d76a3ca5c3e8338e423bca4c097ca#handle-messages):

```
wink@3900x:~/prgs/rust/myrepos/expr-actix/calculator (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/wink/prgs/rust/myrepos/expr-actix/target/debug/calculator`
SUM: 15
```

## ping

This based on [Actor State And Subscription For Specific Messages](https://github.com/actix/actix/tree/8d1081fc6f8d76a3ca5c3e8338e423bca4c097ca#actor-state-and-subscription-for-specific-messages

```
wink@3900x:~/prgs/rust/myrepos/expr-actix/ping (main)
$ cargo run
   Compiling ping v0.1.0 (/home/wink/prgs/rust/myrepos/expr-actix/ping)
    Finished dev [unoptimized + debuginfo] target(s) in 0.56s
     Running `/home/wink/prgs/rust/myrepos/expr-actix/target/debug/ping`
[Game 2] Ping received 10
[Game 1] Ping received 11
[Game 2] Ping received 12
[Game 1] Ping received 13
[Game 2] Ping received 14
[Game 1] Ping received 15
[Game 2] Ping received 16
[Game 1] Ping received 17
[Game 2] Ping received 18
[Game 1] Ping received 19
[Game 2] Ping received 20
[Game 1] Ping received 21
[Game 2] Ping received 22
[Game 1] Ping received 23
[Game 2] Ping received 24
[Game 1] Ping received 25
[Game 2] Ping received 26
[Game 1] Ping received 27
[Game 2] Ping received 28
[Game 1] Ping received 29
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
