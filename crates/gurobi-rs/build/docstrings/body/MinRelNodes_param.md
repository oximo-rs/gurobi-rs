Number of nodes to explore in the minimum relaxation heuristic.

This heuristic is quite expensive, and generally produces poor quality solutions. You should generally only use it if
other means, including exploration of the tree with default settings, fail to produce a feasible solution.

The default value automatically chooses whether to apply the heuristic. It will only rarely choose to do so.

Only affects mixed integer programming (MIP) models