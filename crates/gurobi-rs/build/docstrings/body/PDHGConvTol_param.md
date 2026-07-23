The PDHG algorithm will terminate if both of the following conditions are satisfied:

- The relative difference between the primal and dual objective values is less than `PDHGConvTol` , and
- The primal and dual solution values meet the specified feasibility tolerance. This is satisfied if either: the
  absolute residuals of all primal and dual equations are below `PDHGAbsTol` ; or the relative residuals of all primal
  and dual equations are below `PDHGRelTol` .

You can set `PDHGConvTol` to loosen or tighten the first termination criterion.

The second criterion is controlled by `PDHGAbsTol` and `PDHGRelTol` .

PDHG only