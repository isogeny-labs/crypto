# Replacement text for the 2026-11-04 program document

The document is not present in this checkout, so these are supplied as text. Each patch gives the
original passage (verbatim from the circulated 27-page PDF/Markdown) and the replacement.
`apply_patches.py` applies them literally to a specified file, reports which ones matched, and
leaves the file untouched if any patch fails (`--dry-run` to check).

Patch ids map to REVIEW.md: P1/P2 → §3, P3 → §5, P4 → §6, P5 → §4, P6–P8 → consistency.

---

## P1 — §2, arity row (B values confused with the T = 1 point)

**Original**
> m = 4 needs oracle cost ≪ p^{1/4} at B = p^{1/5}; m = 5 iff T_dec ≪ p^{1/2} at B ≈ p^{1/6}.

**Replacement**
> m = 4 needs oracle cost ≪ p^{1/4} at B ≈ p^{1/4} (at the T = 1 point B = p^{1/5} the condition is
> T_dec < 1); m = 5 iff T_dec ≪ p^{1/2} at B ≈ p^{1/4}. In general the win region is
> T_dec < B^{m+1}/p with B ≲ p^{1/4}.

**Why**: at `B = p^{1/5}` the two terms of the cost model balance only when `T_dec = O(1)`. The
`p^{1/4}` threshold is attained at `B = p^{1/4}`. Both numbers are right; the sentence joins them.

---

## P2 — §3.1, the gate (the load-bearing error)

**Original**
> In the lead-review cost bridge the same thing reads: oracle exponent e < m/2 − 1 and amortised
> exponent a < m/2. Any proposal in Section 4 is judged by whether it can plausibly produce an
> oracle with e < 1 at m = 4 (or e < 0.5 at m = 3 as the tie milestone), and by nothing else.

**Replacement**
> In the lead-review cost bridge the same thing reads: with T_dec ∝ B^s the win condition is
> **s < m − 3**, equivalently **e < (m−3)/4** when e is measured at the tie point B = p^{1/4}
> (T_dec = p^e). Concretely: e < 0 at m = 3 (only an O(1) oracle ties), e < 1/4 at m = 4, e < 1/2
> at m = 5. Any proposal in Section 4 is judged against these thresholds, and by nothing else.
> The weaker gate "e < 1 at m = 4" must not be used: it is satisfied by the generic 2+2
> meet-in-the-middle oracle (T = B², e = 2/5 at B = p^{1/5}, measured s = 1.93), whose optimal
> total cost is p^{2/3}, worse than rho.

**Why**: the stated gate is not implied by the document's own cost model and accepts an algorithm
that is provably worse than rho. See REVIEW.md §3 and `experiments/mitm_oracle.py`.

---

## P3 — §4, Idea 2 (Vieta-conic sieve)

**Original (Mechanism)**
> Enumerating conic points with small e_1 is a lattice problem only when the conic is degenerate;
> in general it costs B. The new element is to sieve the square condition first: for each candidate
> u ≤ 2B (so e_2 = (e_1² − u²)/4), the constraint becomes a univariate quadratic in e_1 modulo p
> with the small root e_1 ≤ 2B, which univariate Coppersmith solves rigorously for e_1 ≤ p^{1/2}.

**Replacement**
> Enumerating conic points with small e_1 is immediate: Q_R is quadratic in e_2, so each
> e_1 ∈ [1,2B] yields at most two points and the enumeration costs O(B) field operations. The
> square test e_1² − 4e_2 = u², x_1,2 = (e_1 ± u)/2 ∈ [1,B] is O(1) per point. Measured
> (2026-11-04): planted 2-decompositions are recovered at B = p^{1/2} in O(B) time (5/5 at
> p = 2^28, 8.4·10^4 e_1-steps, 9.4 s), with no lattice. Note that substituting
> e_2 = (e_1² − u²)/4 into Q_R gives a **quartic** in e_1, so univariate Coppersmith reaches
> e_1 ≤ p^{1/4}, not p^{1/2}.

**Original (Prediction)**
> The batch lattice over (e_1, u) is a bivariate (2,2)-type problem again (genus 1 returns), so the
> certified reach will be 1/6 in e_1, i.e. no gain; the sieve-by-u variant will measure exponent 1.0
> in B.

**Replacement**
> The direction is m = 2. By §2 an m = 2 oracle cannot beat rho even if it is free, so no outcome
> here can change the exponent. The enumeration above is already the O(√p) generic bound, and the
> batch-(e_1,u) lattice can at best produce a certified sub-1/6 lemma. The honest deliverable is a
> lemma, not an oracle: "the symmetric quotient does not lower the lattice reach."

**Original (Experiment/Criterion)** — replace the u-sieve production run with a single certificate;
mark the direction closed. Keep the (a) certificate task only if the Idea 1 stack is running anyway.

**Why**: REVIEW.md §5. The measured O(B) algorithm removes the premise that a lattice is needed.

