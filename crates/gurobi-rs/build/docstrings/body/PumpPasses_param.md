Number of passes of the feasibility pump heuristic.

This heuristic is quite expensive, and generally produces poor quality solutions. You should generally only use it if
other means, including exploration of the tree with default settings, fail to produce a feasible solution.

This parameter is callback settable . It can be changed from within a callback when the where value is PRESOLVED ,
SIMPLEX , MIP , MIPSOL , MIPNODE , BARRIER , or MULTIOBJ (see the Callback Codes section for more information). How to
do that for the different APIs is illustrated here . In case of a remote server, the change of a parameter from within a
callback may not be taken into account immediately.

Only affects mixed integer programming (MIP) models