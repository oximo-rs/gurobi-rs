Controls the presolve sparsify reduction. This reduction can sometimes significantly reduce the number of non-zero
values in the presolved model. Value 0 shuts off the reduction, while value 1 forces it on for mixed integer programming
(MIP) models and value 2 forces it on for all types of models, including linear programming (LP) models, and MIP
relaxations. The default value of -1 chooses automatically.