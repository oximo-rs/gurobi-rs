Enables or disables quad precision computation in simplex. The -1 default setting allows the algorithm to decide. `Quad`
precision can sometimes help solve numerically challenging models, but it can also significantly increase runtime.
`Quad` precision is only available on processors that support quadruple precision, e.g., common Intel processors. On
other processors, the parameter has no effect.