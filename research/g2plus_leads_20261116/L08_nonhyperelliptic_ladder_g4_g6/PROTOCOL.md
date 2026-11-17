# L08 — Non-hyperelliptic ladder at genus 4 (5, 6 optional): Diem line relations on nodal plane models (frozen protocol, 2026-11-16)

Status of everything this lead produces: `[pcc:unverified]` (one producer, shared
4-core host, no independent replay, no pcc claim). This is **not a benchmark**: no
leaderboard row, no `benchmarks/` row, no speed claim, no wall-time metric. Synthetic
random curves only; nothing here is a statement about a deployed curve. Lab numbers
quoted below are `[pcc:unverified]` assertions.

Written-order note: this file and `SPEC.md` were written and hashed
(`frozen.sha256`) before any code of this lead existed and before any datum.

## 0. Prior art checked (grep; research-kb is not installed in this container)

| path read | what it shows |
|---|---|
| `crypto/research/genus_ladder_20261114/{PROTOCOL,RESULTS}.md` | Hyperelliptic g = 2..5 and plane quartics (g = 3, Diem lines) over p = 67..4099. RESULTS line 205: "non-hyperelliptic curves of genus 4 and 5 are not covered". Source of the lab row hyperelliptic g = 4 at p = 1031, IC/rho 0.117 [0.077, 0.216] (best arm C+pre, rho with negation) and of the 393 FPM per genus-4 Cantor step. `[pcc:unverified]` |
| `crypto/research/genus_ladder_bound_r_20261114/RESULTS.md` | Genus 4 hyperelliptic and quartics to p = 8209; quartic Pic addition about 8,900 field multiplications per step (line 98). No non-hyperelliptic genus >= 4. |
| `crypto/research/genus_ladder_{harvest,scale,sge,sge_scale}_2026111x` | Hyperelliptic g >= 4 and quartic variants only. |
| `crypto/research/genus4_n4_census_20261024/REPORT.md` | Existence census of non-hyperelliptic genus-4 Jacobians in Weil-restriction isogeny classes; notes they have plane-quintic models and cites Diem's O~(q^{4/3}) as literature, "not verified here". No relation collection. |
| `crypto/research/leads_20261112/lanes/lane_J_covers.md` (line 417) | Proposal only (theta-isogeny walk to a genus-4 Jacobian, then Diem on a quintic); not executed. |
| `crypto/research/g2plus_leads_20261116/L07_prym_ic_desk_count/PROTOCOL.md` (line 126) | Desk statement of the same exponent class for a cover; no implementation. |
| `crypto/research/plane_quartic_program_20261018/`, `src/cryptanalysis/{quartic_ladder,diem_quartic,plane_quartic_pic,genus_ladder}.rs` | Diem line relations implemented only for degree-4 plane curves (residual 2). No degree-5/6 nodal model anywhere in `src/`. |
| `crypto/research/leads_20261111/EXISTING_LEADS.txt` | No entry for nodal plane quintic / sextic IC. |
| `cryptanalysis/` (grep `plane quintic`, `nodal`, `non-hyperelliptic`) | Only an unrelated nodal-cubic control in `research/handbook-ff-ic-ideas-20261103/IDEAS.md`. |

Conclusion: not a duplicate. The degree-4 (genus-3) case is executed lab work and is
reused here only as the positive control.

## 1. Mathematics of the brief, checked and corrected

1. Genus 4, plane quintic with 2 nodes. Arithmetic genus of a plane quintic is
   (5-1)(5-2)/2 = 6; two ordinary nodes give geometric genus 4. Correct. A line meets
   the curve in 5 points; through 2 chosen smooth points it leaves **3** residual
   points. The probability that a random line section's residual cubic splits into
   3 distinct rational roots is C(p,3)/p^3 = (1/6)(1 - 3/p + 2/p^2), so about 1/6.
   Correct.
2. **Correction (relation to the canonical model).** The quintic is the projection of
   the canonical sextic C in P^3 (C = Q ∩ cubic) from a rational point P0 of C. Lines
   in the plane are canonical planes through P0, so a line through base points B1, B2
   is the canonical hyperplane through the g - 1 = 3 points P0, B1, B2, and the two
   relations agree: line section ~ K - P0. "Canonical hyperplane through g-1 base
   points" with all three points free is a different (denser) relation family; it is
   **not** the one implemented here, because Diem's 4/3 analysis is the plane one.
