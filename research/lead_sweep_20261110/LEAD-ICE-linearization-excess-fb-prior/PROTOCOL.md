# LEAD-ICE-linearization-excess-fb-prior

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_unexamined (also 20261101). Family: ic-engine. Last reviewed 2026-10-24. Source: atlas_survey_20261107 --pursuit.
- Title: Linearization excess as a factor-base design prior inside descent cells.
- Next step (truncated in leads.json): "On the m = 3 and m = 4 descent cells, rank candidate factor-base subspaces of equal size by e. M..." (rest unavailable; atlas YAML encrypted).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; hits in /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 275, table row only) and docs/session-logs/2026-10-26.md (line 468: "Advance this lead ... actual point/subgroup inventories and fresh independent-rank evidence remain required beyond structural scores"). Also listed, not read: research/ecdlp_subexp_desk_20261101/data/{atlas_tally,candidate_matches}.json.
- Not found: any definition of "e". The toy below fixes its own definition (see 2); this is an interpretation, not the lead's.

## 2. Hypothesis
Over F_{p^n}, for an F_p-subspace V (dim l=2) of x-coordinates defining a factor base S(V)={points with x in V}, define the linearization excess e(V) = l(l+1)/2 - dim_Fp(V*V) (collapse of the degree-2 product space). Claim: among equal-|V| subspaces, those with e>=1 have strictly higher m-fold sumset coverage (fraction of the cyclic group reachable as a sum of m FB points, m=3 and m=4) than the size-matched e=0 baseline, beyond what |S| alone predicts, with mean residual >= 0.01 absolute.

## 3. Preregistered prediction and falsification criterion
- Statistic: mean over units of mean residual coverage (coverage minus mean coverage of e=0 subspaces with the same |S|) for the e>=1 class. Pooled over 8 units (4 cells x 2 curves).
- consistent_with_prediction: pooled permutation p <= 0.01 (1000 label shuffles), observed >= 0.01, and >= 75% of units positive. falsified: p > 0.05 or observed <= 0 (e carries no yield information beyond |S|, or the wrong sign). Otherwise inconclusive. Also inconclusive if the positive control fails.
- Negative control / null: label-permutation null of the e class labels within each unit.
- Positive control: Spearman correlation of |S| with coverage must exceed 0.3 in every unit (detector sees a known driver of yield).
- A/A arm: random split of the e=0 subspaces into two halves, same residual statistic; reported as aa_abs_diff_max (noise floor; effect should exceed it for a convincing result).
- Decides: whether e is worth carrying as a design prior in the m=3/4 descent cell work (consistent), or should be dropped as a prior (falsified).

## 4. Method
- Cells (p,n,m): (5,6,3), (5,6,4), (7,4,3), (7,4,4); l=2, so |V|=p^2; group orders ~q/2..2q, q in {15625, 2401}.
- Per unit (seeds 101000+..., 202000+..., derived from seed, p, m): build F_q via a primitive polynomial found by random search, random curve y^2=x^3+ax+b with N>=q/2 and a cyclic generator; discrete-log table of all points (point -> k mod N).
- 100 random 2-dim subspaces plus 40 structured ones a*W with W the subfield F_{p^2} (these have e=1; random ones have e=0 almost always). e from F_p rank of {v_i v_j}.
- FB S(V) = points with x in V (both signs); coverage = |{s1+..+sm mod N}|/N via iterated sumsets (m=4 as S2+S2).
- Baseline: mean coverage of e=0 subspaces with the same |S| (nearest |S| if missing); permutation, A/A and positive control as in 3.
- Cost: pure Python, one core, small memory (< 100 MB); estimated 1-6 minutes total (smoke: 0.6 s); timeout 660 s in run_cmd.

## 5. Classification: runnable_local
Everything needed (finite field, curve arithmetic via log tables, sumsets, permutation test) is stdlib and fits the limits.

## 6. What a run cannot establish
- Not the lead's own e: the atlas definition is not visible here; this toy's e is a proxy defined by me. A result does not confirm or refute the recorded lead.
- Toy sizes (q ~ 2^11-2^14), l=2 only; no statement about asymptotic exponents, real Semaev/Weil-restriction descent, F4/Groebner linear algebra, or rho comparison (no matched rho, no GAE pricing, no validation gate).
- Structured e>=1 subspaces are all subfield multiples, so e is confounded with that structure; random e>=1 is too rare to separate them.
- Coverage is a sumset yield, not relation-collection cost. Results are labelled [pcc:unverified].
