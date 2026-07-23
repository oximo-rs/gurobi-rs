Limits the number of MIP nodes explored. Optimization returns with an NODE_LIMIT status if the limit is exceeded. Note
that if multiple threads are used for the optimization, the actual number of explored nodes may be slightly larger than
the set limit.

This parameter is callback settable . It can be changed from within a callback when the where value is PRESOLVED ,
SIMPLEX , MIP , MIPSOL , MIPNODE , BARRIER , or MULTIOBJ (see the Callback Codes section for more information). How to
do that for the different APIs is illustrated here . In case of a remote server, the change of a parameter from within a
callback may not be taken into account immediately.

Only affects mixed integer programming (MIP) models