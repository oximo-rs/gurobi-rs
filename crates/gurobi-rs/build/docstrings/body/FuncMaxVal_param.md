Deprecated since version 13.0: Using function constraints is deprecated since version 13.0. Please use nonlinear
constraints instead.

Very large values in piecewise-linear approximations can cause numerical issues. This parameter limits the bounds on the
variables that participate in function constraints approximated by a piecewise-linear function. Specifically, any bound
larger than `FuncMaxVal` (in absolute value) on the variables participating in such a function constraint will be
truncated.

If the `FuncNonlinear` attribute of the constraint is set to 1, or if it is set to -1 and the global `FuncNonlinear`
parameter is set to 1, the function constraint is not approximated by a piecewise-linear function and the `FuncMaxVal`
parameter does not apply.