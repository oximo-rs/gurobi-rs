Controls how Gurobi deals with indicator constraints when creating the fixed model (e.g. GRBconverttofixed and
GRBfixmodel in C, or Model.convertToFixed and Model.fixed in Python).

If set to 0 (the default), then an indicator constraint is discarded if its premise is false (i.e., if the associated
binary indicator variable is fixed to a value that does not satisfy the premise condition) in the solution or MIP start
that is associated to the fixed model. On the other hand, if the premise of the indicator is true, then the implied
linear constraint is added as a regular linear constraint to the fixed model.

Let’s consider the case where `FixVarsInIndicators` is set to 0. If there is an indicator constraint

\(z = 0 \rightarrow ax \leq b\)

in the model and variable \(z\) has value 1 in the solution for which the fixed model is created, then the indicator
constraint is not active and it is therefore discarded from the fixed model. If the indicator variable \(z\) has value
0, then the indicator is active and the linear constraint \(ax \leq b\) is added to the fixed model.

If the `FixVarsInIndicators` parameter is set to 1, then all variables (including continuous variables) in the indicator
constraint are fixed to their solution value, independent of whether the indicator is active or not in the associated
solution.

Only affects mixed integer programming (MIP) models