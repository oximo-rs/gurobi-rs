Primal start vector.

For LP models, this defines the current simplex start vector. If you set `PStart` values for every variable in the model
and `DStart` values for every constraint, then simplex will use those values to compute a warm start basis, as described
in Warmstarting Simplex for Continuous Problems . If you’d like to retract a previously specified start, set any
`PStart` value to GRB_UNDEFINED .

For the NL barrier method, the value of `PStart` determines the starting point for a particular variable if it is set to
a value different from GRB_UNDEFINED . The NL barrier method is the main algorithm used to solve continuous nonconvex
models when OptimalityTarget is set to 1. It is also used automatically as a primal heuristic when solving nonconvex
(MI)QCP and (MI)NLP models to global optimality.

For other problem types, the `PStart` values will be ignored.

If you’d like to provide a feasible starting solution for MIP, non-convex (MI)QCP, or (MI)NLP models, you should input
it using the `Start` attribute, as described in Providing a Feasible Solution (MIP `Start`) .

Note that if you provide a valid starting extreme point, either through `PStart` , `DStart` , or through `VBasis` ,
`CBasis` , then LP presolve will be disabled by default. For models where presolve greatly reduces the problem size,
this might hurt performance. For presolve to be enabled, the parameter LPWarmStart must be set to 2.