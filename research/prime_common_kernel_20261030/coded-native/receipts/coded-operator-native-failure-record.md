# Preserved development failures

The prospective protocol predates the first compilation. Strict build001
exited1 on misleading indentation at two adjacentifstatements; source
`coded-operator-native-attempt-001.cpp.txt` and buildlog retained.

Build002 succeeded, but run001 exited134 with `realloc(): invalid old size`;
the raw redirected output is empty because stdout was buffered. The exact
source is `coded-operator-native-attempt-002.cpp.txt`. The retained initial
ASan run exits1 and locates an invalid free within GMP rational multiplication
at cardinal-line20. The code had constructed a rational with a negative
denominator and multiplied it before canonicalization, violating GMP's
canonical-rational arithmetic requirement.
The [official GMP manual](https://gmplib.org/manual/Rational-Number-Functions)
confirms that operands must already be reduced with positive denominators
before rational arithmetic; assignment does not perform this automatically.

Final code canonicalizes each rationalfactor immediately after construction.
Strict build003 and run002 pass. An ASan/UBSan rebuild passes with byte-identical
output to run002. No mathematical theorem was changed to fit the corrected
unit, and no host-contention or performance inference is made.
