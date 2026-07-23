Quadratic constraint right-hand side. Note that a less-than-or-equal constraint with a `RHS` value of 1e20 or larger, or
a greater-than-or-equal constraint with `RHS` value of -1e20 or smaller, will be treated as always being satisfied.
Equality constraints with very large `RHS` values can lead to numerical issues, so these should be avoided.