---

## P4 — §4, Idea 3 (Γ₀(p) census)

**Original (Mechanism, last sentence of the "decomposition is:" paragraph)**
> Forms with a given root mod p and given discriminant Δ form finitely many orbits under the
> stabiliser of x_R in SL_2(Z), a conjugate of Γ_0(p).

**Replacement**
> With D = 16 f(x_1) f(x_2) > 0 the forms are indefinite, so SL_2(Z)-orbits are infinite; what is
> finite is the set of Γ₀(p)-reduced representatives of bounded size, equivalently the class-group
> cosets meeting the root constraint.

**Original (Experiment (a))**
> (a) Exact census: for 200 random targets, enumerate all vectors of L_R with |c_i| ≤ B^4, count
> those with c_2 = □, with D ∈ S_B, with both; compare to the decomposition count.

**Replacement**
> (a) Exact census, at a size where it exists: the box |c_i| ≤ B^4 in the 3-dimensional lattice of
> determinant p contains ≈ 8B^{12}/p points, so a census is tractable only for δ ≲ 0.1 (O(1) points
> at δ = 1/12, p² at δ = 1/4). Run it at p = 2^20…2^32 with δ ≤ 0.10 and state explicitly that this
> is below the LLL threshold; do **not** attempt a census at δ ≈ 1/5.

**Why**: REVIEW.md §6. The census as written is infeasible exactly in the regime it is meant to
probe, and the orbit statement is false for indefinite forms.

---

## P5 — §4, Idea 10 (squarefree collisions) — close it

**Original (Prediction)**
> The number of pairs x_1 < x_2 ≤ B with equal squarefree part of f(x) is B^{1/2+o(1)} (heuristic:
> f(x) behaves like a random integer of size B³ whose squarefree part is a random divisor), so the
> harvest is B^{1/2} relations against the B needed, and it vanishes relative to the relation phase.

**Replacement**
> Measured 2026-11-04: exactly **one** colliding pair for every B ≤ 2^16 (x = 2, 212, kernel 15:
> 212³ + 7 = 15·797²). The birthday heuristic is wrong: a collision is an integer point on
> x³ + 7 = d·m², so per d there are finitely many (Siegel), and Σ_d C(k_d,2) ≈ (log B)/B rather than
> B^{1/2}. At B = 2^12 the predicted 64 pairs are 1. The direction is closed: it re-derives the
> global-lift/free-relation wall of RESEARCH_FREE_RELATIONS_CENSUS_20261012.md.

**Experiment/Criterion** — delete the 20-pair PARI step; record the census table (256…65536 → 1,1,1,1,1)
as the closing measurement, status `closed-obstructed`, not `open`.

**Why**: REVIEW.md §4 and `scripts/sqf_census.py`.

---

## P6 — §5.4, what a win looks like

**Original**
> An exponent change requires either a certified lattice or auxiliary-polynomial reach of at least 1/5
> at m = 4, or a measured oracle exponent e < 1 at m = 4 (e < 0.5 at m = 3 for a tie).

**Replacement**
> An exponent change requires either a certified lattice or auxiliary-polynomial reach of at least 1/5
> at m = 4, or a measured oracle cost T_dec = B^s with s < 1 at m = 4 (e < 1/4 at the tie point
> B = p^{1/4}; s < 2 at m = 5, s < 3 at m = 6). At m = 3 only an O(1) oracle ties, so m = 3 cannot
> produce a win. A generic 2+2 meet-in-the-middle oracle has s = 2 and does not qualify.

**Why**: consistency with P2; prevents a generic oracle from being scored as a win.

---

## P7 — §3.5, grouping sentence

**Original**
> Ideas 5, 9 and 10 attack (b).

**Replacement**
> Ideas 5 and 9 attack (b). Idea 10 is closed (measured, see §4).

---

## P8 — §4.1 summary table, rows 2, 3, 10

Replace the three rows with:

| # | Direction | Attacks | Expected outcome | Decisive number | Cost |
|---|---|---|---|---|---|
| 2 | Vieta-conic sieve + square condition | oracle (m=2) | **closed as a lead**: O(B) enumeration is the generic bound; lemma only | measured O(B) recovery at B=p^{1/2} | done |
| 3 | Γ₀(p) forms / representation numbers | oracle (m=2) | census only below δ≈1/12; lemma | census ratio at δ ≤ 0.10 | 0.5 CPU-d |
| 10 | Squarefree-part global relations | relations | **closed**: 1 pair ≤ 2^16 vs B^{1/2} predicted | collision count | done |

---

## Consistency note (no patch needed)

§3.1's "the only known algebraic oracle … reach is 1/(m(m+1)2^{m−2})" and the `1/6, 1/24, 1/80`
values are internally consistent (m = 2, 3, 4) and are not affected by the gate error. §2's
"m = 4 needs ≪ p^{1/4}" was already correct; P1 only fixes the accompanying B.
