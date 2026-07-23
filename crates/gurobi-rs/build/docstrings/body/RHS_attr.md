Constraint right-hand side.

Note that a less-than-or-equal constraint with a `RHS` value of 1e20 or larger, or a greater-than-or-equal constraint
with `RHS` value of -1e20 or smaller, will be treated as always being satisfied. However, if your intention is to
disable an inequality constraint, we recommend removing it from the model, rather than setting a large `RHS` value so
that it is always satisfied. If the constraint must remain in the model, you should use GRB_INFINITY in C, C++ and
GRB.INFINITY in C#, Java, and Python as a suitably large constant for the right-hand side.

Equality constraints with large `RHS` values can lead to numerical issues, so these should be avoided.