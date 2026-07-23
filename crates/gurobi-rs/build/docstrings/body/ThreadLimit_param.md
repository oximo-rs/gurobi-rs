The `ThreadLimit` parameter is a configuration parameter for an environment which can be used to limit the number of
threads used. This limit is enforced for all optimization calls based on this environment. The default value of 0
implies no limit.

If a thread limit is set, trying to set the `Threads` parameter above this limit will display a warning and not change
the value of the parameter.

You must set the `ThreadLimit` parameter through either a gurobi.env file (using ThreadLimit=limit ) or an empty
environment . Changing the parameter after the environment has been created will result in an error.