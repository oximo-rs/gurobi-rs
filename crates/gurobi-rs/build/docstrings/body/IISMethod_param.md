Chooses the IIS method to use. To compute an IIS for an LP, it is sufficient to solve an LP with dimensions similar to
the dual of the original model. If the solve time for that LP is excessive, setting the `IISMethod` parameter to 1 may
offer a faster alternative; other settings do not alter the default approach for infeasible LPs. For MIPs, filtering of
constraints and variables is required, which involves solving a series of related MIP subproblems. Methods 0-2 all use
filtering techniques. `Method` 0 is often faster than method 1, but may produce a larger IIS. `Method` 2 ignores the
bound constraints. It therefore tends to be faster than methods 0-1, but will fail if these bounds are necessary to make
the problem infeasible. `Method` 3 will return the IIS for the LP relaxation of a MIP model if the relaxation is
infeasible, even though the result may not be minimal when integrality constraints are included. The default value of -1
chooses automatically.