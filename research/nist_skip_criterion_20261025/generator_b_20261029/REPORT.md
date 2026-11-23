# Base-point and coefficient investigation — 29 October 2026

The historical skip criterion remains unknown. The requested wider comparison
is prepared and tested on synthetic inputs, but has **not run on the actual
21-curve cohort**. The restored environment has no configured checkpoint-upload
identity; [AGENTS.md](../../../AGENTS.md#resume-checkpoints-in-s3-before-every-run)
requires a successful uploaded checkpoint before an experiment starts.

## What the G–b connection establishes

For the user's canonical point, the defining equation is
`y² = x³ − 3x + b (mod p)`. Thus `x=0` implies `y²=b`, and `x=1` implies
`y²=b−2`. The supplied P-224 index 777 and P-256 index 876 points use x=0;
P-224 indices 1485 and 5736 use x=1. A close numerical or algebraic relation
between these assigned points and b is therefore expected. These points were
chosen by the user’s rule; they are not recovered historical generators.

The existing, independently checked complete-factor feature tables record
`b:chi_p=1` for every one of the 17 non-P-224 rows. Including P-224 index 777,
b is square for nine of ten published and nine of eleven skipped curves.
P-224 indices 1485, 5736 and published 7026 have character −1. The literal
old measurements and member hashes are in
[evidence/existing-b-character.json](evidence/existing-b-character.json) and
[the inherited-feature custody record](evidence/existing-feature-custody.json).
This reuses prior results; it is not a new cohort execution.

Under the reference convention in the non-P-224 streams, p≡7 mod8 and
`b=R^((p+1)/4) mod p`. The exponent is even, so this b is square. The
first-x point rule consequently starts at x=0 for both labels. Conditional
on the seed-generation equation `c*b²=−27 mod p`, x=0 also implies
`c*y⁴=−27`; x=1 implies `c*(y²+2)²=−27`. These are consequences of the
stated equations, not a selection mechanism or an attack. Five conditional
[Lean theorems](lean/CurveEquation.lean) compile; their transitive axiom audit
passes using only standard foundations. The [proof map](lean/PROOF_MAP.md)
states their assumptions and limits.

## Model and generator controls

The historical P-224 coefficients for 777 and 5736 have the opposite sign to
the user's smaller-b representatives. Keep these models distinct. Since
P-224 p≡1 mod4, choosing i²=−1 transports b to−b by
`(x,y)→(−x,i*y)`. Substitution negates both sides of the curve equation;
the inverse map exists because i≠0. This readable derivation and native toy
controls are not a formal proof of the full elliptic-curve isomorphism.
The prepared real-curve checks must verify the actual transported generators.
Do not identify opposite signs as same-order models over p≡3 mod4.

The comparison retains canonical points, ten standardized published
generators and **eleven unknown historical skipped generators** separately.
It tests both signs of seven canonical-point multiples and 22 public scalar
recipes. A known-scalar equality test is not discrete-log recovery. A relation
found only for an assigned canonical point cannot explain how an unavailable
historical generator was chosen. Implementation and protocol preconditions
would still be needed to turn any trait into a vulnerability.

## Prepared scope and actual checks

There are infinitely many functions of curve parameters. The finite inventory
is explicit: the existing 1,127 traits, plus 171 point/coefficient traits across
two coefficient views and 14 point choices. The views and point choices are
controls on the same 21 curves, not additional independent historical samples.
Planned outputs include all 25 same-field published/skipped pairs, 28,175
inherited differences, 119,700 point-trait differences and 19,152 tight
published-envelope comparisons. No resulting actual-curve values or matches
are claimed. All labels are already visible; any fitted rule is exploratory.

The actual development check uses six tiny prime-field examples repeated
into a 21-row parser fixture, with planted G=[b]Q relations and dummy inherited
features. The separate GMP verifier checked its 100,548 trait cells,
19,152 envelopes, 119,700 point differences and 28,175 dummy-feature differences.
Nine deliberately corrupted outputs were rejected for the intended reasons.
The fixture’s 106 recipe matches and zero exact single separators are **toy
results only**, with no implication for the NIST cohort. See the
[development report](reports/development/REPORT.md), raw-bundle custody and
[control receipts](controls/synthetic-verification.json).

Production flow: one comparison planned; zero attempted; zero completed;
zero curves excluded. No DLP, timing benchmark or paid experiment ran.

## Requirements and remaining obligations

| Request | Status and evidence |
| --- | --- |
| Original four ICV1/DiSSECT assessments | Previously delivered in the [original study](../../curve_candidates_dissect_20261021/README.md). |
| Four remaining complete factorizations | Verified complete: [84/84 records across all 21 curves](../factor_followup_20261026/REPORT.md). Unknowns were not numeric labels or silently dropped. |
| Comprehensive differences and G–b investigation | Partial: equation consequences and existing traits assessed; finite native comparison implemented and synthetically checked; actual new measurements blocked. |
| Historical criterion explaining skips | Unresolved. Existing post-hoc separators and failed fixed hypotheses do not establish original use. The user has no original visit/acceptance logs. |
| Weak selected or weak skipped curves | Neither established. No applicable attack construction or measured DLP. |
| Updated PDF and PR | This report/PDF and existing PR 2666; preparation delivery and actual cohort comparison remain open. |

Restore the existing checkpoint identity through runtime settings, admit the
preparation bundle, freeze the supported execution route and run the full
comparison with independent verification. Keep failures, zero recipes and
unknown historical generators explicit. A separation would justify a stated
prospective test; a negative comparison would exclude only this finite family.
Neither outcome alone supplies historical intent or a security verdict.

Sources: [SEC1v2 §3.1.3.2](https://www.secg.org/sec1-v2.pdf),
[RFC5639 A.2](https://www.rfc-editor.org/rfc/rfc5639.html#appendix-A.2),
[pinned std-curves](https://github.com/J08nY/std-curves/tree/77fe6e3585ca2c2225b59d7df24b7c775437276f),
[prior source check](../source_check_20261027/REPORT.md),
[PROTOCOL.md](PROTOCOL.md), [REPRODUCE.md](REPRODUCE.md).
SEC1 and RFC5639 supply methods to compare; they do not authenticate the 1997
caller or historical skip rule. Standard-source characteristics are not
substituted for independently measured traits.

The new [diagram](visual.svg) describes this evidence boundary. Existing
factor, embedding, FreeLIP and ECPP-work figures remain unchanged because no
new actual-cohort numerical result supersedes them. No leaderboard, performance
gain or security metric is promoted. Storage migrations in upstream changed
some old metadata bindings; additive custody observations preserve those
differences without rewriting frozen manifests.
