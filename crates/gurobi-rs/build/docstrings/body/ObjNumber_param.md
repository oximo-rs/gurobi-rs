When working with multiple objectives, this parameter selects the index of the objective you want to work with. When you
query or modify an attribute associated with multiple objectives ( ObjN , ObjNWeight , etc.), the `ObjNumber` parameter
determines which objective function is actually affected. When you query an attribute associated with multi-objective
passes ( ObjPassNStatus , ObjPassNObjVal , etc.) and the parameter `ObjPassNumber` is set to -1, the `ObjNumber`
parameter determines for which pass the value is returned. The value of this parameter should be less than the value of
the NumObj attribute (which captures the number of objectives in the model).

Please refer to the discussion of Multiple Objectives for more information on the use of alternative objectives.