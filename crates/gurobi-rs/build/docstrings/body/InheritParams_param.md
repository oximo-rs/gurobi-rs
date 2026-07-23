Indicates whether parameters from a main environment should be inherited when working with Concurrent Environments or
Multiobjective Environments . If set to 1, parameters are inherited from the main environment: if their value has not
been set in the supporting environment, their value in the main environment is considered. If set to 0, parameters are
not inherited: only parameters defined on the supporting environment are used. The default value of -1 is equivalent to
0, i.e., parameters are not inherited. This parameter can be set either on the main environment or on the supporting
environment. If it is set on both, its value in the supporting environment overrules the value in the main environment.

When using the command line parameters `MultiObjSettings` or `ConcurrentSettings` , set this parameter to 1 if you want
to provide additional parameters valid for all objectives or concurrent settings, respectively.