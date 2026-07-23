When querying attribute PoolNX , ObjNVal , PoolNObjVal or any of the solution pool quality attributes (e.g., PoolNMaxVio
) to retrieve information about an alternate MIP solution, this parameter determines for which alternate solution the
data are retrieved. The value of this parameter should be less than the value of the SolCount attribute.

When accessing alternate solutions, it is most efficient to query all information about one solution before proceeding
to the next. For more details, refer to our section on Retrieving Solutions .

Only affects mixed integer programming (MIP) models