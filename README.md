# neptune-types

This crate provides re-implementations of types from the [`neptune-cash`](https://docs.rs/neptune-cash) crate that are exposed via its RPC API.

## WARNING: EXPERIMENTAL!

This crate is in an early, very rough prototype state. Everything is subject to change, or it could be abandoned altogether.

## Overview

The primary motivation for this crate is to make the core data types used in `neptune-cash` available in a WebAssembly (WASM) environment. This is useful for developing browser-based GUI wallet software and other web-based tools that need to interact with the Neptune network.

The types in this crate are designed to be direct, binary-compatible replacements for their counterparts in `neptune-cash`.

## Project Goals

1. **WASM Compatibility:** The immediate goal is to provide a set of types that can be compiled to WASM, allowing developers to build rich client-side applications (e.g., GUI wallets) that can interact with the `neptune-cash` RPC API.

2. **Serialization Compatibility:** It is a requirement/guarantee that the types defined in `neptune-types` are serializable to/from the original types in `neptune-cash`. This is achieved by ensuring field order of types are identical, and via a set of unit tests that actually de/serialize in both directions and compare the serialized data for equality.  Thus changes/breakages can be detected when the tests are run.

3. **RPC API Compatibility:** The neptune-cash RPC API is implemented via `tarpc` which automatically generates a Client type from a trait defined in neptune-cash.  The Client type calls the server and performs necessary serialization steps.  The `neptune-types` crate mirrors this trait from neptune-cash, but the types used in the method signatures are from `neptune-types`.  In this way, an app can use the `neptune-types` tarpc client to call a neptune-core (neptune-cash executable) server.

3. **Future Integration:** A longer-term objective is to have `neptune-cash` itself depend on `neptune-types`. This would establish a single source of truth for these core data structures, simplifying maintenance and reducing code duplication.  Once that is complete, the RPC trait itself could be made a separate crate.

## Non Goals

* Not attempting to include all types from neptune-cash, only those needed for wallet & rpc purposes.
* Not attempting to have every single method or derive as each equivalent neptune-cash type.  Sometimes that is not feasible.

## Unit Tests

There are two types of unit test modules in the source files of this crate.

1. The original unit tests from neptune-cash.  These presently do not compile and are gated behind feature `original-tests`.

2. de/serialization tests that verify compatibility with neptune-cash types across bincode, serde-json, and serde-json-wasm serialization methods. These do compile and can be run with `cargo test`, however some still fail because the code to create the relevant type is stubbed with todo!().

## Usage

Add this crate to your `Cargo.toml`:

```
[dependencies]

# note: neptune-types is not on crates.io yet.  It is generally best to pick a specific github revision to use.
neptune-types = {git = "https://github.com/Neptune-Crypto/neptune-types/", rev = "<revision>"}
```

## Run tests

cargo test

## wasm check/build

one time:

```
rustup target add wasm32-unknown-unknown
```

and then:


```
cargo check --target wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```
