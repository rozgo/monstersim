# Simulation engine for a provably correct MonsterEOS economy 
An experimental economy simulation engine designed to model different aspects of MonsterEOS game design. A category theory inspired rules engine, where objects are assets and morphisms are rates, creating a mathematical formalization for assets, accounts, rates and transactions.

The end goal is to optimize for provably fair behavioral economics design.

## Requirements:
- Install Rust [https://rustup.rs](https://rustup.rs)
```
$ curl https://sh.rustup.rs -sSf | sh
```
Set default Rust to **nightly** (for latest features):
```
$ rustup default nightly
```

## Run tests:
```
$ cargo test -- --nocapture
```

If everything worked you should see test results like this:
```
running 1 test
RIP! Monster was alive for 3 hours, 5 minutes and 12 seconds.
test monster_lifetime_until_death ... ok
```
