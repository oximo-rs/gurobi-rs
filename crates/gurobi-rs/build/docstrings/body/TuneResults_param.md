The tuning tool often finds multiple parameter sets that improve over the baseline settings. This parameter controls how
many of these sets should be retained when tuning is complete. A non-negative value indicates how many sets should be
retained. The default value (-1) retains the efficient frontier of parameter sets. That is, it retains the best set for
one changed parameter, the best for two changed parameters, etc. Sets that aren’t on the efficient frontier are
discarded. If you are interested in all the sets, use value -2 for the parameter.

Note that the first set in the results is always the set of parameters which was used for the first solve, the baseline
settings. This set serves as the base for any improvement. So if you are interested in the best tuned set of parameters
you need to request at least 2 tune results. The first one (with index 0) will be the baseline setting and the second
one (with index 1) will be the best set found during tuning.