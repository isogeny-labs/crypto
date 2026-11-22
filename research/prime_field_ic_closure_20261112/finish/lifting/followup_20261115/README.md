# Lifting follow-up (item 4): Silverman–Suzuki with growing rank, and concrete-size lifting at r ≥ 7

**Standing.** Everything here is `[pcc:unverified]`. No pcc claim was filed. No `bench.record/v1` was written. Nothing ran on EKS, no resume checkpoint was made, and nothing was committed.
- **Where it ran.** Local toy runs on the shared 4-vCPU VM, with PARI/GP 2.17.2 through cypari2 2.2.4 (installed into the scratchpad) and python-flint 0.9.0.
- **Prior art was not checked.** research-kb is not reachable here (`scripts/research-kb status`: "no executable found"). Prior-art statements rest on the repo corpus files only (`docs/ecdlp-hardness-corpus/`).
- **Literature tags.** `[L-recalled]` marks literature quoted from memory and not re-read. That covers Hermite constants for k ≤ 8, Blichfeldt's bound, Silverman's specialization theorem, and the family ranks.
- **Inputs.** This builds on `research/prime_field_ic_closure_20261112/finish/lifting/PROOF.md` and the closure note §5 (2026-11-12).

## 0. Verdicts

| Sub-route | Verdict | Strength |
|---|---|---|
| **Concrete size, r ≥ 7 at 256 bits, conditional on a height-ratio certificate ρ = H/λ** | **Excluded for r = 7, 8, 9 if ρ ≤ 32.6, 10.8 and 4.94.** The thresholds come from a new packing lemma (Lemma 3H, §1). The R5 bound needed ρ ≲ 1 at r = 7 and failed at r = 8 and 9 for every ρ ≥ 1. The new cubic-path toy measures ρ_G at r = 7, 8 at 8–16 bits: medians 1.086 / 1.044, maxima 1.644 / 1.42. Those maxima give 256-bit margins of +90.7 and +82.0 bits. | D (paper proof of Lemma 3H) + T (toy, 2,100 trials) |
| Concrete size, r ≥ 7 at 256 bits, **unconditional** | **Open.** Petsche's λ bound is far too weak: under the cubic-toy fits the unconditional thresholds are 7,669 bits (r = 7) and 10,803 bits (r = 8). At 256 bits, Lemma 3H needs λ ≥ 40.6 nats (r = 7) and λ ≥ 268 nats (r = 8) against the extrapolated H(256), and nothing proves that. | D |
| Concrete size, r = 2, unconditional | Excluded at 256 bits under the R5 lift-size fits (margin +7.0 bits, threshold 241 bits, down from 265). **Not** excluded under the cubic-path fits (margin −7.5, threshold 273 bits). The result depends on the construction. | D + extrapolated fits |
| SS, enumeration/table lifting (lifts found by enumerating E′(Q) points or any known-log table) | **Ruled out:** expected cost ≥ √(2q) group operations, about 1.6× rho with the negation map (§2.2) | D (short argument) |
| SS with fixed rank, or s = o(log p / log log p), and lifts of polynomial height | **Ruled out asymptotically** (PROOF Prop 4 + Petsche, restated). The toy confirms X* ∝ q^{1/s} on 8 curves of rank 1–12. | D + T |
| SS through a family of fixed generic rank s₀ specialised to reach E | **Ruled out asymptotically.** Heights scale with h(t) ≈ ½ log p, so the needed X/λ grows like q^{1/s₀} (§2.3). **Not excluded at 256 bits by counting:** the probability bound alone needs only s₀ ≥ 13–31 for X/λ = 10⁶–10². | D, with one `[L-recalled]` input |
| SS by **direct** bounded-height lifting with s = Ω(log p / log log p) | **Open.** No unconditional rank bound excludes such curves, and no lower bound is known for lifting a residue class to a bounded-height rational point. Huang–Kueh–Tan (abstract only) and Cheng–Huang (corpus, conditional) bear on this but were not read. | — |

