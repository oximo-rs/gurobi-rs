The PDHG algorithm will terminate if both of the following conditions are satisfied:

- The relative difference between the primal and dual objective values is less than `PDHGConvTol` , and
- The primal and dual solution values meet the specified feasibility tolerance. This is satisfied if either: the
  absolute residuals of all primal and dual equations are below `PDHGAbsTol` ; or the relative residuals of all primal
  and dual equations are below `PDHGRelTol` .

You can set `PDHGRelTol` to loosen or tighten the second termination criterion.

If you set `PDHGRelTol` to the special value zero (0), then only the absolute feasibility tolerances are considered.
Specifically, primal and dual solutions are considered feasible only if the residuals of all primal and dual equations
are below `PDHGAbsTol` .

The first criterion is controlled by `PDHGConvTol` .

PDHG only