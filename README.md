# Minimal project setup to trigger hanging tests

Run with `cargo test -- --nocapture`

Rust version:

```
rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/exul/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu

installed targets for active toolchain
--------------------------------------

wasm32-unknown-unknown
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
```

OS:

```
uname -a
Linux xi 5.5.5-arch1-1 #1 SMP PREEMPT Thu, 20 Feb 2020 18:23:09 +0000 x86_64 GNU/Linux
```

Output when tests hang:

```
cargo test -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.06s
     Running target/debug/deps/wiremock_min-82e7dc40df60a582

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/hello-1ea17145e8a9e2f7

running 2 tests
sending surf request
sending reqwest request
```

Output when tests pass:

```
cargo test -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.06s
     Running target/debug/deps/wiremock_min-82e7dc40df60a582

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/hello-1ea17145e8a9e2f7

running 2 tests
sending surf request
sending reqwest request
reqwest request done
test hello_reqwest ... ok
surf request done
test hello_surf ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests wiremock-min

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
