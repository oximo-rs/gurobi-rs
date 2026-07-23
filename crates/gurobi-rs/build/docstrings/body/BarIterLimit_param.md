Limits the number of barrier iterations performed. This parameter is rarely used. If you would like barrier to terminate
early, it is almost always better to use the `BarConvTol` parameter instead.

Optimization returns with an ITERATION_LIMIT status if the limit is exceeded.

This parameter is callback settable . It can be changed from within a callback when the where value is PRESOLVED ,
SIMPLEX , MIP , MIPSOL , MIPNODE , BARRIER , or MULTIOBJ (see the Callback Codes section for more information). How to
do that for the different APIs is illustrated here . In case of a remote server, the change of a parameter from within a
callback may not be taken into account immediately.

Barrier only