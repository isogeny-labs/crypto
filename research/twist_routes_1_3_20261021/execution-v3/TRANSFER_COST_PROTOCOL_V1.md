# Given-map genus-two transfer and generic subgroup solving

Reuse exactly the 24 public toy targets and three explicit degree-three
BHLS maps from the accepted 2026-10-23 transfer record. No new target is
generated, and no NIST logarithm is attempted. Freeze native source and the
96-process plan before executing workers.

For every target run source-group BSGS A/A, source-group BSGS, transferred
subgroup BSGS A/A and transferred subgroup BSGS. Use the same ceiling-sqrt(n)
table/giant-step algorithm in both groups. The transferred worker constructs
the arithmetic chart from the given public map, pulls back the source
generator/target, checks subgroup order, solves in the destination, computes
the norm/inverse-three source point, and verifies the recovered scalar on
the source. The worker receives no old scalar or destination point.

The public BHLS parameter/map discovery, polarized graph identification and
original source-curve selection are outside the worker interval. This is
given-map cold evaluation and solving, not the full geometry-construction
cost. Original base-field balanced arithmetic and faster destination IC are
not substituted by this existing Fp6 reference implementation.

Measure whole-worker Callgrind Ir, one process/thread at a time, with 4 GiB
address space, 32 MiB per file and 60 seconds hard timeout per process.
The arithmetic loops have explicit finite degree/root-factor/BSGS bounds;
the outer timeout also bounds construction. Cap the whole run at 2,400
seconds and 1 GiB raw output, checkpointing each cell. Retain all incomplete
and cap-unstarted records. No admitted wall time or paid host.

Parent admission requires a successful child, a raw single-event Ir total,
source verification and scalar equality to the previously independently
verified public known answer. Generate fresh PARI source-scalar replay and
wrong-scalar controls for every accepted output. Independently reopen raw
hashes/counts and aggregate; require <=5-percent A/A spread separately in
source and transferred groups before comparing them.

The reference is the same generic BSGS algorithm on the original group,
not the strongest matched rho or a fastest external solver. Source-rho cost,
full map discovery, optimized base-field arithmetic and destination IC remain
open obligations. No asymptotic, NIST-security, novelty, fastest-tool or
complete IC claim follows. An observed loss closes only this measured
implementation and interval; it does not prove every transfer route useless.
