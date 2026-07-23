Sets a limit on the amount of diagonal perturbation that the optimizer is allowed to perform on a Q matrix in order to
correct minor PSD violations. If a larger perturbation is required, the optimizer will terminate with a Q_NOT_PSD error.

Only affects QP, QCP, MIQP, and MIQCP models