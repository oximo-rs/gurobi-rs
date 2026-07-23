When solving a QCP model, the barrier solver terminates with a GRB_OPTIMAL status when some solution quality attributes
are less than the specified tolerance. The algorithm considers the relative difference between the primal and dual
objective values, relative primal and dual feasibility, and complementarity. Tightening this tolerance may lead to a
more accurate solution, but it may also lead to a failure to converge.

Barrier only