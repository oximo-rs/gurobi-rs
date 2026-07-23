This attribute is used to query the objective bound for optimization pass \(p\) of a multi-objective solve. You set
\(p\) using the ObjPassNumber parameter. If that parameter is set to -1 (its default value), the optimization pass
selected is the one in which objective \(k\) was processed. You set \(k\) using the ObjNumber parameter.

Note that this attribute is only available for the objectives that were actually processed. When an objective was not
processed (e.g. a time limit was reached during some prior optimization pass), an error is raised.