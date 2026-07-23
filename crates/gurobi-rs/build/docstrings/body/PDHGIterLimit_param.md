Limits the number of PDHG iterations performed.

The PDHG algorithm will terminate if this limit is exceeded. If crossover is enabled, it will start from the final PDHG
iterate. If crossover is disabled, optimization will return with an ITERATION_LIMIT status.

PDHG only