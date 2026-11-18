# Reuse of the validated torsion cache

The subgroup predicate now has a validated() form returning an owned Validated
object. Its private fields bind the value-only engine to p,a,b,r,Q. The projected
initializer consumes that object, checks all bindings, and evaluates active direct
factors using its retained division values. It creates no second factor engine.
The ordinary validate() predicate remains as a compatibility wrapper.

The base and nonanchor direct-window/short-extension paths use the consumed cache.
The twice-base anchor keeps its existing fresh value-only engine, because it did
not separately compute the torsion predicate before this change. All-zero factors
still return the unit pair after checking context and retaining torsion validation.
The full-value reference remains mandatory; the original cohort is unchanged.

Six selected native methods pass. On all three saved fixtures, direct
initialization uses one validation engine and no extra factor engine. Re-requesting
cached psi_r adds no entries; point, field, curve and order binding mismatches are
checked, and a wrong-point cache is rejected by the consuming initializer.
Existing499 subgroup rows,100 splits,1375 advances,450 skips,588 predicate cases,
7 malformed cases and factor/closure checks pass. Claim PCC-20261107-e63506 is
unsigned producer evidence. Each invocation filters61 methods. [pcc:unverified]

These tests establish engine reuse and preserved outputs, not complete arithmetic
or timing savings. Retained factor requests may add more cache entries. Setup,
geometry residual, projection and full-value reference work remain. No scalar
recovery, timing panel or cloud job ran.

Evidence: receipt.json, source-lock.json, cache.log and other named logs. Exact
commands in run-record.py; use fresh paths for replay. generate.py regenerates
the tested source byte-for-byte and generation.json pins predecessor/output hashes.
The report PDF was rendered and inspected; editable SVG/Typst sources accompany it.

Sources checked: preceding torsion predicate, transport direct paths, value-only
BTreeMap engine, and source-identity/ownership requirements. This reuses the prior
subgroup/separability argument without changing it. KB unavailable in current
tools; prior wrapper check lacked the executable. No novelty claim.

Next: investigate direct computation of the projected geometry residual, then
complete-cost accounting. The shared-source tests are not independent review.
