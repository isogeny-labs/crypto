# AUDIT MP-lattices-2 (2026-10-31/11-01)

verdict: partially_valid. corrected_outcome: supports, but only for the narrow construction (row deletion with all monomial columns kept, JM box, S3, l<=5 exact, l<=3 LLL). Not evidence about BD-style engineered sublattices, S4 l>=2, or unravelled linearization.

## Rerun (reran=true)
- layer2.py rerun in audit_rerun/ (154 s): all 336 records identical to raw/layer2.json on hg, heur, unplanted_recovery, r; margins identical (diff 0.0). Deterministic.
- layer1_run.py S3 l<=4 seed 1 rerun (39 s): delta_star identical for all 8 tau at l=1..4 (matches raw and README).
- layer1_closedA.py rerun: S3 l=16 0.15533, fit delta_inf 0.16294 c 0.13935, S4 l=1..5 0/.0113/.0192/.0239/.0270, S4 fit 0.03693; matches README. S3_lim l=64 = 0.16408.
- l=5 and seed 2 not rerun (2 min each); raw logs inspected and consistent with README (B tau0 .0775/.0791, logvol excess 87.6/86.1).

## Code vs protocol: checks passed
- S3 polynomial in common.S3_specialised matches Semaev S3 (checked term by term vs S3_int); monic in x1^2x2^2.
- S4_specialised coefficient of x1^4x2^4x3^4 is 1 (monic), so the closed-form S4 diag assumption is valid.
- build_rows: k=min(floor(alpha_i/d), l), row x^(alpha-dk) f^k p^(l-k). Reduction of f^k mod p^l keeps validity (still vanishes mod p^l at root) and leading coeff 1; arm A exact det == prod diag to 1e-4 (excess -0.0 in logs). Control arm A is real and triangular.
- Exact Gram det over retained rows with all nonzero columns: correct (Cauchy-Binet gives vol >= prod diag, so excess >=0 as observed: +25.8/+48/+88).
- HG surrogate margin and 12-step bisection: correct; resolution ~1.2e-4. B selection set is recomputed at each trial delta (set shrinks as delta grows), monotone enough.
- Layer 2: lbound=l ln p - 0.5 ln r, norm via ln sqrt(sum sq), division by X^|e| exact. No off-by-one found. Planted xR solved from quadratic, assertion f(x1,x2)=0 mod p. Seeds 100-103 / 200-203 fixed, not cherry-picked (cells reported in full, 4/4 criterion in summarize.maxd is stated).

## Findings (ordered by weight)
1. Near-tautological kill. A is triangular and optimal for the leading-monomial determinant; deleting rows while keeping columns adds off-diagonal mass of size ~p^l*X^|e| per kept row in the dropped-leading-monomial columns, so vol>=prod diag with large excess is forced by construction, not discovered. The experiment rules out the strawman. Real BD/HM gains come from choosing shift sets so the sublattice stays (near) triangular (drop rows AND columns consistently, or add extra shifts); that is untested. README states this scope but the Decision line "Each row lost costs more than the excess it removes" reads as general.
2. "Every proper sublattice is strictly worse" is supported only for 8 thresholds tau in {-0.5..1.5} plus A. The trend is monotone (l=5: 0.0555,0.0681,0.0775,0.0869,0.0908,0.1111,0.1335 as rows 67..121), and tau=1.0 (115 of 121 rows) is already below A (0.1111 vs 0.1335), so the claim is empirically plausible but not exhaustive (no search over single-row or corner-only deletions, no non-threshold subsets).
3. Layer 2 B/C=0 is partly grid-trivial. Layer 1 already gives B reach 0 (l=2) and 0.0435-0.046 (l=3); the Layer 2 grid starts at 0.06 (l=2) and 0.08 (l=3), above B reach, so B=0 is the expected outcome, not an independent test. The table entry "B = 0" is "below grid", not a measured reach of 0. Layer 2 for B' (intermediate tau) was never run; only B (tau=0) and C. C is identical to B (no antisymmetric vectors), so C adds nothing.
4. Layer 2 'heur' for planted instances is oracle-assisted: only vector pairs that both vanish at the true root are resolved, and resultant vanishing at x1 is then implied (res(x1)=0 follows from common root). So 'heur' = (>=2 vanishing, coprime vectors among the 6 shortest). The unplanted false-positive control uses a different blind code path (factor resultant, integer root filter), so control (b) does not calibrate the planted criterion. Bias favours B/C, so B/C=0 is conservative, but A's heuristic reach is optimistic.
5. HG in Layer 2 is a lattice-only property (unplanted A shows HG 4/4 at delta<=0.10), so it is not a recovery criterion; README acknowledges.
6. Pre-registered fit for B (delta_inf(B) <= 1/6, <=1/24) was never computed; only A was fitted (A 0.1629, 0.0369). Rule holds trivially since B<A at every l, but the stated check was not performed as written. A in Layer 1 is closed form so seeds 1/2 do not replicate A; they only perturb B (e.g. l=3 .0435 vs .046).
7. S4: Layer 1 S4 sublattice evidence is empty (A itself is 0 at l=1). The S4 conclusion rests on the arm A closed form only. S4 l>=2 untested. "Do not trigger S5" is not supported by S4 data for sublattices.
8. Surrogate vs LLL: A beats surrogate in Layer 2 (0.11 vs 0.0955), so Layer 1 reaches are lower bounds for achievable delta; B could likewise beat its surrogate, so Layer 1 B/A gaps (0.04-0.055 at l=3..5) are not tight, though the Layer 2 B margins (-0.18..-0.6) agree in sign. n=4 instances per cell.
9. PROTOCOL.md was written after a 2-minute pilot (disclosed); the "prediction_if_true" was not altered after main runs as far as files show (all mtimes within 10:04-10:13, so cannot verify ordering beyond that).

## Overclaim check on README
- Accurate: Layer 1 numbers, Layer 2 table, controls, scope-of-kill bullets, caveats.
- Overclaim/ambiguous: "Outcome SUPPORTS ... rigorous reach of helpful-row sublattices stays <= 1/6" ("rigorous" is wrong: it is a vol^{1/r} surrogate and only A's reach is proved <=1/6; B is only shown < A numerically at l<=5). "Every proper sublattice is strictly worse" (8 thresholds). "B=0" in Layer 2 table (below grid). Decision "Do not trigger S5" for S4 (no S4 sublattice data).
- Classification "accounting" is reasonable. No security claim made. Novelty not asserted.

## Corrected outcome
supports, scope: JM box, S3, delta_surrogate and LLL at l<=5/3, threshold row-deletion with columns kept. For S4 and for non-threshold or column-consistent sublattices: not_run.
