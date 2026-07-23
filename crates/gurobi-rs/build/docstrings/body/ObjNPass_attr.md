This attribute is used to query the index of the multiobjective optimization pass in which objective \(k\) was
processed. You set \(k\) using the ObjNumber parameter.

If the objective was not processed (e.g. a time limit was reached during some prior optimization pass), querying this
attribute returns -1.