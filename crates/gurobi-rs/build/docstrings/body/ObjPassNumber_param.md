After a multi-objective model has been solved, this parameter selects the index of the optimization pass for which you
want to query results such as ObjPassNIterCount , ObjPassNMipGap , etc. The value of this parameter should be strictly
less than the value of the NumObjPasses attribute (which captures the number of optimization passes processed in the
last run). If the parameter value is set to -1 (default), the optimization pass selected is the one in which objective
\(k\) was processed. You set \(k\) using the `ObjNumber` parameter.

Please refer to the discussion of Multiple Objectives for more information on the use of alternative objectives.