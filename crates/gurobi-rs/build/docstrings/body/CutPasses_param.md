A non-negative value indicates the maximum number of cutting plane passes performed during root cut generation. The
default value chooses the number of cut passes automatically.

In addition to cutting plane separation, each cut pass also applies heuristics and node probing and also may launch
parallel root helper threads. So even when the `Cuts` parameter is set to 0, the cut loop will apply probing, heuristics
and parallel root helpers in a single cut loop iteration.

You should experiment with different values of this parameter if you notice the MIP solver spending significant time on
root cut passes that have little impact on the objective bound.

Only affects mixed integer programming (MIP) models