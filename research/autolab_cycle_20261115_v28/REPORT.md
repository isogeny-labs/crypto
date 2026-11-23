# Signed block lifting validated across nine fields

Hypothesis: the prepared exact field-generic affine-fiber reduction preserves complete original solution sets with correct odd-characteristic signs, including singular and exceptional cases. This first execution passed.

Independent polynomial-convolution arithmetic checked every scalar addition/multiplication pair and brute-force inverse over F3,F5,GF9 andGF25. For each supplied matrix and domain frame, a literal oracle enumerated every original inner assignment; a separate reconstruction enumerated the entire returned affine space. Complete sets, counts and hashes agreed. Cases include zero and inconsistent systems, dependent and full-rank matrices, every supplied-kernel dimension, empty equation lists and zero-variable systems.

| Field | Recorded matrix cases | Additional semantic cases | Scalar pairs |
| --- | ---: | ---: | ---: |
| F3 | 80 | 3 | 9 |
| F5 | 80 | 3 | 25 |
| GF9 | 80 | 3 | 81 |
| GF25 | 66 | 3 | 625 |

The worker checked 93,454 literal original assignments, below the frozen 100,000 cap. This means complete solution sets for the registered cases, not every possible matrix over these fields.

A separate oracle checked 16 full-width cases over each source-identified NIST coordinate prime P-192,P-224,P-256,P-384,P-521. Their triangular leading block gives an independently known affine line, or an explicit contradictory third equation. All 80 cases and full-width inverse checks passed. These establish linear-backend correctness on the registered examples, not new primality proofs, curve validation, hard nonlinear root solving, timing coverage or the release field gate.

The experiment was frozen before execution and used lab.runner resume/run-ready with 90 seconds, 4 GiB and one core. Worker time was 1.083865796 seconds and peak process RSS 31,340 KiB. This includes the correctness workload; it is not a matched production benchmark. Raw SHA256: 3cde41bf85e3d955da130e122a853d09a47abb1583dbaaa6f6cf580a64ae76de, 174,758 bytes.

The portable standard-library verifier imports no solver or arithmetic backend. It independently re-enumerates all 306 recorded small-field solution sets (92,706 assignments; semantic cases were separate in the worker), checks their hashes, and verifies all 80 NIST line/contradiction records by integer modular formulas. It passed. Sole-model source preparation remains distinct from algorithmic oracles and independent external source admission.

Static review caught a noncanonical F3 literal before registration and added exact failure contexts; unexecuted drafts are retained. No frozen algorithm, input or raw evidence was changed. One scientific experiment ran this session. The prepared backend's old documentation remains intact; this report supplies its first executed validation evidence.

Changed next stage: integrate the signed field interface with an odd-characteristic nonlinear representation. The new prepared odd_norm_20261115_v28 compiler proves that on y^2=x^3+a x+b, fixed h,k and desired first two quintic elementary coefficients determine r,s uniquely by division by two, preserving h=0 and k=0. This differs from binary additive-square elimination. Source and proof are syntax-checked but unexecuted and unregistered. No compiler measurement or correctness result is inferred from backend tests.

Next experiment: freeze a separately reviewed compiler worker, compare all h,k,r,s over F3/F5/GF9 with independent literal norm arithmetic, add bounded GF25 and actual NIST curve/generator cases, and preserve repeated-root, characteristic-three, zero-branch and invalid-target tests. Start with 90 seconds/4 GiB/one core, bounded state/output counts. Other extension degrees need an arithmetic backend; this implementation currently supplies primes and quadratic extensions only.

The sparse/Frobenius-aware route remains separate: the preceding complete-system displacement rank-eight prediction was refuted; an exact AS block plus bounded residual generator remains a candidate, not a speed result. Norm representations and structured elimination are prior art; novelty clearance is outstanding.

The requested 8x objective and unchanged genuine-F4/F5/F6 release gate remain open. No complete original 339-variable root set, norm/source/ghost/chart coverage or qualified end-to-end speed ratio is claimed. Internal experiment-management provenance remains separate from the public mathematical checkpoint.

Reproduce: restore exact raw JSON through its artifact reference if relocated, then run python research/autolab_cycle_20261115_v28/REPLAY_RECORDED.py. Fresh worker runs need new output paths and a frozen plan; do not overwrite existing evidence.
