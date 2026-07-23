Limits the total time expended (in seconds). Optimization returns with a TIME_LIMIT status if the limit is exceeded.

Note that optimization may not stop immediately upon hitting the time limit. It will stop after performing the required
additional computations of the attributes associated with the terminated optimization. As a result, the Runtime
attribute may be larger than the specified `TimeLimit` upon completion, and repeating the optimization with a
`TimeLimit` set to the Runtime attribute of the stopped optimization may result in additional computations and a larger
attribute value.

This parameter is callback settable . It can be changed from within a callback when the where value is PRESOLVED ,
SIMPLEX , MIP , MIPSOL , MIPNODE , BARRIER , or MULTIOBJ (see the Callback Codes section for more information). How to
do that for the different APIs is illustrated here . In case of a remote server, the change of a parameter from within a
callback may not be taken into account immediately.