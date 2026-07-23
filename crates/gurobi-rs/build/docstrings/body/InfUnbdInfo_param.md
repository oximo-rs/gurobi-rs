Set this parameter if you want to query the unbounded ray for unbounded models (through the UnbdRay attribute), or the
infeasibility proof for infeasible models (through the FarkasDual and FarkasProof attributes).

When this parameter is set additional information will be computed when a model is determined to be infeasible or
unbounded, and a simplex basis is available (from simplex or crossover). Note that if a model is determined to be
infeasible or unbounded when solving with barrier, prior to crossover, then this additional information will not be
available.

Note that if a model is found to be either infeasible or unbounded, and you simply want to know which one it is, you
should use the `DualReductions` parameter instead. It performs much less additional computation.

Only affects linear programming (LP) models