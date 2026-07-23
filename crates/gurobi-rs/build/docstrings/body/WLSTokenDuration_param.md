When using a WLS license, this parameter can be used to adjust the lifespan (in minutes) of a token. A token enables
Gurobi to run on that client for the life of the token. Gurobi will automatically request a new token if the current one
expires, but it won’t notify the WLS server if it completes its work before the current token expires. A shorter
lifespan is better for short-lived usage. A longer lifespan is better for environments where the network connection to
the WLS server is unreliable.

The default value of 0 means ‘automatic’, and is currently equal to 5 minutes. This value may change in the future. The
WLS server will cap the chosen value automatically to be at least 5 minutes and no more than 60 minutes. This behavior
may change in the future as well.