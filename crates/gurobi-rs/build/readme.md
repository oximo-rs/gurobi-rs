# The Build Script

This build script is responsible for generating the enums containing Gurobi attributes and parameters.
The inputs are `attrs.csv` and `params.csv` in this directory. The generated catalog is part of
the compatibility surface: every catalog update must be version-gated and checked against the
installed Gurobi library for the corresponding Cargo feature.

## Supported version matrix

The public feature-to-library mapping is:

| Gurobi major version | Cargo feature | Catalog entries                        |
| -------------------- | ------------- | -------------------------------------- |
| 10                   | `gurobi10`    | common entries                         |
| 11                   | `gurobi11`    | common entries                         |
| 12                   | `gurobi12`    | common entries                         |
| 13                   | `gurobi13`    | common entries plus `gurobi13` entries |

An empty `feature` field means that an entry is generated for every supported version. A
`gurobi13` value adds an entry only when the `gurobi13` Cargo feature is selected. Keep a new
attribute or parameter out of the common catalog until it is present in every supported library.
The generator currently accepts only the empty value and `gurobi13`; adding another gate requires
updating `build/main.rs`, the feature matrix, and the checks below together.

## Running the Python tooling

Dependencies are managed with [uv](https://docs.astral.sh/uv/) via `pyproject.toml`. Run the scripts with:

```sh
uv run scrape-docs.py
uv run check-for-missing.py
```

For Gurobi 13, select the current Sphinx documentation scraper explicitly:

```sh
uv run scrape-docs.py --gurobi13
uv run check-for-missing.py --gurobi13 --strict
```

The legacy scraper can also be run in strict mode when checking the common catalog:

```sh
uv run check-for-missing.py --strict
```

The strict checks compare the CSV catalog with the selected online reference manual and return a
failure when an entry is missing or extra. Review any difference against the intended version
gate before editing the CSV files.

`attrs.csv` has the following format:

```csv
attr,dtype,otype,feature
```

where `attr` is the Gurobi attribute name (case sensitive), `dtype` is the datatype which governs the marker trait used for blanket impls.
The allowed values for `dtype` are described below:

| `dtype`  | Description                                      |
| -------- | ------------------------------------------------ |
| `dbl`    | `f64`,  marker trait `DoubleAttr`                |
| `int`    | `i32`,  marker trait `IntAttr`                   |
| `chr`    | `c_char`, marker trait `CharAttr`                |
| `str`    | `String`,  marker trait `StrAttr`                |
| `custom` | Custom datatype, no marker traits will be added. |

The `otype` is the object type to which this attribute belongs (`Model`, `Var`, `Constr`, etc).
The allowed values for `otype` are listed below. The optional `feature` field restricts an entry to
a Cargo feature. Leave it empty for attributes available in every supported version; use
`gurobi13` only for attributes introduced by Gurobi 13.

| `otype`   | Description                           |
| --------- | ------------------------------------- |
| `model`   | no marker trait                       |
| `var`     | marker trait `ObjAttr<Obj=Var>`       |
| `constr`  | marker trait `ObjAttr<Obj=Constr>`    |
| `gconstr` | marker trait `ObjAttr<Obj=GenConstr>` |
| `qconstr` | marker trait `ObjAttr<Obj=QConstr>`   |
| `sos`     | marker trait `ObjAttr<Obj=SOS>`       |

This build script will group attributes by `otype` and `dtype`, and generate enums as needed.  For example,
for `otype = "constr"` and `dtype = "str"` the following code is generated (in  `src/attribute/attr_enums.rs`):

```rust
/// String Gurobi attributes for [`Constr`](crate::Constr) objects.
/// 
/// This enum contains the following Gurobi attributes:
///  - [`CTag`](https://docs.gurobi.com/projects/optimizer/en/current/reference/attributes/constraintlinear.html#attr-CTag)
///  - [`ConstrName`](https://docs.gurobi.com/projects/optimizer/en/current/reference/attributes/constraintlinear.html#attr-ConstrName)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, FromCStr, AsCStr)]
pub enum ConstrStrAttr {
    CTag,
    ConstrName,
}

impl StrAttr for ConstrStrAttr {
}

impl ObjAttr for ConstrStrAttr {
    type Obj = Constr;
}
```

Note the two marker traits.  The latter would not be implemented if `otype = "model"`

`params.csv` has the format similar format,

```csv
param,dtype,feature
```

where `param` is the Gurobi parameter name (case sensitive), `dtype` has the same meaning as above, and the optional
`feature` field has the same version-gating behavior as for attributes.
Note that there are currently no `char` parameters implemented in Gurobi.

The parameters always relate to an `Env`, so marker traits are not needed.  Below is example output in `src/parameter/param_enums.rs`
for `dtype = "str"`

```rust
/// String Gurobi parameters.
/// 
/// This enum contains the following Gurobi parameters:
///  - [`LogFile`](https://docs.gurobi.com/projects/optimizer/en/current/reference/parameters.html#param-LogFile)
///  - [`NodefileDir`](https://docs.gurobi.com/projects/optimizer/en/current/reference/parameters.html#param-NodefileDir)
///  - [`ResultFile`](https://docs.gurobi.com/projects/optimizer/en/current/reference/parameters.html#param-ResultFile)
///  - [`WorkerPool`](https://docs.gurobi.com/projects/optimizer/en/current/reference/parameters.html#param-WorkerPool)
///  - [`WorkerPassword`](https://docs.gurobi.com/projects/optimizer/en/current/reference/parameters.html#param-WorkerPassword)
///  - [`Dummy`](https://docs.gurobi.com/projects/optimizer/en/current/reference/parameters.html#param-Dummy)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, FromCStr, AsCStr)]
pub enum StrParam {
    LogFile,
    NodefileDir,
    ResultFile,
    WorkerPool,
    WorkerPassword,
    Dummy,
}
```

Finally, in both cases, the enums are added to a module called `enum_exports`, and the variants of the enums are added to a module called `variant_exports`:

```rust
pub(super) mod enum_exports {
  pub use super::{ModelDoubleAttr, ModelIntAttr, ...};
}

pub mod variant_exports {
  pub use super::ModelDoubleAttr::*;
  pub use super::ModelIntAttr::*;
  ...
}
```

## Release check

From the repository root, run the dependency-free catalog validation and compile the generated
Rust code for every supported feature:

```sh
uv run crates/gurobi-rs/build/check-catalog.py
```

Each command must resolve the shared library for the selected major version through
`GUROBI_HOME`, `GUROBI_LIBNAME`, and the platform library path. Do not use a different installed
major version to validate a feature. A catalog refresh is complete only after the generated-code
check, the strict reference-manual check, and the corresponding installed-library checks pass.
