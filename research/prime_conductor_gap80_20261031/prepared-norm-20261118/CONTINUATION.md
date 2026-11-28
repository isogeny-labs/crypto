# Reference-reuse qualification after the actual cap

Attempt A reaches the 300-second cap and exits 124 during the prepared
version of the first degree-10009 PARI expansion. Native primitive controls
and default fixture comparison passed. Its baseline and candidate-default
reference expansions both finished and compare byte-for-byte; their raw
outputs are retained. No complete pipeline qualifier pass is asserted for A.

The fixed acceptance originally repeated direct PARI expansion for every
schedule/backend. Attempt B explicitly changes this validation execution:
reuse the completed degree-10009 independent-PARI result for the identical
source/kernel/point panel across its block/window/coordinate schedules.
Every new output must match its complete input/codomain/images/coverage
projection. Keep each command's pari_direct_reference=false when no fresh
PARI expansion was called; do not relabel it true. Other small-degree cases,
including the 96-bit field, still call fresh PARI.

Candidate source/binary and mathematical acceptance stay unchanged. Reuse
the already passed primitive/default-fixture outputs with exact hashes;
run all old/new-default byte and prepared semantic comparisons freshly.
The new qualifier has the same 300-second cumulative development cap,
retains A's timeout, and makes no runtime or memory claim. This is not a
passing fresh-PARI expansion at every schedule or an independent signed
execution. Original 80-bit construction and timing criteria stay open.
