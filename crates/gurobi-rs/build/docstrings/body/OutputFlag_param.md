Enables or disables solver output. Use `LogFile` and `LogToConsole` for finer-grain control. Setting `OutputFlag` to 0
is equivalent to setting `LogFile` to "" and `LogToConsole` to 0.

Note that server-side logging is always active for remote jobs run on Gurobi Instant Cloud, Compute Server, or Cluster
Manager. This is not impacted by any user parameter settings.