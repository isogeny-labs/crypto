# Lock ownership attribution clarification

The frozen report says a peer changed the benchmark lock's ownership. The
observation establishes only that the same inode1088 had different ownership/
mode at later readiness. This task made no ownership/mode change, but the
writer was not identified. Treat attribution to a peer as unverified.
Both final native acquisitions, exclusion, tests and run outcomes are unaffected.
The archived report and underlying readiness records remain unchanged.
