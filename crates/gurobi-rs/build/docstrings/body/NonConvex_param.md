Sets the strategy for handling non-convex quadratic objectives or non-convex quadratic constraints. With setting 0, an
error is reported if the original user model contains non-convex quadratic constructs (unless Q matrix linearization, as
controlled by the `PreQLinearize` parameter, removes the non-convexity). With setting 1, an error is reported if non-
convex quadratic constructs could not be discarded or linearized during presolve. With setting 2, non-convex quadratic
problems are solved by translating them into bilinear form and applying spatial branching. The default -1 setting is
currently almost equivalent to 2, except that it takes less care to avoid presolve reductions that might transform a
convex constraint into one that can no longer be detected to be convex, and thus can sometimes perform more presolve
reductions.

Only affects QP, QCP, MIQP, and MIQCP models