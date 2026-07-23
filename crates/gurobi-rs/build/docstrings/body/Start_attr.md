The current MIP start vector. The MIP solver will attempt to build an initial solution from this vector when it is
available. Note that the start can be partially populated – the MIP solver will attempt to fill in values for missing
start values. If you wish to leave the start value for a variable undefined, you can either avoid setting the `Start`
attribute for that variable, or you can set it to a special undefined value ( GRB_UNDEFINED in C and C++, or
GRB.UNDEFINED in Java, .NET, and Python). Refer to Providing a Feasible Solution (MIP `Start`) for more details.

Only affects MIP models.