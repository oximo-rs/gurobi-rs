Index of constraint with the largest violation in a sub-optimal MIP solution. Use parameter SolutionNumber to indicate
which alternate solution to retrieve.

The constraint order is linear, quadratic, SOS and general. Assume there are \(l\) linear, \(q\) quadratic, \(s\) SOS
and \(g\) general constraints and the index \(i\) is between \(l+q+s\) and \(l+q+s+g\) . Then, the general constraint
with index \(i-l-q-s\) has the biggest violation.

Please consult the section on Solution Pools for a more detailed discussion of this topic.

Only available for MIP models.