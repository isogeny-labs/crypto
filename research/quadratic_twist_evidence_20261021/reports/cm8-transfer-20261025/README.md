# Actual F13 twist transfer and finite useful S3 comparisons

2026-10-25 · run `cm8-transfer-20261025` · existing PR2381 ·
[protocol](../../CM8_TRANSFER_20261025_PROTOCOL.md) · [PDF](report.pdf) ·
[all-comparison graph](comparisons.svg) · [map/recovery diagram](bridge.svg) ·
[native records/audit](../../native/cm8-transfer-20261025/) ·
[Lean](../../lean-cm8/CM8Transfer.lean) ·
[validation/raw restore](../../validation/cm8-transfer-20261025/README.md).

The C++ producer and independently implemented Rust arithmetic verify the
explicit coordinate map on all14 rational source points, **196 actual addition
pairs**, its exact two-point rational kernel, the seven subgroup images, and
**72 separate main-source scalar recoveries** across12 factor-base arms.
The generator is the public lexicographic first nonzero point killed by7,
P=(0,2); actual image U=(8,11). Phi is a coordinate computation, not a scalar
lookup. Source labels generate/check fixtures outside recovery. All prior
models, sources, certificates, registry/catalog/benchmark records stay unchanged.

## Exact construction and identity

The registered source is `icv1-fp4-t0-6bee67e0`, E/F13:y²=x³+x+4.
The intermediate is `icv1-fp4-t0-bb514d71`, H:y²=x³+4x+7.
The final is `icv1-fp4-t0-097da6b6`, T:y²=x³+4x+6, exactly the d2 monic
quadratic twist. Translate the source two-torsion point(10,0), apply the
standard degree-two coordinate construction, and scale the intermediate by5:

    Phi(x,y)=(12*(x+2/(x-10)),8*y*(1-2/(x-10)^2)).

Infinity and(10,0) map to infinity; all other denominators are nonzero.
Rust independently enumerates the source/image equations, all rational
addition pairs, kernel and subgroup scalars. Lean constructs actual Mathlib
nonsingular points, proves the coordinate image equation, both generator
annihilators/nonzero statements, all seven scalar images and all49 subgroup
addition pairs. It proves an actual finite source reduction:

    Q in <P>, n in Fin7: Phi(Q)=nU iff Q=nP.

The source-membership hypothesis remains explicit. Neither a global geometric
isogeny theorem nor a general-model normalization theorem is inferred from
finite enumeration. No short scaling c satisfies c⁴=4,c⁶=8. Lean also exhausts
the necessary coefficient equations of standard Weierstrass changes
X=u²x+r,Y=u³y+u²sx+t; no u≠0,r,s,t satisfies them. The library's general
classification/normalization bridge is not formalized here. Selected generator
EC1 representations are not newly registered or aliased to unknown generators;
exact model and coordinate bindings stay explicit.

Universal cleared Montgomery coordinate identity IDC1h45f7f67f60c827c3 passes
32+16 native points and false identity/source controls. It certifies polynomial
arithmetic only; the source domain, denominators and actual scalar result rest
on the separate checked statements. All16 previous study identities/sources
remain unchanged,17 identities total.

## Useful rows and relevant degrees: all policies, including losses

All three size-two bases from the three subgroup x classes are tested on each
curve. All nine canonical base pairings and all six nonidentity source targets
are retained: **54 target comparisons**. Three transported source bases and
three pulled-back target bases give12 arms/72 canonical query records, each
with both original and reduced solver certificates (144 complete certificates).
Every row retains an exact witness in its three original supplied inputs.
Rust independently replays the incremental row solver and separately audits
its row spans, witnesses, promotion/evaluation ranks and terminal conditions.
It also reproduces24 frozen earlier prefix certificates by actual query x,
even though the prior twist generator differs from the transported U.

