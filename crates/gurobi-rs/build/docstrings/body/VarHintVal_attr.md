A set of user hints. If you know that a variable is likely to take a particular value in high quality solutions of a MIP
model, you can provide that value as a hint. You can also (optionally) provide information about your level of
confidence in a hint with the `VarHintPri` attribute. Refer to Variable Hints (Heuristic Guidance) for more details.

If you wish to leave the hint value for a variable undefined, you can either avoid setting the `VarHintVal` attribute
for that variable, or you can set it to a special undefined value ( GRB_UNDEFINED in C and C++, GRB.UNDEFINED in Java,
.NET, and Python, NA in R or nan in MATLAB).

Only affects MIP models.