3. **Correction (population).** The two nodes are the images of the two lines of the
   quadric Q through P0, i.e. of the two g^1_3. They are both rational iff Q is split
   over F_p (P^1 x P^1 with rational rulings). Placing two rational nodes, as this
   protocol does, samples only genus-4 curves with a split quadric (two rational
   g^1_3) and a rational point. Curves with a non-split quadric (conjugate nodes) or a
   quadric cone are not sampled. Diem's exponent does not depend on this; the
   population restriction is disclosed, not corrected.
4. Genus 5: plane sextic, arithmetic genus 10, 5 nodes, g = 5; genus 6: sextic with 4
   nodes, g = 6. Brill–Noether rho(5,2,6) = 2 and rho(6,2,6) = 0, so g^2_6 exist
   geometrically; rationality over F_p is again a population restriction. Residual
   after 2 base points: 4. Correct.
5. Exponents. Diem (plane curve of degree d, double large prime):
   O~(p^{2 - 2/(d-2)}). d = 4: 1 (positive control); d = 5: 4/3; d = 6: 3/2. Factor
   base size in that analysis: p^{1 - 1/(d-2)}. Correct as stated in the brief.
6. Rho. #J ≈ p^g within the Weil band [(√p - 1)^{2g}, (√p + 1)^{2g}]. These curves
   have no automorphism to exploit, and negating a class costs a group operation, so
   rho is charged sqrt(pi #J / 2) steps, no sqrt 2 credit. The lab's hyperelliptic
   g = 4 row did credit negation. Correct as stated.
7. Nodes. A line through a node meets the plane curve there with multiplicity >= 2
   and on the normalization picks up both branch points (possibly a conjugate pair).
   Such lines are **excluded** (tested explicitly, counted as `node_lines`). Points of
   the factor base are the smooth rational points of the plane model; nodes and their
   branches are never factor-base points.

Not void, not a duplicate: proceed.

## 2. Hypothesis and exact claim

H1 (primary). On random genus-4 curves given as plane quintics with two rational
nodes over F_p, p ∈ {131, 257, 521, 1031, 2053, 4099, 8209}, the counted cost (FPM)
of Diem double-large-prime line-relation collection plus the modelled linear
algebra grows like p^e with fitted e ≤ 1.45 (theory 4/3).

Exact claim form if supported: "the OLS slope of ln(mean cost) on ln p of arm A_2LP
(definition §4) is e [lo, hi] (95% curve-bootstrap)". Nothing else is claimed. The
IC/rho ratios are reported as the distance remaining, with no threshold.

**What the cost covers.** Point enumeration, line relation collection (all candidate
lines, prefilter, split test, root finding, normalization), and a linear-algebra
cost model `LA = 3 · cols · W` (W = matrix nonzeros). **Not covered**: descent of a
target, Pic^0 arithmetic, any logarithm. The IC figure is therefore a partial cost and
the IC/rho ratio a lower bound on the true ratio (optimistic for IC).

## 3. Instances

- Primes: 131, 257, 521, 1031, 2053, 4099, 8209 (same grid as the lab ladder).
- Genus 4 (primary): d = 5, m = 2 nodes at affine (0,0) and (1,1).
- Positive control: d = 4, m = 0 (smooth plane quartic, g = 3).
- Optional stage 2 (only if stage 1 is complete): d = 6, m = 5 (g = 5) and d = 6,
  m = 4 (g = 6), primes 131, 257, 521, 1031, 2053. Not part of any verdict.
- Node k (k = 0..m-1) at affine (k, k^2) (no three collinear).
- Curve generation (SPEC §3): coefficients from `random.Random("L08-curve-d{d}-m{m}-p{p}-s{s}")`,
  s = 1, 2, …; node conditions solved linearly; admitted iff the discriminant test
  (SPEC §3.3) proves the curve irreducible with exactly the m prescribed nodes and
  genus g = (d-1)(d-2)/2 - m. First 4 admitted seeds per (d, m, p). Rejections are
  counted by reason. Curve set-up (generation, genus test) is not charged.

## 4. Arms

Per admitted curve, all arms share one counted point enumeration (its FPM is added to
every arm).

| arm | factor base F | relations kept | role |
|---|---|---|---|
| `FB` | all smooth rational points | full only | baseline (Diem full base, theory ~p^2 with LA) |
| `2LP`, cF ∈ {1, 2, 4} | first nb points of a seeded shuffle, nb = min(N, round(cF · p^{1-1/(d-2)})) | full, 1LP, 2LP (graph) | primary |
| `2LP` rep 2 (A/A), cF = 2 | same F | same | A/A noise floor: only the pair order and the splitting RNG differ |

Candidate lines: lines through unordered pairs of distinct F points, visited in a
seeded full-period pseudo-random permutation of pair indices (SPEC §5). A run stops
when useful relations ≥ needed = cols + 8 + floor(cols/100) (cols = nb), or marks
itself `exhausted` when all pairs are used.

Arm A_2LP (primary series): at each p, the minimum over cF ∈ {1,2,4} of the mean
cost over the 4 curves, among cells where all 4 curves are non-exhausted (rep 1).

## 5. Variables held fixed

Implementation (one Python file, stdlib, SPEC), counters, prefilter on (discriminant
quadratic character for residual degree ≥ 3), stop rule and margin, LA model, primes,
node positions, seed strings, Python version (recorded). One thread.

## 6. Metrics and unit

FPM = (field multiplications incl. squarings) + 10 × (field inversions), from the
instrumented counter of SPEC §2. Per run (JSONL row, SPEC §8): enumeration FPM,
relation FPM, LA_model FPM, total; lines tried, node lines, tangent lines,
prefilter-pass fraction, split fraction, full/1LP/2LP/discarded counts, useful,
needed, exhausted, matrix rows/cols/weight, components, N (points).
Sensitivity (reported, not used in the verdict): `total_kappa` = enum + rel + g^2·LA
(mod-r multiplication, r ≈ p^g).
Rho reference: rho_FPM = sqrt(pi · p^g / 2) · c_step, c_step ∈ {393 (lab hyperelliptic
g = 4 Cantor step, a lower bracket), 8900 (lab plane-quartic Pic addition, g = 3)},
both `[pcc:unverified]`, plus the Weil band on #J. Ratios IC_partial / rho reported per
p. The lab hyperelliptic g = 4 row (0.117 at p = 1031) is quoted beside it, marked
non-comparable (full DLP, negation credit, different curve family).

## 7. Correctness gates (must pass before any metric is read)

- G0 self-tests (SPEC §9) all pass.
- G1 (every relation, every main run, uncounted): each of the d points satisfies
  F(P) = 0 and L(P) = 0, has non-zero gradient, is not a node.
- G2 (every relation): the d points are pairwise distinct. With G1 and Bezout this
  proves the line section is exactly those d points, each of multiplicity 1.
- G3 (gate stage, per curve at p ∈ {131, 257}, d = 4 and d = 5, 2 curves each):
  Weil reciprocity f(div g) = g(div f) for f = L1/L2, g = L3/L4 on 200 quadruples of
  FB relations with pairwise disjoint supports: 200/200 must hold.
- G3-PC (positive control of the verifier): on 200 quadruples where one residual
  point of relation 1 is replaced by another curve point not on L1, reciprocity must
  fail in ≥ 190/200.
- G4 (census, gate stage and every main run with ≥ 20,000 prefilter-eligible lines):
  for d = 5 the split fraction among non-degenerate candidate lines lies within
  ±0.01 of C(p,3)/p^3; for d = 4 within ±0.01 of C(p,2)/p^2.
- G5 (union-find formula): at p = 131 and 257 (gate stage, one curve, cF = 2), the
  count Σ_components (E − V + [bipartite and ungrounded]) equals E_total − rank over
  F_ℓ, ℓ = 2^31 − 1, of the large-prime incidence matrix.

If a gate fails in the gate stage, the executor stops and reports; a SPEC
implementation bug may be fixed and the gate stage rerun (gate stage is not data).
Any G1/G2 failure in a main run = **killed (verification)**.

## 8. Decision rule (primary series d = 5, A_2LP)

Evaluated in this order:
1. Any G1/G2 failure in main runs, or G3 failing after a clean implementation → KILLED (verification).
2. Instrument checks; any failure → INCONCLUSIVE (instrument), no further verdict:
   - PC1 positive control: d = 4 A_2LP slope over all 7 primes in [0.55, 1.25].
   - PC2: d = 5 FB slope over the four largest primes ≥ 1.75.
   - AA: |slope(cF=2, rep 2) − slope(cF=2, rep 1)| ≤ 0.10 (d = 5).
3. Point slope e > 1.60 → KILLED (exponent).
4. e ≤ 1.45 and bootstrap 97.5% bound ≤ 1.60 → SUPPORTED.
5. Otherwise → INCONCLUSIVE.

Fallback fixed now: if a p = 8209 (or 4099) cell times out or every cF is exhausted
there, the fit uses the remaining primes (at least 5), and the report says so.
Secondary (reported, no verdict): slope over the 4 largest primes; slopes per fixed
cF; FB slope; IC_partial/rho per p; optional g = 5, 6 slopes against 3/2.

Bootstrap: 2000 replicates, `random.Random("L08-boot")`, per p resample the 4 curves
with replacement (the same index draw applied to every cF cell of that p), recompute
the min over cF and the slope; percentiles 2.5 / 97.5.

## 9. Confounders and isolation

| confounder | isolation |
|---|---|
| pair order / splitting randomness | A/A arm (rep 2), threshold AA |
| factor-base size tuning | fixed grid cF ∈ {1,2,4}, same at every p, minimum rule fixed now; per-cF slopes reported |
| small-p set-up dominance (enumeration ~ p log p) | slope on 4 largest primes reported; enumeration share reported |
| LA is a model, not executed | same model in every arm; FB/2LP compared under it; κ sensitivity |
| curve population (split quadric, rational nodes) | disclosed (§1.3); same generator in all arms |
| genus test conservative (rejects non-generic projections) | rejection counts by reason reported |
| exhaustion bias in min over cF | only cells with 4/4 non-exhausted curves enter |
| rho step cost unknown for non-hyperelliptic Pic | bracketed 393 / 8900, labelled |
| analyst | analysis is the frozen `analyze` subcommand; no manual selection |

## 10. Commands (executor; 1 thread; each wrapped in `timeout 600`; outputs under raw/)

Run from the lead directory `/home/user/crypto/research/g2plus_leads_20261116/L08_nonhyperelliptic_ladder_g4_g6`.

```
python3 --version > raw/python_version.txt
timeout 600 python3 code/l08.py selftest > raw/selftest.txt
timeout 600 python3 code/l08.py gate --d 5 --m 2 --primes 131,257 --curves 2 --out raw/gate_d5.jsonl
timeout 600 python3 code/l08.py gate --d 4 --m 0 --primes 131,257 --curves 2 --out raw/gate_d4.jsonl
# stage 1 main runs (one command per (d, p); for p >= 4099 one per curve index)
for p in 131 257 521 1031 2053; do
  timeout 600 python3 code/l08.py run --d 5 --m 2 --p $p --curves 0,1,2,3 --out raw/run_d5_p$p.jsonl
  timeout 600 python3 code/l08.py run --d 4 --m 0 --p $p --curves 0,1,2,3 --out raw/run_d4_p$p.jsonl
done
for p in 4099 8209; do for c in 0 1 2 3; do
  timeout 600 python3 code/l08.py run --d 5 --m 2 --p $p --curves $c --out raw/run_d5_p${p}_c$c.jsonl
  timeout 600 python3 code/l08.py run --d 4 --m 0 --p $p --curves $c --out raw/run_d4_p${p}_c$c.jsonl
done; done
timeout 600 python3 code/l08.py analyze --glob 'raw/run_d*.jsonl' --out raw/analysis.json --table raw/analysis.tsv
# optional stage 2 (only after stage 1 is complete)
for p in 131 257 521 1031 2053; do
  timeout 600 python3 code/l08.py run --d 6 --m 5 --p $p --curves 0,1,2,3 --out raw/opt_d6m5_p$p.jsonl
  timeout 600 python3 code/l08.py run --d 6 --m 4 --p $p --curves 0,1,2,3 --out raw/opt_d6m4_p$p.jsonl
done
timeout 600 python3 code/l08.py analyze --glob 'raw/opt_d6*.jsonl' --out raw/analysis_opt.json --table raw/analysis_opt.tsv
sha256sum raw/* > raw/data.sha256
```

A `timeout` exit (124) is recorded in `raw/timeouts.txt` as `<command> 124`; the
cell is missing and the §8 fallback applies. A repository hook refusing a command is
recorded verbatim in `raw/hook_refusals.txt`; it is not worked around.
