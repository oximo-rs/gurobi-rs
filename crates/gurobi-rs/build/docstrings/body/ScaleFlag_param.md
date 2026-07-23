Controls model scaling. By default, the rows and columns of the model are scaled in order to improve the numerical
properties of the constraint matrix. The scaling is removed before the final solution is returned. Scaling typically
reduces solution times, but it may lead to larger constraint violations in the original, unscaled model. Turning off
scaling ( ScaleFlag=0 ) can sometimes produce smaller constraint violations. Choosing a different scaling option can
sometimes improve performance for particularly numerically difficult models. Using geometric mean scaling ( ScaleFlag=2
) is especially well suited for models with a wide range of coefficients in the constraint matrix rows or columns.
Settings 1 and 3 are not as directly connected to any specific model characteristics, so experimentation with both
settings may be needed to assess performance impact.