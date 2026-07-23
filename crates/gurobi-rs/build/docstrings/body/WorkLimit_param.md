Limits the total work expended (in work units). Optimization returns with a WORK_LIMIT status if the limit is exceeded.

In contrast to the `TimeLimit` , work limits are deterministic. This means that on the same hardware and with the same
parameter and attribute settings, a work limit will stop the optimization of a given model at the exact same point every
time. On a CPU using a single thread, one work unit corresponds very roughly to one second, but this greatly depends on
the hardware on which Gurobi is running and the model that is being solved. When using a GPU algorithm, the relationship
between work units and elapsed time can be very different: one work unit may correspond to much less than one second
(e.g. 0.01 seconds).

Note that optimization may not stop immediately upon hitting the work limit. It will stop when the optimization is next
in a deterministic state, and it will then perform the required additional computations of the attributes associated
with the terminated optimization. As a result, the Work attribute may be larger than the specified `WorkLimit` upon
completion, and repeating the optimization with a `WorkLimit` set to the Work attribute of the stopped optimization may
result in additional computations and a larger attribute value.

This parameter is callback settable . It can be changed from within a callback when the where value is PRESOLVED ,
SIMPLEX , MIP , MIPSOL , MIPNODE , BARRIER , or MULTIOBJ (see the Callback Codes section for more information). How to
do that for the different APIs is illustrated here . In case of a remote server, the change of a parameter from within a
callback may not be taken into account immediately.