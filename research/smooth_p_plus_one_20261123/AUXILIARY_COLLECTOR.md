# Bounded auxiliary RR5 collector: software qualification

2026-11-25 · `auxiliary_collector_v1` · PR #5142

Add a separate native crate that consumes the existing auxiliary base adapter
and calls the unchanged traced algebraic visitor. Preserve every published
source/evidence pin. Qualification covers equation construction, signed replay,
trace policy, bounded capture and completion/status control using fixed F31
fixtures and an injected test driver. It does not invoke F4/msolve, launch a
scientific experiment or discharge the separately gated output/cost panel.
The native backend must compile, but is explicitly unexecuted in this component.

For target Q=(r,s), parameters t0,a1,a0 and signed leaves (x_i,y_i), use

\[
 (y_i+s)(x_i+t0)+(x_i-r)(a1 x_i+a0)=0.
\]

This sign convention follows the actual native RR5 signed-lifting code. It is
the standard pole-five relation applied to the negatives of the leaves, with
Q as its fifth zero. The result report must present it as established RR5
machinery and prose reasoning, not a new checked theorem/certificate.

Each leaf receives either the adapter's actual-base dense predicate or its
chained rational-map predicate. Add explicit inverse guards for all six
pairwise x differences, all four x_i-r differences and all four x_i+t0
denominators. The adapter already supplies nonzero-y guards and map-pole
exclusion. The collector requests the same distinct-x, target-disjoint regular
chart as the existing cofactor implementation; unsupported charts remain
explicit and are never reported empty.

Input limits: at most16 base points,64 variables,20,000 input terms,4096
captured assignments, a positive deadline at most60 seconds. Native F4 uses
degree cap16 and the existing resource limits20,000 pairs,8192 symbolic
monomials and2,000,000 matrix entries. Construction, capture, validation and
final completion checks share the caller's deadline. No domain enumeration
or reference tuple index is used by the collector. Only algebraic univariate
root branches are accepted. A nonterminal native result with neither a
univariate nor a nonzero constant is refused before fallback enumeration.

Completion requires the native visitor's complete flag, no timeout, early
stop, trace refusal, resource/output cap, driver error or missed deadline,
and successful replay of every captured assignment. Replay checks every
original equation, canonical coordinates, base membership, distinctness,
target/denominator guards and an independent native group sum. Preserve
captured assignments and valid relation prefixes on incomplete outcomes;
never call a prefix an exact answer. Sort signed relation indices and deduplicate
permutations, recording the duplicate count. No subgroup log or scalar hint
is accepted by the collector interface.

The semantic domain is Fp assignments. The chained leaf system may have
additional extension-field points; native root extraction must return field
elements and full assignment replay is mandatory. No equality of ideals over
the algebraic closure, radicality, solver completeness certificate or runtime
advantage is assumed from small map degree.

Fixed positive control: auxiliary and target E:y²=x³+x over F31, G=(6,6),
depth3 and anchor G. Leaves G,5G,9G,13G are
(6,6),(17,1),(13,3),(23,21); Q=28G=(2,14). The regular RR5 parameters are
t0=23,a1=18,a0=26. This is a known public relation for software checking;
the test driver receives its prepared assignment, while the real native
driver accepts only the equation system and limits. Do not reinterpret the
injected driver's output as discovered relations or a successful F4 run.

Tests cover both presentations, all emitted equations, fixed group replay,
corruption of coordinates/parameters/guards, malformed input and unsupported
charts, deduplicated permutations, invalid-row rejection with prefix retention,
incomplete/error/empty statuses, bounded capture and refused trace branches.
Retain every invocation and exit; run the component suite/build once unless
a failure or later code change requires another invocation. Emit no scientific
stream record. A future admitted comparison must include base/map setup,
input construction, all solver attempts and rejected branches, permutations,
capture/replay/deduplication, rank/linear algebra and final scalar replay.

Sources: actual native `extractor.rs` signed lifting and `oriented.rs` traced
visitor policy, the existing auxiliary/adapter protocols, and PKC2016
Sections3.2/3.4. KB status and exact collector queries are retained separately.
