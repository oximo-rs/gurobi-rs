We consider this feature a preview in this release. This means that it is fully tested and supported, but will likely
undergo significant changes in subsequent Gurobi technical or major releases, potentially including breaking changes in
API, behavior and packaging.

Specifies the optimality target for nonlinear continuous problems (NLP), including nonconvex QP and QCP models.
Available options are Automatic (-1), Global Optimum (0), and Local Optimum (1). Currently, the automatic choice always
selects the search for a global optimum (0).

For Option 0, the linearized outer-approximation branch-and-bound approach seeks a feasible point with the best possible
objective value and provides an optimality gap.

For Option 1, the nonlinear (NL) barrier algorithm seeks a local optimum, i.e., a feasible point that has the best
possible objective value among the feasible points within a local neighborhood. This alternative typically converges
faster and is able to handle larger instances than a global search, but it does not provide an optimality gap.

Upon success, the NL barrier algorithm concludes with status LOCALLY_OPTIMAL . If it terminates with LOCALLY_INFEASIBLE
, the method found a point that is an infeasible local minimizer of the constraint violation. This is an indication that
the problem might be infeasible.

Note that the search for a local optimum can only be selected if the model has no discrete variables or SOS constraints,
and does not include nondifferentiable functions, such as PWL functions, Max-, Min-, Abs-operators, or the 1- or
Infinity-norm.

The 13.0 preview version of the NL barrier algorithm does not yet support all solution and quality attributes. In
particular, the optimal values of dual variables and some quality attributes (such as scaled violations) are not yet
available.

Only affects continuous nonlinear nonconvex QP, QCP, or NLP models