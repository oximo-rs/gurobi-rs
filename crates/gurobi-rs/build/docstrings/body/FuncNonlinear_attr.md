Deprecated since version 13.0: Using function constraints is deprecated since version 13.0. Please use nonlinear
constraints instead.

This attribute controls whether the particular general function constraint is replaced with a static piecewise-linear
approximation (0), or is handled inside the branch-and-bound tree using a dynamic outer-approximation approach (1). The
default value (-1) means that the constraint handling will be controlled by the `FuncNonlinear` parameter .

See the discussion of function constraints for more information.