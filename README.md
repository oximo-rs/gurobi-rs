# gurobi-rs

Rust bindings for the [Gurobi](https://www.gurobi.com/) optimizer.

> [!NOTE]
> **This is a maintained fork.** It is forked from the original, now-unmaintained [`ykrist/rust-grb`](https://github.com/ykrist/rust-grb) (the `grb` crate) and its FFI crate [`ykrist/grb-sys2`](https://github.com/ykrist/grb-sys2). That project was originally a fork of [`ubnt-intrepid/rust-gurobi`](https://github.com/ubnt-intrepid/rust-gurobi).
>
> Full credit to the original authors, see [`LICENSE-GRB`](crates/gurobi-rs/LICENSE-GRB) and [`LICENSE-RUST-GUROBI`](crates/gurobi-rs/LICENSE-RUST-GUROBI).

## Workspace layout

| Crate                                 | Path                  | Upstream name | Description                               |
| ------------------------------------- | --------------------- | ------------- | ----------------------------------------- |
| [`gurobi-rs`](crates/gurobi-rs)       | `crates/gurobi-rs`    | `grb`         | High-level Gurobi API                     |
| [`guro-sys`](crates/guro-sys)         | `crates/guro-sys`     | `grb-sys2`    | Low-level C API FFI bindings              |
| [`gurobi-macro`](crates/gurobi-macro) | `crates/gurobi-macro` | `grb-macro`   | Procedural macros (`c!`, `add_var!`, ...) |

## Usage

See the [`gurobi-rs` crate README](crates/gurobi-rs/README.md) for installation, linking against your Gurobi install, feature flags, and examples.

## License

MIT
