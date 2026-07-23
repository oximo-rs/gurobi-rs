The NLP heuristic uses a non-linear barrier solver to find feasible solutions to nonconvex quadratic and nonlinear
models during a global optimization solve. It often helps to find solutions quicker, but in some cases it can consume
significant runtime without producing a solution. A value of 0 disables the heuristic completely, while larger values
call the heuristic more and more aggressively during the optimization process. The default -1 value chooses
automatically.

Only affects models with nonconvex quadratic or nonlinear expressions in the objective or constraints