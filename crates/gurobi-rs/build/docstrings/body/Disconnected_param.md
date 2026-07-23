A MIP or an LP model can sometimes be made up of multiple, completely independent sub-models. This parameter controls
how aggressively we try to exploit this structure. A value of 0 ignores this structure entirely, while larger values try
more aggressive approaches. The default value of -1 chooses automatically.

Only affects mixed integer programming (MIP) models