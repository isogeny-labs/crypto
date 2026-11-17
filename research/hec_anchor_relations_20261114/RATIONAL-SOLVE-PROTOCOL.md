# Frozen normalized rational single-target integration gate

This follows primitive rank gate9e47e6b6f2and its synthetic-input correction.
It is a correctness integration gate, not a timing comparison. No experiment
clock from this gate may be used as a performance claim.

Implement a solver taking only curve,D1,D2,ell,seed,requestedH,d,gate. The
known scalar appears only in the test harness after target construction.
Reuse the existing table root oracle, descent, signed-row canonicalization,
DenseForest and anchor Echelon. Keep the existing compact/control solvers'
operations unchanged; exporting shared helpers does not change their algorithm.

Build tables and the full factor-base catalogue cold, descend D1/D2, and seed
the anchor set with every place in both descent rows. EffectiveH is max of
requestedH,target-support count,M=g+2d, bounded by available FB places. Record
both requested and effective H rather than silently treating the rank screen's
first-H anchors as a single-target construction. Fill other anchors using the
same seeded partial shuffle as the original anchor solver. Do not use a scalar
or subgroup logarithm to select an anchor or construct a relation.

Sample unique sorted M-tuples; visit all integer-ordered signs with first sign+,
constructing a whole Walsh batch before consuming its prefix. For each nonzero
pair, primitive-normalize with canceled-anchor removal; monic-normalize Q,
factor, orient, merge, deduplicate moduloell, and admit at most2off-anchor nodes.
Keep cold work for unused batch tails in constructed count and method total.
Insert pure rows incrementally with the two descent/target equations. Return
only after k*D1=D2. Record setup/relation/forest/solve/total clocks for accounting
development only; full total includes final verification. Gate=true verifies
every admitted raw principal identity and every projected anchor identity.
Projection for these extra checks is harness work; gate=false must avoid it.

Caps:1000000consumed functions;10000duplicate-tuple retries before a terminal
retry cap; exact tuple-family exhaustion detected with bounded binomial count.
Preserve terminal statuses solved/exhausted/draw_cap/tuple_retry_cap/descent_cap/
unavailable. No replacing failed inputs. Tests use d2 and requestedH in
{g+6,4g+12} on deterministic pick_instance fixtures (g,p)=(2,101),(3,61),(4,31),
two scalars k=113,136 and seeds20260916,20260917 respectively:12attempted cases.
All must recover the harness scalar and pass raw/projected gates. Print every
case's requested/effectiveH, constructed/consumed functions, counts and status.

Unit-test budget: one local native process, <=60seconds per invocation,
<=256MiB target memory, no cloud spend. Before measurements, freeze complete
same-target controls, new curves/seeds, timing order, cold boundaries, caps and
acceptance criteria separately. Passing this gate neither proves performance
nor mathematical novelty. Prior art/KB receipt: PRIMITIVE-PROTOCOL.md.
