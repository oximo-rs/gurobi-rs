Deprecated since version 13.0: Using function constraints is deprecated since version 13.0. Please use nonlinear
constraints instead.

This parameter controls whether general function constraints with their `FuncNonlinear` attribute set to -1 are replaced
with a static piecewise-linear approximation (0), or handled inside the branch-and-bound tree using a dynamic outer-
approximation approach (1).

See the discussion of function constraints for more information.