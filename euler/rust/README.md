# Euler by Rust

The project is initiated by following commands:

```sh
$ cd euler
$ cargo new euler && mv euler rust
```

## How to Run Test

Test all:

```sh
$ cd euler/rust
$ cargo test
```

Test one:

```sh
$ cd euler/rust
$ cargo test euler_17
$ cargo test euler_17 -- --nocapture # 允许打印日志
```

Benchmark all:

```sh
$ cd euler/rust
$ cargo bench
```

Benchmark one:

```sh
$ cd euler/rust
$ cargo bench euler_15
```

## How to Write Test

See example [src/euler_14.rs](./src/euler_14.rs)

benchmark 需要 nightly 版本。
