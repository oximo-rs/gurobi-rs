Limits the total amount of memory (in GB, i.e., \(10^9\) bytes) available to Gurobi. If more is needed, Gurobi will fail
with an OUT_OF_MEMORY error.

Note that it is not possible to retrieve solution information after an error termination. Thus, the behavior of this
parameter is different from that of other termination criteria like `SoftMemLimit` , `TimeLimit` , or `NodeLimit` ,
where the solver will terminate with a Status Code and solution information will still be available.

One advantage of using this parameter rather than the similar `SoftMemLimit` is that `MemLimit` is checked after every
memory allocation, so Gurobi will terminate at precisely the point where the limit is exceeded.

Note that allocated memory is tracked across all models within a Gurobi environment. If you create multiple models in
one environment, these additional models will count towards overall memory consumption.

Memory usage is also tracked across all threads. One consequence of this is that termination may be non-deterministic
for multi-threaded runs.