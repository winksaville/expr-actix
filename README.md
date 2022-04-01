# Experiment with Actix

This is a cargo workspace for experimenting with [Actix](https://github.com/actix/actix)

## i-am-alive

This has warnings right now, asking on Discord if a PR fixing them would be accepted.

```
$ cargo run --bin i-am-alive
warning: unused import: `Addr`
 --> i-am-alive/src/main.rs:1:20
  |
1 | use actix::{Actor, Addr, Context, System};
  |                    ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `addr`
  --> i-am-alive/src/main.rs:17:9
   |
17 |     let addr = system.block_on(async { MyActor.start() });
   |         ^^^^ help: if this is intentional, prefix it with an underscore: `_addr`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `ctx`
 --> i-am-alive/src/main.rs:8:27
  |
8 |     fn started(&mut self, ctx: &mut Self::Context) {
  |                           ^^^ help: if this is intentional, prefix it with an underscore: `_ctx`

warning: variable does not need to be mutable
  --> i-am-alive/src/main.rs:15:9
   |
15 |     let mut system = System::new();
   |         ----^^^^^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: unused `Result` that must be used
  --> i-am-alive/src/main.rs:19:5
   |
19 |     system.run();
   |     ^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `i-am-alive` (bin "i-am-alive") generated 5 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/i-am-alive`
I am alive!
```

## hello-world

This is from [Implementing an Actor](https://github.com/winksaville/actix#implementing-an-actor)


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