**Distance remaining to matched rho:** unchanged at the exponent level. Every excluded case costs at least the matched √q. No lifting variant measured here comes close to rho. The open cells are open because no lower bound is known, not because any data points toward rho.

## 1. Concrete sizes: the Hermite circuit bound (Lemma 3H)

### 1.1 Statement and proof (paper; not machine-checked)

**Setup.** Take P′₁,…,P′_r ∈ E′(Q) and set:
- H ≥ max ĥ(P′_i);
- 0 < λ ≤ ĥ(x) for every non-torsion x ∈ E′(Q);
- ρ = H/λ;
- γ_k, the Hermite constant in dimension k.

**Lemma 3H.** If the P′_i are Z-dependent, there are a set J ⊆ {1..r} with |J| = k+1 and 0 ≤ k ≤ r−1, and a vector n supported exactly on J, such that:
- Σ n_i P′_i ∈ E′(Q)_tors;
- |n_i| ≤ (γ_k ρ)^{k/2} for every i.

*Proof.*
1. Let Λ = E′(Q)/tors with the Euclidean norm √ĥ, and let v_i be the image of P′_i.
2. **Case k = 0.** If some v_i = 0, take n = e_i.
3. **Choosing a circuit.** Otherwise take a minimal dependent subset J, of size k+1. The v_j for j ∈ J span a lattice L of rank k. By minimality, each L_{−i} = ⟨v_j : j ≠ i⟩ also has rank k.
4. **The kernel vector.** Let A be the integer (k+1)×k coordinate matrix of v_J in a basis of L. Set c_i = (−1)^i det A_{−i}. Then c^T A = 0, by Laplace expansion of a matrix with a repeated column. Also c_i ≠ 0, and |c_i| = [L : L_{−i}] = covol(L_{−i}) / covol(L).
5. **Bounding the numerator.** By Hadamard, covol(L_{−i}) ≤ Π_{j≠i} √ĥ(P′_j) ≤ H^{k/2}.
6. **Bounding the denominator.** By Hermite, covol(L) ≥ (λ₁(L)²/γ_k)^{k/2} ≥ (λ/γ_k)^{k/2}.
7. **Conclusion.** Hence |c_i| ≤ (γ_k ρ)^{k/2}, and Σ c_i v_i = 0 means Σ c_i P′_i is torsion. ∎

**Reduction step (needs q > 12, no torsion bound T).**
- Σ n_i P_i = red(Σ n_i P′_i) lies in ⟨S⟩, by Lemma 1 of PROOF.md.
- Its order divides the order of a torsion point of E′(Q), which is at most 12 (Mazur).
- Since q is a prime greater than 12, the reduction is O.

So the factor T = 16 in R5's cutoff disappears. The per-coordinate factor (2r)^{r−1} also disappears.

