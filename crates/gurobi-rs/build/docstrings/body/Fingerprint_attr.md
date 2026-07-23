A hash value computed on model data and attributes that can influence the optimization process. The intent is that
models that differ in any meaningful way will have different fingerprints (almost always). The fingerprint of the same
model with the same attributes remains the same across different operating systems and architectures, though the
solution path is expected to differ. The fingerprint is shown in the Header of the log file in hexadecimal format.