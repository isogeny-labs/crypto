# Exploratory attribution and limits

The two predeclared symbolled-source profiles completed exit0 and returned the
same complete ordered relation set as the historical scalar oracle. Native
GNU gprofng function tables include `__umodti3` among the largest observed
self-sample entries and polynomial division/multiplication among callers.

Both native experiment headers warn: **Collection interval timer period was
changed (1000 -> 0); profile data may be unreliable.** The function tables
account0.054/0.089s while the native process statistics record multi-second CPU
use. These are undercovered/provisional profiles, not precise time fractions,
reliable hotspot rankings, or speed measurements. Raw native tables/headers
and per-worker outputs are retained; original binary PC trees are ignored local
cache, with no remote availability assertion. No profile wall time is used as a
speedup result.

The concrete next hypothesis also follows directly from source: the legacy
WordField uses u128 `% p` in every add and multiply. The repository already
provides Montgomery Fp64 and fixed-limb Fp224 in fp_backend.rs. The new adapter
reuses that owner code and preserves the evaluator, GCD algorithms, primitive
counter meanings and physical points. It must normalize output ordering from
Montgomery to canonical integers. Constructor/precomputation costs must be
inside the full cold clock; their internal primitive operations are not yet
intercepted by the old evaluator counters, so fast cold-counter totals must be
labelled incomplete, not fully priced arithmetic.

The standalone facade compiles the owner source with only the unused bounded
Fp64Arithmetic::try_new API excluded (it links the unrelated walk subsystem).
All used Fp64/Fp224 constructors and arithmetic functions are the literal owner
source, not a separate arithmetic implementation. The default root module is
unchanged. Adapter tests cover conversion/edge/random products and complete
coset/rational relations for every point on the order19 F17 curve, including
the identity, over both word and fixed-limb representations. Performance remains
an untested hypothesis pending the new controlled comparison [pcc:unverified].
