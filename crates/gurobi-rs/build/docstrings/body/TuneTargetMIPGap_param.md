A target gap to be reached. As soon as the tuner has found parameter settings that allow Gurobi to reach the target gap
for the given model(s), it stops trying to improve parameter settings further. Instead, the tuner switches into the
cleanup phase (see `TuneCleanup` parameter).

This parameter only applies if no other secondary tuning criterion than `MIPGap` is set, i.e., `TuneCriterion` is at its
default value or 1.