| Fixed first target Q=P | Main base(0,7) | Twist base(6,11) |
| --- | ---: | ---: |
| Actual base points |(0,2),(7,4)|(6,5),(11,4)|
| Distinct coefficient rows mod7 |1|2|
| Incremental useful rank |1|2|
| Ordered constrained roots |1|3|
| Original operational matrix degree |4|4|
| Original expanded witness peak |6|4|
| Reduced operational matrix degree |3|2|
| Reduced expanded witness peak |3|2|

Main first row is(0,2). Twist rows are(0,−2) and(1,1), with nonzero
determinant modulo7. Lean kernel-checks the complete ordered sign-lift sets,
the determinant, and base-log equations yielding(5,3) for this twist base.
Setup logs are actually solved from public relation equations until rank2,
then verified against points. They are not recovered by enumerating logs.
There are20setup relation queries,72online queries and36online forward-map calls; these counts are not calibrated costs. Every online recovery receives Q alone, maps it where needed, runs the actual
reduced S3 solver and verifies the recovered source scalar. All signs are
retained, including after a successful sign in that query. Public base pullback
currently enumerates seven public source points; that dependency is explicit.

Exactly **3 of54 comparisons** have both greater incremental useful rank and
lower named degree on reduced S3; **0** do on original S3. Both complete
canonical streams eventually reach rank2. These are fixed-policy finite
comparisons, not a stationary useful-yield law or superiority over the strongest
source policy. The supplied size-two leading forms retain the same homogeneous
cutoff2 on both sides. Operational matrix degree and expanded witness peaks
are distinct; neither is renamed intrinsic regularity or F4/F5 runtime.

The transported/pulled-back controls preserve every ordered accepted lift,
coefficient row and rank step. Across72 matched original/reduced degree checks,
68 are equal and **4 differ**: at n3,4 main0→matched0 goes2→3, and
twist2→pullback2 goes3→2. All four favor the source presentation. This is a
recorded control outcome, not omitted counterevidence. Yield is conserved under
matched subgroup transport while the nonlinear coordinate encoding can change
this solver's degree. All nine pairings, ties/losses and traces remain available.

## Executed validation and remaining work

The unmodified scoped proof gate passes **208 public named study theorems /
269 audited declarations**,0 failures/extra trusted;17 public/20 audited are
new. Only propext/Classical.choice/Quot.sound occur. No trusted native evaluation,
external lean4checker or universal formalization of the native solver is claimed.
The exact fresh `run-cm8-transfer.sh` passes canonical bytes, eleven rehashed
semantic controls (map, subgroup/source, arm/target coverage, degree/witness,
row/log, query/scalar), all101/no receipt, plus22 backend tests. There are zero
failed online shifts: the lost-failed-query control is explicitly null; the
missing-query control removes a real successful query. No failure is invented.
Fresh root release library4479 passed/119 existing ignores; workflow/naming/
card/size/artifact/source/output/archive checks are recorded in validation.

All native commands stay under600s; no timing, calibrated cost, speedup,
GAE/S/rho/external-fastest or leaderboard result is admitted. Setup/map calls
and actual query work are recorded, but complete arithmetic/storage overhead
remains unpriced. This is a verified finite reduction and useful-row/degree
comparison, not a general ECDLP hardness statement. General geometric maps,
normalization, exceptional S3/S4 beyond this finite seed, generic pairing,
universal trace-zero and stationary useful-yield/complete-cost obligations
remain. Conductor exits127, no reservation/replacement; no paid compute,
new registry inputs, upload or credential reuse occurred.

The [next explicit-dual draft](../../../../backlog/XP-20261025-cm8-explicit-dual/README.md)
proposes D with v=12x−6 and D(x,y)=(10*(v+5+8/v)+10,12*y*(1−8/v²)),
including kernel(7,0). It must verify both compositions as doubling and the
r7 inverse[4]D before replacing public-base enumeration. This dual is
**not tested by this round**. All prior evidence stays frozen. Canonical
scoreboards/browser/leaderboard have no new cost/model/session claim and
remain unchanged; this report's graph includes every paired target.
