We consider this feature a preview in this release. This means that it is fully tested and supported, but will likely
undergo significant changes in subsequent Gurobi technical or major releases, potentially including breaking changes in
API, behavior and packaging.

Limits the number of barrier NL iterations performed.

Optimization returns with an ITERATION_LIMIT status if the limit is exceeded.

Only affects the NL barrier algorithm