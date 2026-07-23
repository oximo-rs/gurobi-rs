This parameter limits the total work (in work units) spent on completing a partial MIP start.

In contrast to the `StartTimeLimit` , work limits are deterministic. This means that on the same hardware and with the
same parameter and attribute settings, a work limit will stop the optimization of a given model at the exact same point
every time. One work unit corresponds very roughly to one second on a single thread, but this greatly depends on the
hardware on which Gurobi is running and the model that is being solved.

Only affects mixed integer programming (MIP) models