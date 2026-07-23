The variable value in a sub-optimal MIP solution. Use parameter SolutionNumber to indicate which alternate solution to
retrieve. Solutions are sorted in order of worsening objective value. Thus, when SolutionNumber is 1, `PoolNX` returns
the second-best solution found. When SolutionNumber is equal to its default value of 0, querying attribute `PoolNX` is
equivalent to querying attribute `X` .

The number of sub-optimal solutions found during the MIP search will depend on the values of a few parameters. The most
important of these are PoolSolutions , PoolSearchMode , and PoolGap . Please consult the section on Solution Pools for a
more detailed discussion of this topic.

When accessing alternate solutions, it is most efficient to query all information about one solution before proceeding
to the next. For more details, refer to our section on Retrieving Solutions .

Only available for MIP models.