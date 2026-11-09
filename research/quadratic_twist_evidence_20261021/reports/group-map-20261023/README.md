# The explicit twist map is a group isomorphism onto the minus eigenspace

2026-10-23 · `group-map-20261023` · PR #2381 · proof iteration, not a timing run.

**Checked result:** the existing actual-point twist map preserves addition and
all integer scalars and is a group isomorphism onto the entire Frobenius-minus
eigenspace of the explicit elliptic short model. Rational base-change points
are exactly the Frobenius-fixed eigenspace. Lean now transports the annihilator
from the source point; it is no longer assumed separately on the image.
The unmodified proof gate passes **49 named research theorems, 57 audited
declarations including eight generated helpers, zero failing/extra trusted**.

[Inspected report PDF](report.pdf) · [editable diagram](bridge.dot) ·
[vector diagram](bridge.svg) · [readable catalog chart](jacobian-readable.svg) ·
[raw native chart](jacobian-chart.svg) ·
[proof map](../../lean/README.md) · [raw validation and failures](../../validation/group-map-20261023/README.md).

## Scope and mathematical proof

Let k be finite of odd characteristic, q=#k, d a nonsquare, K any field algebra
containing u with u²=d, and E: y²=x³+ax+b an elliptic short Weierstrass model.
Let E_d: Y²=X³+d²aX+d³b. No degree-two or finite-extension hypothesis on K
is assumed. The existing map is psi(O)=O, psi(X,Y)=(X/d,Y/(d·u)).

1. `TwistGroupMap.slope_scale` proves slope scaling by 1/u, considering secant,
   tangent and vertical cases. The addition coordinates scale by 1/d and
   1/(d·u). The actual-point proof separately includes O and inverse pairs.
   It constructs `liftHom`, not an abstract map assumed to preserve addition.
2. Scalar and finite-sum compatibility follow from that actual group map.
   The existing injection proves r·psi(P)=O iff r·P=O. Together with the
   earlier derived Frobenius sign, A+psi(P)=R for rational A,R and odd r
   with rP=O implies P=O. For a finite family with common odd annihilator,
   the conclusion is that the source sum is zero, not that every term is zero.
3. `TwistEigenspace.frobenius_fixed_iff` derives the fixed field from the
   complete root set of X^q−X. If Frob(Q)=−Q with affine Q=(x,y), x is fixed
   and y is negated. Thus d·x and d·u·y are fixed and descend to X,Y in k.
   The actual twist equation and nonsingularity are checked, giving a source
   point whose image is Q. The construction includes O and two-torsion.
   `liftEquivMinus` is an actual additive equivalence onto **all** minus points.
4. The same fixed-field result descends the coordinates of every fixed actual
   point on a general elliptic Weierstrass model. This proves exactly the
   rational plus-space identification, without assuming descent.

The group-map identities themselves need only d≠0 and u²=d over fields with
the stated elliptic model; finiteness/nonsquareness/oddness enter the Frobenius
and odd-order consequences. Only propext, Classical.choice and Quot.sound occur.
The theorem-by-theorem hypotheses and declaration names are in the canonical
[proof map](../../lean/README.md); source files are
[TwistGroupMap.lean](../../lean/TwistGroupMap.lean) and
[TwistEigenspace.lean](../../lean/TwistEigenspace.lean).

