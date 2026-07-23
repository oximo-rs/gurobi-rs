The barrier solver terminates with a GRB_OPTIMAL status when some solution quality attributes are less than the
specified tolerance. The algorithm considers the relative difference between the primal and dual objective values,
relative primal and dual feasibility, and complementarity. Tightening this tolerance often produces a more accurate
solution, which can sometimes reduce the time spent in crossover. Be aware that such tightening may result in an
increase of barrier iterations and hence computation time spent therein. Loosening it causes the barrier algorithm to
terminate with a less accurate solution, which can be useful when barrier is making very slow progress in later
iterations but increases chances of prolonged runtime in crossover.

This parameter does not affect models with quadratic constraints. For these models use `BarQCPConvTol` .

Barrier only