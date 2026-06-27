# Literalize

A library to build literal-based singleton types and values.

## Installation

To install this package, run the following command:

```sh
cargo add literalize
```

## Usage

Apply the `literal` attribute to a unit struct with a string, integer, float, or boolean literal.

```rust
use literalize::literal;

#[literal("not_found")]
struct NotFoundErrorCode;

#[literal(404)]
struct NotFoundHttpStatusCode;

#[literal(3.14)]
struct PiApprox;

#[literal(true)]
struct FeatureEnabled;
```

Each generates a zero-sized singleton type with an inherent `VALUE` constant, `Default`, `Deref`, and `Debug` impls. Nest them in enums to build tagged constant sets:

```rust
enum ErrorCode {
    NotFound(NotFoundErrorCode),
}
```

## Features

- `serde` - Implement `Deserialize` and `Serialize` from [`serde`](https://crates.io/crates/serde).
- `utoipa` - Implement `ToSchema` from [`utoipa`](https://crates.io/crates/utoipa).
- `all` - Enable both `serde` and `utoipa`.

## Contributing

For contributing, please refer to the [contributing guide](./CONTRIBUTING.md).

## License

This project is licensed under the terms of the MIT license.
