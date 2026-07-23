By default, the PDHG algorithm runs on the CPU. You can set this parameter to 1 to have PDHG run on the GPU instead, if
your Gurobi build has GPU support and your system has compatible hardware. Note that you must additionally set the
`Method` parameter to GRB_METHOD_PDHG (6) to enable PDHG.

PDHG only

Not available on Compute Server or Instant Cloud