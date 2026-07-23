The current simplex start vector. If you set `DStart` values for every linear constraint in the model and `PStart`
values for every variable, then simplex will use those values to compute a warm start basis. See Warmstarting Simplex
for Continuous Problems for details. If you’d like to retract a previously specified start, set any `DStart` value to
GRB_UNDEFINED .

If you’d like to provide a feasible starting solution for a MIP model, you should input it using the `Start` attribute.
See Providing a Feasible Solution (MIP `Start`) for details.

Only affects LP models; it will be ignored for QP, QCP, or MIP models.

Note that if you provide a valid starting extreme point, either through `PStart` , `DStart` , or through `VBasis` ,
`CBasis` , then LP presolve will be disabled by default. For models where presolve greatly reduces the problem size,
this might hurt performance. For presolve to be enabled, the parameter LPWarmStart must be set to 2.