**Theorem 1H.** Under the hypotheses of PROOF Theorem 1, with fresh uniform masks:
- Pr[dependent] ≤ N(r,ρ)/q, where N(r,ρ) = ½ Σ_{j=1}^{r} C(r,j)·(2⌊(γ_{j−1}ρ)^{(j−1)/2}⌋)^j;
- E[#trials] ≥ q/(2N).

*Proof.* Union over the events of Lemma 2, each of probability 1/q, with ±n identified.

**Notes.**
- The exponent of ρ is the same as in R5 (ρ^{r(r−1)/2}), so the asymptotic Theorem 2 does not change.
- The improvement is in the constants only.
- Inputs `[L-recalled]`:
  - γ_k is exact for k ≤ 8: γ₂² = 4/3, γ₃³ = 2, γ₄⁴ = 4, γ₅⁵ = 8, γ₆⁶ = 64/3, γ₇⁷ = 64, γ₈ = 2.
  - For k ≥ 9, the code uses Blichfeldt's upper bound γ_k ≤ (2/π)Γ(2+k/2)^{2/k}. It is rigorous but loose.

### 1.2 Numbers at 256 bits

Margin = log₂(E[#trials]) − b/2, so a positive margin means the case is excluded. The source is `out/bounds_hermite.json`; the R5 box cutoff was recomputed under the same convention.

| r | ρ_max (Hermite) | margin at ρ = 1.065 (Hermite / R5 box, T = 1 / R5 box, T = 16) | margin at ρ = 4 (Hermite / box, T = 16) |
|---|---|---|---|
| 6 | 185.3 | +112.3 / +43.1 / +19.1 | +83.0 / −9.5 |
| 7 | **32.6** | +104.7 / +1.2 / −26.8 | +63.6 / −66.9 |
| 8 | **10.8** | +94.6 / −50.5 / −82.5 | +40.0 / −136.0 |
| 9 | **4.94** | +80.1 / −112.5 / −148.5 | +11.0 / −217.2 |
| 10 | 2.75 | +61.6 | −24.4 |
| 12 | 1.35 | +22.6 | — |
| 13 | 1.05 | −1.1 | — |
| ≥ 14 | none (negative already at ρ = 1) | — | — |

**Unconditional (Petsche) thresholds with Lemma 3H.** The R5 fits (affine lifts, r = 2..6) give **241, 795, 1,690, 3,001 and 4,817 bits** for r = 2..6. R5 had 265, 842, 1,773, 3,134 and 5,015.
- At r = 2, the 256-bit margin is +7.0 bits.
- Under the cubic-path fits (non-minimal Δ, which is conservative), the thresholds are 273, 899, 1,913, 3,351, 5,237, **7,669 (r = 7)** and **10,803 (r = 8)** bits.

### 1.3 Toy: cubic-path xedni for r = 2..8 (`code/xedni_cubic.py`)

The R5 toy used a flex-at-O Weierstrass fit, which stops at r = 6. This toy uses the general plane cubic with Y³-coefficient 0, so that O = (0:1:0) lies on it. That gives 9 monomials, so up to 8 lifted points plus O.

**Per trial:**
1. **Lifts.** Projective lifts of size about p^{2/3}, from LLL.
2. **Cubic.** The shortest cubic u ≡ c·u_E (mod p).
3. **Weierstrass model.** Projection from O gives a quartic (2L₁y + Q₂)² = Q₂² − 4L₁K₃. That quartic is shifted to O's tangent parameter and mapped to Weierstrass form by Washington Thm 2.17. Every image point is checked exactly on the curve.
4. **Measurements.** The R5 relation sieve, and Néron–Tate Gram matrices by the doubling limit with 5 doublings. λ_G is the exact Fincke–Pohst minimum of the Gram form.

**Grid.** Bits 8, 10, 12, 14, 16 × r = 2..8, with 3 curves × 20 trials per cell: 2,100 trials, 1,981 of them "ok". The skips are:
- `tangent-at-O-is-Z=0`: 63;
- repeated point: 36;
- point at infinity: 16;
- no coset: 2;
- q0 = 0: 2.

**Detector positive control (arm DET).** P′₃ was replaced by P′₁ + P′₂ computed over Q, and the sieve returned exactly ±(1,1,−1,0…) on **45/45** trials (r = 3, 5, 8; 10 and 16 bits). Three trials were skipped by status.

**Planted mod-p arm (PS: P₃ = P₁ + P₂ in E(F_p), lifted independently).** The relation survived the lift in 2 of 26 "ok" trials at 8 bits (one each at r = 3 and r = 5) and in **0/60** trials at 12–16 bits.

**Honest arm.**
- **Relations.** There were 2 non-trivial relations among 1,981 trials, both at 8 bits: (−1,−1,1,0) at r = 4 and (−1,1,0,1,0) at r = 5. There were **0 at 10–16 bits**.
- **Certificates.** Relation-free trials carry a "no relation with ‖n‖∞ ≤ 2^x" certificate. The smallest x per cell ranged from 11.7 (r = 8, 10 bits) to 38.8.

| r | H(b) fit (nats) | ρ_G median | ρ_G p90 | ρ_G max (8–16 b) | ρ_G max (12–16 b) | margin at 256 b, ρ = max | margin at 256 b, ρ = 4 × max |
|---|---|---|---|---|---|---|---|
| 6 | −9.11 + 3.115 b | 1.111 | 1.220 | 1.617 | 1.258 | +103.0 | +72.6 |
| 7 | −19.75 + 5.248 b | 1.086 | 1.169 | 1.644 | 1.366 | **+90.7** | +48.5 |
| 8 | −46.02 + 11.444 b | 1.044 | 1.091 | 1.420 | 1.099 | **+82.0** | +25.8 |

How to read this table:
- ρ_G falls with bit size: the r = 8 maximum is 1.42 over 8–16 bits but 1.099 over 12–16 bits.
- Trials with a relation have no defined ρ, and both of them are excluded from the ρ statistics.
- **Measurement defect found and corrected.** On 17 trials, `heights.min_form` (R5 code, tolerance 3·r·ε) returned the smallest eigenvalue instead of the lattice minimum, which raised ρ_G to as much as 16.2. Fincke–Pohst re-enumeration (`code/fix_singular.py` → `out/runs_v2/`) gave ρ_G between 1.25 and 1.64 on the 15 relation-free cases. The same tolerance rule is in the R5 toy, so some of its "ρ_G max 2.68" values may be eigenvalue bounds rather than minima. That direction is pessimistic for the bound. This was not re-checked here.

### 1.4 Calibration: λ_G against the true Mordell–Weil minimum (`code/lambda_calib.py`)

**Why it is needed.** λ_G is at least the true λ, which is the gap the R5 checker flagged.

**Method.** Shortest integral cubics through r random small projective points (|X|,|Y| ≤ e, 1 ≤ Z ≤ e) plus O, then:
- PARI `ellminimalmodel`;
- `ellrank` with the points supplied;
- `ellsaturation` with B = 100;
- the minimum of the full Mordell–Weil height form, by `qfminim`.

Only curves with rank lower bound = upper bound count. r = 8 ran into the 90-s limit, so it is absent. **The run was stopped before it covered e = 16 and 32**; see §4.

**Results** (`out/lambda_calib_analysis.json`; e = 4 for r = 2..7, e = 8 for r = 2..6, and 6 trials at e = 8, r = 7; 171 records):
- **λ_MW / λ_G:** 80 usable pairs. Median 1.000, below 1 in 34%, 10th percentile 0.561, **minimum 0.279**.
- **ρ_MW / ρ_G:** at most **3.58**, and at most 2.27 for r ≥ 3.
- **Rank above r** is common: 1–9 trials per cell (cells of 12–15 usable trials).
- **Lang ratio** λ_MW / log|Δ_min|: per-cell minima 0.001–0.026, per-cell medians 0.016–0.040.
- **Applied to the toy:** scaling the toy's ρ_G maxima by 3.58 gives 256-bit margins of +51.8 (r = 7) and +30.3 (r = 8), but **−1.5 at r = 9** (ρ = 5.09).

**Caveat.** These curves are far smaller than real lifts, so this is a sanity check, not a certificate.

## 2. Silverman–Suzuki lifts with growing rank

### 2.1 Coverage toy (`code/ss_coverage.py`)

**Curves.** Eight curves over Q. Their ranks were re-derived by PARI `ellrank` with lower bound = upper bound, and the generators were saturated with B = 200:

| curve | rank | Weierstrass coefficients |
|---|---|---|
| 37a | 1 | [0,0,1,−1,0] |
| 389a | 2 | [0,1,1,−2,0] |
| 5077a | 3 | [0,0,1,−7,6] |
| r4 | 4 | [1,−1,0,−79,289] |
| r5 | 5 | [0,0,1,−79,342] |
| r6 | 6 | [1,1,0,−2582,48720] |
| r8 | 8 | [1,−1,0,−106384,13075804] |
| r12 | 12 | [0,0,1,−6349808647,193146346911036] |

The coefficients were recalled from memory. Two of the recalled rank labels were wrong, which ellrank caught, and three candidates turned out to have rank 0 or 1 and were dropped.

**Measurement.** For 21 primes per curve with #E′(F_p) = q prime, at 12–36 bits:
- the image of the height ball B_X = {n : nᵀGn ≤ X} under reduction was computed by discrete logs of the reduced generators;
- X* is the smallest X whose image has at least √q points.

**Results** (`out/ss_analysis.json`):
- **Slope.** The slope of log X* against log q is 1.0006, 0.4986, 0.3336, 0.2503, 0.2007, 0.1672, 0.1264 and 0.0847 for s = 1, 2, 3, 4, 5, 6, 8, 12. That is, X* ∝ q^{1/s}.
- **No collisions.** The image size equals N(X*) exactly: 0 collisions in 168 cells, against 84.2 expected for a random map.
  - **Why:** the differences of lattice-ball points stay inside a ball of radius 2√X. A collision needs a vector of the index-q relation lattice there, and that lattice's minimum is about q^{1/s}·X*.
  - So the per-target hit probability is exactly N(X)/q at these sizes, which is the packing bound's quantity.
- **Gaussian volume.** N(X*) over the Gaussian volume V_s X^{s/2}/√det G lies between 0.69 and 1.46.
- **256-bit extrapolation** (Gaussian, with each curve's own Gram matrix): X*/λ ≈ 1.3·10⁶ (s = 12), 2.3·10⁹ (s = 8) and 3.7·10¹² (s = 6).

### 2.2 Enumeration/table SS is ruled out (paper)

**Model.** Suppose an algorithm finds lifts by computing the reductions of M points of E′(Q), or of any precomputed table, and then tests targets R = aS + bT against it. Each table point costs at least one group operation; the negation map gives ±P for one operation, so the table holds at most 2M points. Each target costs at least one operation.

**Argument.** With fresh uniform targets, Pr[hit] ≤ 2M/q per target. So:
- E[cost] ≥ M + q/(2M) ≥ √(2q);
- when the generators' logs are unknown and s+1 hits are needed, E[cost] ≥ √(2(s+1)q).

**Comparison.** √(2q) ≥ 1.59 × √(πq/4), the expected cost of rho with the negation map. This is a generic table bound, and it covers any SS variant whose lifting step is enumeration.

### 2.3 Families and growing rank (paper)

**The counting condition.** Lifts of height ≤ X can beat √q only if the rank is large enough. Rigorously (packing, T = 16), s·log₂(1+2√(X/λ)) ≥ b/2 − 4.

At b = 256 the minimum ranks are:

| X/λ | s (packing, rigorous) | s (Gaussian heuristic with γ_s) |
|---|---|---|
| 2 | 69 | 123 |
| 10 | 46 | 57 |
| 100 | 31 | 33 |
| 10³ | 22 | 23 |
| 10⁴ | 18 | 18 |
| 10⁶ | 13 | 12 |

**Asymptotics.** With λ ≥ (log p)^{−O(1)} (Petsche) and X = poly(log p), the condition needs s = Ω(log p / log log p) (PROOF Prop 4).

**A fixed family cannot supply that.** To land on a prescribed E mod p, the curve must be a specialisation E_t of a family over Q(t), with j(E_t) ≡ j(E). A generic solution has h(t) ≈ ½ log p (rational reconstruction).
- By Silverman's specialization theorem `[L-recalled]`, the specialised section lattice is about h(t)·G_fam.
- Then X/λ and the count N(X) do not depend on p, while the required count √q grows.
- So for a family of fixed generic rank s₀, the needed X/λ grows like q^{1/s₀}: exponential in b/s₀. This is ruled out asymptotically.
- Any extra rank of individual specialisations has no known structure or bound.

**At 256 bits** the counting bound alone does not exclude a family of rank s₀ ≥ 13–31 with X/λ = 10⁶–10². I recall the largest known family ranks over Q(t) as about 18–19 `[L-recalled]`. What stops this route is the lifting step itself:
- Enumeration is excluded (§2.2).
- Direct lifting of a given residue class to a bounded-height point has no known algorithm and no known lower bound.

## 3. Commands (all run 2026-11-15; F = `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting`)

```
git -C /home/user/crypto pull -q origin main
pip download cypari2 --no-deps -d $SCR/pipdl ; pip install --target $SCR/pylib $SCR/pipdl/cypari2-2.2.4-*.whl
python3 $F/code/bounds_hermite.py $F/out/bounds_hermite.json
xargs -P 3 -L 1 python3 $F/code/xedni_cubic.py < $F/out/grid.txt          # 35 honest cells + 9 PS cells
python3 $F/code/xedni_cubic.py --bits {10,16} --r {3,5,8} --arm DET --curves 1 --trials 8 --heights 0 --out $F/out/runs_det.jsonl
python3 $F/code/fix_singular.py $F/out/runs $F/out/runs_v2
python3 $F/code/analyze_cubic.py $F/out/runs_v2 $F/out/cubic_analysis.json
python3 $F/code/bounds_hermite.py $F/out/bounds_hermite_cubic.json $F/out/cubic_fits.json
PYTHONPATH=$SCR/pylib python3 $F/code/ss_coverage.py --bits 12,16,20,24,28,32,36 --per 3 --out $F/out/ss_coverage.jsonl
python3 $F/code/ss_analyze.py $F/out/ss_coverage.jsonl $F/out/ss_analysis.json
PYTHONPATH=$SCR/pylib python3 $F/code/lambda_calib.py --r 2,3,4,5,6,7 --e 4,8,16,32 --n 15 --tlimit 90 --out $F/out/lambda_calib.jsonl
python3 $F/code/analyze_calib.py $F/out/lambda_calib.jsonl $F/out/lambda_calib_analysis.json
python3 $F/code/plot_margins.py $F/out/bounds_hermite.json $F/out/margins_256.svg
```

## 4. Still open or blocked

1. **The height-ratio certificate is unproved.** Excluding r = 7–9 at 256 bits still needs λ_true ≥ H/ρ_max, which is about 41 nats for r = 7 and 268 nats for r = 8 against the extrapolated H(256). That is a Lang-type statement with an explicit constant, and no theorem supplies it.
   - The toy ρ_G values are from 8–16-bit lifts, extrapolated to 256 bits.
   - The calibration covers small curves only, and stopped early: e = 16 and 32 were not reached, and r = 8 timed out.
   - With the calibration's worst ratio (3.58), r = 9 goes negative (−1.5 bits).
2. **r = 9 has no toy.** The cubic path with O as one of the points allows at most r = 8. r = 9 needs an origin outside the lifted set, which is not built.
3. **SS direct lifting is open.** That is the bounded-height lifting of a residue class, i.e. the partial-lifting problem. Huang–Kueh–Tan (corpus `THM-XT-HKT-LIFT-RANK`) and Silverman–Suzuki 1998 were not read, so their exact statements were not compared.
4. **Model sizes.** The cubic-toy Δ values are non-minimal: PARI's minimal model takes 30–75 s at r = 8 and was not run. So the Lang ratios and the unconditional thresholds for r = 7, 8 are conservative. The minimal models in the calibration are from PARI.
5. **Not re-checked.** The R5 toy may share the min_form tolerance defect (§1.3).
6. **No checks.** Lemma 3H is a paper proof, with no Lean and no adversarial check. Hermite constants, Blichfeldt's bound, the specialization theorem and the family ranks are recalled, not re-read.
