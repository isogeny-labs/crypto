# Pairing output amendment, version 2

Preserve V1's input, GP script, empty stdout/stderr, zero GP exit and failed
native result status. GP's single-line function definition consumed the print
and quit statements into the function body, so it emitted no replayable result.
The first attempt is unresolved and supplies no pairing-validation claim.

Put the jstr function definition on its own line, freeze the source amendment,
and repeat the exact V1 fixture, ten public target points, algorithms, resource
limits and checks in a fresh output directory. No other mathematical or
acceptance condition changes. Keep every output and require the independent
logs and native source-group replay before accepting the positive control.