The [proof protocol](../../GROUP_MAP_20261023_PROTOCOL.md) was committed in
`6fb418e8a` before execution. Lean 4.34.1 and Mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612` remain pinned. The original
TwistEmbedding source hash remains
`a0ea9109317a03a5c6e88c5e3adacd9806118ea564b75964290f4eee7602c9c8`.
Earlier dated reports describe their earlier verification state and are preserved.

## Native algebra controls and PR-caused metadata repair

Four source-bound polynomial certificates, each replayed at 32+16 points:

| Polynomial part | Certificate |
| --- | --- |
| Tangent numerator factor | `IDC1h501f62832dc059fb` |
| Cleared addition x coordinate | `IDC1he0c463dc2e585208` |
| Cleared addition y coordinate | `IDC1hfb09521ce1a737cb` |
| Short-model discriminant scaling | `IDC1hb3a3c677428bc4b1` |

The native checker rejects each false plus-one polynomial with an actual
counterexample and rejects each false Lean source hash. Exact corrupted payloads,
counterexamples, source hashes and checked records are retained in
[certificates](../../validation/group-map-20261023/certificates/).
These records cover polynomial parts; Lean separately proves division, the actual
group maps and Frobenius statements. The exact CI replay command and all 22
isolated backend tests pass. No certificate is called a full elliptic proof.

The current-head Jacobian CI failure was caused by this PR's registry additions:
the unchanged generator accepts the old 2,572-model base registry and rejects the
old report with the new registry. Following the separately committed
[repair protocol](../../JACOBIAN_REFRESH_20261023_PROTOCOL.md) (`ea1929fd9`), the
native producer regenerates the active catalog at the unchanged 200,000 budget:

| Catalog field | Before | After |
| --- | ---: | ---: |
| Models | 2,572 | 4,493 |
| Declared formula routes | 6,565 | 11,365 |
| `boundary` curve verdicts | 1,917 | 3,838 |
| `formula_advantage` | 598 | 598 |
| `formula_marginal` | 57 | 57 |

All 2,572 prior per-model objects and the cost model remain exactly equal;
membership equals the full registry, without duplicates. The registry itself is
unchanged. Native replay, table/query/chart checks and altered-file rejection pass.
The marked Markdown summary table regenerates byte-identically because the
formula-advantage/marginal population is unchanged. Rows are conditional formula
leads, not constructed transfers, timings or security findings. The raw native
SVG/TikZ are retained. Their overlapping labels are replaced in the PDF by a
readable plot of the already recorded subgroup-bit and formula-ratio fields;
`plot.py` performs document layout only. All 4,493 models are plotted.

The strict cover assessment then detected the changed inventory digest. Under
[its additional protocol](../../ASSESSMENT_REFRESH_20261023_PROTOCOL.md)
(`b17483810`), source bindings were refreshed and the obsolete claim of stale
registry membership was corrected. The existing `refuted` applicability status
now explicitly concerns the stronger claim that formula rows themselves supply
verified executable transfers. Its status, the historical open-obligation
ledger and every non-source assessment section remain unchanged. This is a
disclosed scope correction, not a transfer promotion. A first attempt to mark
that artifact `supported` while retaining its obligation correctly failed the
unchanged independent verifier; its exact payload is retained as a reconstructible
patch and hash. The final generator byte check, schema and independent verifier
pass with zero problems. Both catalog provenance dependencies are preserved.

## Execution, requirement status and limits

| Requirement | Status and evidence |
| --- | --- |
| Actual group/scalar map, source-order obstruction and full explicit short-model eigenspaces | verified in Lean; 21 new named theorems, unmodified gate passes |
| Native polynomial replay and corruption controls | verified; exact bytes and 22 backend tests pass |
| Full library suite | 4,479 passed, zero failed, 119 existing ignores |
| Workflow metadata parser | all four tests pass; no Python research arithmetic/harness introduced |
| Frozen evidence and provenance | retained; old source/certificates/reports/942-model study unchanged |
| General-model normalization, exceptional S3/S4 lifting, pairing reduction and universal trace-zero geometry | open in the existing problem record |
| Shared Conductor access | CLI still exits 127; no reservation or replacement coordinator claimed |
| Complete exact-head review/check/merge gates | incomplete; PR held open |

The 942-model finite population and the 4,493-model global catalog are distinct.
No new finite relation-search or timing claim is made in this proof iteration.
The prior exhaustive 74,693,760-word experiment remains independent finite
evidence. No general ECDLP hardness, GHS algorithm, speedup or universal
no-connection conclusion follows. The positive pairing/IC example over F_43
(r=11, embedding degree two) remains a concrete twist connection; its native
certificates do not close the open formal pairing-reduction bridge.

The initial CI snapshot in [ci-before.json](../../validation/group-map-20261023/ci-before.json)
has 26 all-event runs: 15 success, four failed, four cancelled and three queued.
The stale Jacobian failure is repaired here. Other failures are the inherited
SEA Lean 4.19 `--trusted` incompatibility, an authoritative upstream metadata
download returning 404, and CodeQL uploads reporting repository code scanning
disabled. These are not patched around. Complete check-runs and combined status
endpoints currently return 403, so workflow successes alone cannot satisfy a
complete merge audit. Submitted reviews/threads are empty in this snapshot.
Copilot was already asked to handle merge conflicts; the three conflicting
browser/traits/leaderboard files are untouched. All final-head CI is a separate
gate; no old successful run is substituted for it.

The [problem record](../../../../problems/quadratic-twist-evidence-native-parity.md)
now names exceptional S3/S4 lifting and compatible general-model normalization
as the next discriminating steps. These are obligations, not unscheduled work
represented as running. All prior failed builds remain evidence, not proof receipts.

## Visual/source review

The new diagram shows the completed group and eigenspace bridges and the open
ones. The native catalog chart and all rendered copies accompany the regenerated
catalog. Checked canonical artifacts: `docs/index-calculus-scoreboard.html`,
`docs/ic/progress-timeline.json`, `docs/curves/TRAITS.md`, existing study figures
and dated Jacobian-scan figures. No IC/rho measurement, performance result,
new transfer construction or subgroup identity is promoted; those scoreboards
and frozen historical figures remain unchanged. The current Jacobian table is
byte-identical; its active JSON and this dated chart carry the new population.

Primary mathematical APIs are the pinned Mathlib
`AlgebraicGeometry/EllipticCurve/Affine/{Formula,Point}.lean`,
`FieldTheory/Finite/Basic.lean`, and `Algebra/Polynomial/Roots.lean`.
See also Silverman, *The Arithmetic of Elliptic Curves*, III.1 (coordinate
changes) and III.8 (pairings); the latter route remains an explicit open formal
dependency rather than being ruled out by these eigenspace obstructions.
