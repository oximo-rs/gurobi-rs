The PDHG algorithm will terminate if both of the following conditions are satisfied:

- The relative difference between the primal and dual objective values is less than `PDHGConvTol` , and
- The primal and dual solution values meet the specified feasibility tolerance. This is satisfied if either: the
  absolute residuals of all primal and dual equations are below `PDHGAbsTol` ; or the relative residuals of all primal
  and dual equations are below `PDHGRelTol` .

You can set `PDHGAbsTol` to loosen or tighten the second termination criterion. Note though that relative tolerances
typically lead to earlier termination than absolute tolerances. If you wish to terminate PDHG based solely on absolute
tolerances, you should set `PDHGRelTol` to zero (0).

The first criterion is controlled by `PDHGConvTol` .

PDHG only