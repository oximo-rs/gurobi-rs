# gurobi-rs ![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/oximo-rs/gurobi-rs?sort=semver) ![](https://img.shields.io/crates/v/gurobi-rs.svg) ![](https://img.shields.io/docsrs/gurobi-rs)

This crate provides Rust bindings for Gurobi Optimizer.  It currently requires Gurobi 9.0 or higher.

This library started as fork of the [`gurobi`](https://github.com/ubnt-intrepid/rust-gurobi) which appears to be no longer maintained.  It has since undergone a number of fundamental API changes.

This crate supports Gurobi 10, 11, 12 and 13.

## Installing and Linking

Before using this crate, you should install Gurobi and obtain a [license](http://www.gurobi.com/downloads/licenses/license-center).

### Feature flags

The `gurobi-rs` crate requires one of the following feature flags to be set:

- `gurobi13`
- `gurobi12`
- `gurobi11`
- `gurobi10`

The flag should match the major version of Gurobi, for example (in Cargo.toml):

```toml
gurobi-rs = {..., features = ['gurobi13']}
```

for Gurobi 13.X.

If multiple feature flags are set, the highest version one is used, i.e. setting `gurobi13` and `gurobi10` is equivalent to only setting `gurobi13`.

### Building

In this section, it is assumed Gurobi is installed at `/opt/gurobi/linux64`.

It is recommended you use the environment variables for your system's linker to ensure Gurobi can be found.
For example, on Linux systems this can be done by appending the path to the `lib` subfolder of the gurobi installation to `LIBRARY_PATH`.   For example, put

```base
export LIBRARY_PATH="LIBRARY_PATH:/opt/gurobi/linux64/lib"
```

in your `~/.profile` file.  You can also set this in a `PROJECT/.cargo/config.toml` file on per project basis (see the `[env]` [section](https://doc.rust-lang.org/cargo/reference/config.html)).

The other option is to set the environment variable `GUROBI_HOME` set to the installation path of Gurobi
(like eg `/opt/gurobi95/linux64`).  

The Gurobi shared library will have the major and minor version of Gurobi in the library name.  For example, Gurobi 11.0.* will have a shared library file `libgurobi110.so`.  The `guro-sys` crate, which this crate depends on, will link against with `-lgurobi110`.  On Linux, we make an guess for the library name based on `GUROBI_HOME`.  If this guess is incorrect (or `GUROBI_HOME` is not set, or you are on Windows), you will need to set the `GUROBI_LIBNAME` environment variable.  For example, suppose you have the `LIBRARY_PATH` set to `/opt/gurobi1003/linux64/lib` (which contains `libgurobi100.so`), and `GUROBI_HOME` is **not set**.  Then, you would set `GUROBI_LIBNAME=gurobi100`, so that the correct `-lgurobi100` flag is emitted during compilation.



### Running
When running the compiled binaries or running tests, you may get
```bash
error while loading shared libraries: libgurobi95.so: cannot open shared object file: No such file or directory
```
In this case, you need to set the `LD_LIBRARY_PATH` (on Windows I believe this is called `PATH`) environment variable or embed the path to `libgurobi95.so` in the [rpath](https://en.wikipedia.org/wiki/Rpath) during compilation by supplying the appropriate linker flags in `RUSTFLAGS`.

For the example below, suppose Gurobi is in the path `/opt/gurobi/linux64/lib/libgurobi95.so`.  You set `LD_LIBRARY_PATH` in the same manner as the `LIBRARY_PATH` variable, in your `~/.profile`:

```base
export LD_LIBRARY_PATH="LD_LIBRARY_PATH:/opt/gurobi/linux64/lib"
```

## Documentation
Docs can be found on [docs.rs](https://docs.rs/gurobi-rs/)

## License
This software is released under the [MIT license](LICENSE).
