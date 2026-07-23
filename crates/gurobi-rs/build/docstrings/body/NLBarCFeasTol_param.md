We consider this feature a preview in this release. This means that it is fully tested and supported, but will likely
undergo significant changes in subsequent Gurobi technical or major releases, potentially including breaking changes in
API, behavior and packaging.

For the NL barrier algorithm, the complementarity error must be smaller than `NLBarCFeasTol` in order for a model to be
declared locally optimal. Due to problem transformations like presolve or internal scaling, the returned solution’s
residuals may deviate from those observed by the algorithm.

Only affects the NL barrier algorithm