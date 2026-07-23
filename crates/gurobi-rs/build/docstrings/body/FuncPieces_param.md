Deprecated since version 13.0: Using function constraints is deprecated since version 13.0. Please use nonlinear
constraints instead.

This parameter sets the strategy used for performing a piecewise-linear approximation of a function constraint. There
are a few options:

- `FuncPieces` >= 2 : Sets the number of pieces; pieces are equal width.
- `FuncPieces` = 1 : Uses a fixed width for each piece; the actual width is provided in the `FuncPieceLength` parameter.
- `FuncPieces` = 0 : Default value; chooses automatically. Currently it uses the relative error approach for the
  approximation, while for version 10.0 or earlier it mainly uses the number of function constraints to set the total
  number of pieces.
- `FuncPieces` = -1 : Bounds the absolute error of the approximation; the error bound is provided in the
  `FuncPieceError` parameter.
- `FuncPieces` = -2 : Bounds the relative error of the approximation; the error bound is provided in the
  `FuncPieceError` parameter.

This parameter only applies to function constraints whose `FuncPieces` attribute has been set to \(0\) .

See the discussion of function constraints for more information.