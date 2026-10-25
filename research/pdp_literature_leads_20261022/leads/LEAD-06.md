# LEAD-06 — Trace-zero variety representation (Gorla–Massierer) as the coordinate system for E(F_{q^n}), n = 3, 5

**Verdict.** closed-by-theorem. T_n exists only for a curve defined over F_q, so it
cannot be a coordinate system for the repository's baseline instances (random
prime-order E/F_{p^k}). Where it does apply, n = 3 ties rho's exponent and carries a
per-residual cost floor; n = 5 is the same exponent family as the repository's k = 4
ladder with a larger system. The n = 5 half rests on comparison with measured k = 4
data, not on a theorem (§5).

**Confidence.** high that it yields no measured improvement over the current baseline
rows (the instance-class obstruction is elementary and the n = 3 exponent tie is
quoted from the source and re-derived); medium for the n = 5 asymptotics, because
nobody has solved the 9-equation, 8-unknown T_5 system.

**Expected gain if it works.** None against any baseline row. For the T_3 instance
class the distance to matched rho is about 2·10³–10⁴ (relation phase only, unverified
desk estimate in the repository's unit) or about 10²–5·10² (Gorla–Massierer's own
Magma timings, wall clock), flat in q. The k = 3 Gaudry baseline's relation phase is
about 33× rho at p ≈ 2^20 and falls as p^{-1/2} (desk, from §11.3 of the residual-walks
note with the post-§11.6 C₃). The one real positive is a per-residual constant that does
not transfer to generic curves (§4).

**Cost of the discriminating test.** 2–3 person-days (no T_3 code exists; about 600 lines
of Rust beside `gaudry_cubic.rs`), at most 2 CPU-hours, under 1 GB. The expected
outcome is the kill; a 0.5-day structure-only subset (§6 steps 1–2) already settles the
card's own question.

## 1. Mechanism (precise)

**1.1 Objects.** For E/F_q and prime n, Tr: E(F_{q^n}) → E(F_q), P ↦ P + φP + … +
φ^{n−1}P, and T_n = ker Tr. T_n is the F_q-points of an (n−1)-dimensional abelian
subvariety V_n of the Weil restriction. There is an exact sequence 0 → E(F_q) → E(F_{q^n})
→ T_n → 0 (map φ − 1), so |T_n| = #E(F_{q^n})/#E(F_q) ≈ q^{n−1} and the DLP in E(F_{q^n})
is as hard as in T_n [GM-IC, Prop. 2.3].

**1.2 gcd condition and the prime-order subgroup.** A point P of prime order ℓ lies in
T_n iff Tr P = O. If gcd(ℓ, #E(F_q)) = 1, then Tr P has order dividing 1, so P ∈ T_n.
Derived here: with α the Frobenius, #T_n = N(1 + α + … + α^{n−1}), so a prime ℓ | #E(F_q)
divides #T_n iff ℓ = n or ord_ℓ(q) = n; otherwise the ℓ-part of E(F_{q^n}) lies in E(F_q)
and misses T_n. The "only if" direction was desk-checked on 2,996 random (q, E, n ∈ {3,5})
pairs with no exception (unverified desk check). Any ℓ > q + 1 + 2√q satisfies the gcd
condition, so it is automatic for the large prime. Two consequences. First, #E(F_{q^n}) is
never prime for a subfield curve (#E(F_q) divides it and exceeds 1), so the largest
prime-order subgroup has size ≤ #T_n ≈ q^{n−1}. Second, the matched rho is
√ℓ ≈ q^{(n−1)/2}, not q^{n/2}. Rho also folds by ⟨−1, φ⟩, a factor √(2n) (2.4 at n = 3,
3.2 at n = 5).

**1.3 Equations [GM-IC §2–§5, read at source].** With the Semaev polynomial f_n (degree
2^{n−2} in each variable, total (n−1)2^{n−2}), T_3 = {f_3(x, x^q, x^{q²}) = 0, y ∈ F_{q³}}
exactly; T_5 needs ≤ 16 exceptional x-values excluded [GM-PC, Prop. 4, Remark 5]. Weil restriction
modulo x_i^q − x_i leaves one equation f̃_n(x_0..x_{n−1}) = 0 of total degree (n−1)2^{n−2}
(4 for n = 3, 32 for n = 5). The symmetrised g_n has degree 2 (n = 3) or 8 (n = 5) and
gives the (n−1)-coordinate optimal representation (s_1..s_{n−1}) [GM-PC]. The companion
paper [GM-OR] represents P by the coefficients of the function with divisor P + φP + … −
nO. For n = 3 this is the F_q-rational line y = mx + c through P, φP, φ²P.

Derived here, desk-checked below: s_1 = 3x_0 = m², so the factor base x_0 = 0 is the set of
points on horizontal lines y = c ∈ F_q. The count 3·#{c : x³ + Ax + B − c² irreducible}
reproduced Gorla–Massierer's |F| exactly at q = 1021, 4093, 16381 (900, 4002, 16380;
unverified desk check). In Kummer coordinates (ζ^n = µ, n | q − 1) φ acts diagonally, so
the coordinate-subspace base is φ-stable.

**1.4 Decomposition systems, n = 3 and n = 5.**

| | T_3 (m = 2) | T_5, 4-point (m = 4) | T_5, 3-point (Joux–Vitse) |
|---|---|---|---|
| unknowns | 4 raw; 2 after x_{i2} = −A/(3µx_{i1}) | 8 | 6 (4 after fixing x_{03}) |
| equations | 5 raw (3 Weil components of f_3, 2 membership); 3 after elimination | 9 (5 Weil components of f_5, 4 membership) | 8 (5 of f_4, 3 membership); 7 after fixing |
| degrees | 4 in each of x_{01}, x_{11}; total 7, 8, 7 | Weil part ≤ 32; membership 32 | Weil part total 12 (4 per variable); membership 32 |
| F̄_q solutions | 6 (univariate degree 6) | ideal degree bound 32^8 ≈ 1.1·10^12 | not stated |
| regularity | 14 (12–13 for special R) | GM bound 249 (binom(257,8) ≈ 4.2·10^14) | Magma failed at q = 1021 (weeks, >300 GB) |

**1.5 Against Gaudry's descended S_4 (E/F_{p³}, x ∈ F_p), n = 3, q ≈ 2^20.**

| | T_3 | S_4, x ∈ F_p [`gaudry_cubic.rs`, §11.4] |
|---|---|---|
| group, matched rho | order ≈ q², ≈ 0.89q steps | prime order ≈ p³, ≈ 0.89p^{1.5} steps |
| relation size m | 2 | 3 |
| unknowns / equations | 2 / 3 (3 equations, overdetermined) | 3 / 3 (e_1, e_2, e_3) |
| degree | 4 per variable; total 7–8 | total ≤ 4 |
| F̄ solutions | 6 | 64 (quotient dimension 64 at every residual) |
| solving degree, dense columns | 14, C(16,2) = 120 | 10, C(13,3) = 286 |
| base | ≈ q signed points, a curve in V_3 | ≈ p signed points, a line |
| yield per target | 0.50 (2,091,965 targets for 1,045,963 relations at q = 2^20 − 3) | 1/6; measured 0.15–0.18 |
| per-residual cost | 1.36 ms Magma (q = 2^20 − 3); desk 5·10⁴–3·10⁵ F_q mults | C₃ = 4.8·10⁶ → 0.88·10⁶ F_p mults |

So T_3 has fewer unknowns (2 against 3), 10× fewer solutions and smaller matrices, but
higher degree and higher solving degree. The yield is 3× better per target.

**1.6 Complexity [GM-IC Thm 4.2, §6].** With dimension d = n − 1, Gaudry's Õ(q^{2−2/d}) gives
q^1 for n = 3 and q^{3/2} for n = 5 with large primes. Without them it gives q² for both.
Rho on T_n is q^{(n−1)/2}: tie at n = 3; 3/2 against 2 at n = 5 (large primes only). Gaudry
on the whole curve is q^{2−2/n}: 4/3 and 8/5. The hybrid T_5 variant costs Õ(q³), worse
than rho. The constants are binom((2n−2)(n−1)2^{n−2}+1, 2n−2)^ω · (n−1)!: binom(17,4) =
2380 at n = 3. GM §6.2 notes the whole-curve system is smaller (n equations in n unknowns,
total degree 2^{n−1}).

## 2. Prior art: literature

- [GM-IC] Gorla, Massierer, *Index calculus in the trace zero variety*, Adv. Math. Commun. 9(4)
  (2015) 515–539; arXiv 1405.1059 v2; ePrint 2014/318. **read-at-source** (arXiv PDF, all
  sections); ePrint and AMC pages abstract-only. Claim taken: §1.3–1.6 above, including
  Tables 1–2 (n = 3 full DL to 36 bits in 957 s, relations at 40 bits in 2845 s, LA not solved
  in Magma; n = 5 only via the hybrid) and "n = 3: Pollard–Rho and index calculus have the
  same complexity".
- [GM-PC] Gorla, Massierer, *Point compression for the trace zero subgroup over a small degree
  extension field*, Des. Codes Cryptogr. (2015), DOI 10.1007/s10623-014-9921-0; arXiv 1403.0126;
  ePrint 2014/158. **read-at-source** (arXiv PDF). Claim: the Semaev-derived equation for T_n,
  the optimal representation (s_1..s_{n−1}), explicit n = 3, 5 equations, the exceptional
  points for n = 5. The card's "optimal representation" (DCC 2015) is this paper.
- [GM-OR] Gorla, Massierer, *An optimal representation for the trace zero subgroup*, arXiv
  1405.2733 v4, ePrint 2014/333. **read-at-source, partial** (introduction and §4; appendix
  unread). It is a different paper from the card's "ePrint 2014/158" pointer. Claim: for
  elliptic curves the representation is the coefficient vector of h_P (Cor. 4.2–4.3).
- Massierer, Basel thesis 2014, *Trace zero varieties in cryptography: optimal representation
  and index calculus*. **abstract-only** (search snippet; the edoc mirror failed with DNS
  ENOTFOUND). Not used for a claim.
- Gaudry, J. Symb. Comput. 44 (2009); Diem, Compositio Math. 147 (2011); Joux–Vitse, J.
  Cryptology (2012/13). **recalled**; the statements used (exponent q^{2−2/d}; dropping one
  factor-base point) were read second-hand in [GM-IC §2.1, Remark 3.3].
- Diem–Scholten, trace-zero cover attacks (genus 4 for n = 5, 1 curve found in the AREHCC
  report). **recalled via [GM-IC §6.3]**; not read. It is a cover attack, not index calculus.
- Galbraith–Gaudry survey, Des. Codes Cryptogr. 78 (2016), ePrint 2015/1022. **not read:
  HTTP 403** on the PDF.

## 3. Prior art: these repositories

- `/home/user/crypto/src/cryptanalysis/gaudry_cubic.rs` (header 1–65, 528–600): random
  prime-order E/F_{p³} (a, b ∈ F_{p³}, p < 2^20), base {x ∈ F_p}, S_3 pair test; S_4 solve
  built later. The baseline class contains no subfield curve, because prime order excludes it.
  **Bears: closes the "same ECDLP" premise.**
- `…/glv_gaudry.rs` (header 1–50): j = 0 curves y² = x³ + b, b ∈ F_{p³}, ψ-folded base. Same
  instance class. **Unrelated to T_n; shows folding levers already exist.**
- `/home/user/crypto/research/ic_next10_20261014/01-fp4-ladder/README.md`: k = 4, plain
  C₄ = 1.213·10¹² F_p mults, D₄ 2.54·10¹⁰; DLP exponent 0.333 against rho 0.464; plain LA
  1.44× rho-neg; D₄ handover 2^126–2^137. Notes that on a non-subfield curve the ψ fold and
  the D₄ Y-line cannot coexist. **Supports:** dimension-4 reference for T_5.
- `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md` §11.1–11.4 (k = 3: three
  quartics, Bézout 64, C₃, decomposition rate 1/6) and §11.15–11.19 (k = 4: D = 4096,
  Macaulay degree 29, C₄ = 17× floor, n* ≈ 2^151, r∞ = 0.518 ± 0.031, 0.733 ± 0.036 against
  negation-folded rho, handover ≈ 2^156). **Supports:** the cost scales used in §4–§5.
- `/home/user/crypto/RESEARCH_ISOGENY_FIBERS_ECDLP.md:1819`: a bibliography line only. Its H8
  (715–734, ranked "negative (Diem invariance)") and C2 (861–862, a T_3 test at p ≈ 2^14)
  were proposed; no result file found. **Supports closure, unmeasured.**
- `research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md:442`: "trace zero" is the
  absolute trace F_{2^n} → F_2 of binary abscissae. **Unrelated.**
- `docs/ic/ECDLP_RESEARCH_TECHNIQUES.md:64–78`: "does not yet contain a general trace-zero-
  variety ECDLP pipeline"; only a Koblitz subspace policy. **Confirms novelty status.**
- `docs/ic/factor-base-encyclopedia/REPORT.typ:1125–1132`: trace-zero base over F_{p²},
  relations land in E(F_p), "descent blocked". **Related failure mode** (wrong subgroup), different
  target.
- `/home/user/crypto-autoresearcher/focus/current_plan.md:551–570, 2219–2241` (P1545):
  transfer of a prime-field subgroup into a trace-zero variety, inconclusive; "full fixed-degree
  trace-zero index calculus costs at least N^{1+o(1)} relative to the source problem".
  **Different question; consistent.**

## 4. Why it could lower the Gröbner cost or the degree of regularity

It lowers the cost, not the degree, and only through dimension. T_3 is a 2-dimensional
variety, so a relation has m = 2 points and 2 unknowns, which is why the quotient has 6
points instead of 64. A dense Macaulay count (assumption: the repository's design, rows ×
columns × rank) is about 100 × 120 × 100 ≈ 1.2·10⁶ for T_3 against 252 × 286 × 252 ≈
1.8·10⁷ for S_4, about 15× less. Scaling the measured S_4 Macaulay cost, 4.4·10⁶ at baseline
and 0.88·10⁶ tuned, gives C_T3 ≈ 6·10⁴–3·10⁵. A resultant-cascade count gives about 5·10⁴.
Magma's 1.36 ms is of the same order. All of this is an unverified desk estimate.

Per relation, T_3 needs 2 solves against 6, so it is about 10–50× cheaper than S_4 at equal
q. Two assumptions carry the gain. The yield is 1/2 because the group has order q²; that is
the same trade as Joux–Vitse's k − 1 decomposition (§11.9: C 580× lower, residuals ∝ p more).
And it is paid for by the matched rho dropping from p^{1.5} to q. Measured against its own
matched rho, T_3 is far worse (§5.3). No step lowers the degree: it rises from 4 to 7–8
per equation and from 10 to 14 in solving degree.

## 5. Obstructions

1. **Instance class (exact).** T_n needs φ ∈ End(E), i.e. E/F_q. The repository's baseline
   instances have prime order ≈ p^k, and a subfield curve cannot (1.2). A random E/F_{q^n} has
   j ∈ F_q with probability ≈ q^{1−n}. T_n is therefore a new instance family with its own
   matched rho, not a representation of the existing rows. Under the owner's rule an
   improvement must be measured on the same instances, so a T_n row would be a new baseline.
2. **Gaudry's base degenerates on subfield curves (elementary).** For odd n, x ∈ F_q forces
   y ∈ F_q, so {x ∈ F_q} = E(F_q). Relations land in a subgroup of order ≈ q that
   misses the ℓ-part. A shifted subspace x ∈ vF_q (derived, untested) avoids this but costs
   q^{2−2/n}, still above rho's q^{(n−1)/2} at n = 3.
3. **n = 3 exponent tie.** Dimension 2 gives q^{2−2/2} = q = √|T_3|, GM's statement with
   large primes. The ratio to rho is then a constant ≈ 0.036·C_T3 (relation phase, unfolded:
   2 solves per relation, q relations, rho 0.886q·63 mults). C_T3 = 5·10⁴–3·10⁵ gives
   2·10³–10⁴ (unverified desk). From GM's Table 1, relation collection plus enumeration
   (2950 s) against rho at the quoted 25,000–150,000 additions per second (6–37 s) gives
   80–490×. Folding by ⟨−1, φ⟩ helps IC by a net √(2n) ≈ 2.4 at most. The ratio drops below 1 only
   if C_T3 falls below ≈ 30 F_q multiplications unfolded (≈ 70 folded), about one affine
   addition, while merely evaluating the three Weil components costs about 10² or more.
   Plain linear algebra, q² against rho's q, makes the plain method worse still.
4. **n = 5 is dominated.** The dimension-4 reference is the repository's k = 4 ladder: 4
   equations of total degree ≤ 8 in 4 unknowns, D = 4096, Macaulay degree 29, C₄ = 1.2·10¹²,
   plain n* ≈ 2^151. The T_5 system has 9 equations of degree ≤ 32 in 8 unknowns (ideal degree
   ≤ 2^40; GM could not symmetrise it). GM's own attempt failed at q = 1021, and the hybrid
   costs Õ(q³). Its exponent window (3/2 against 2) is the same as k = 4's, and it is
   unreachable at any size the repository can run.
5. **Generic bound.** Rho on T_n is the generic bound. Meet-in-the-middle or table oracles
   over F are enumeration in disguise (constant-factor). The algebraic Weil-descent oracle
   is structure, but at n = 3 it only ties.
6. **Not a counter to the card's own kill.** The card kills if neither degree nor per-target
   time is 2× below Gaudry's. Per-target time is plausibly 10–20× below, so that clause
   will not fire; the lead is closed by §5.1 and §5.3, not by that clause.

## 6. Minimal discriminating test (frozen protocol)

Requires new Rust code first (T_3 generator and pair solver); the mechanical tier runs it
only after that. No Python in `/home/user/crypto`.

- **Instances.** p ∈ {16381, 131071, 1048573} (all ≡ 1 mod 3, prime), µ the smallest
  non-cube, curve y² = x³ + x + B with B scanned from 1 until ℓ = #E(F_{p³})/#E(F_p) is
  prime and gcd(#E(F_p), ℓ) = 1 (GM's recipe). Three curves per p; generator [#E(F_p)]P.
  Control arm: three random prime-order generic E/F_{p³} per p via `generate_instance3`.
- **Instrument.** `examples/gaudry_cubic_bench.rs --protocol --groebner` for the S_4 arm;
  `src/cryptanalysis/f4_fp.rs` (`f4_record`, `field_ops_total`) for the T_3 system and its
  counted cost; `run_rho3` for rho on the same group, 16 runs per curve.
- **Step 1 (structure, p = 16381).** For 20 targets build the system (10), then record
  unknowns, equation degrees (per variable and total), solving degree, quotient dimension.
  Prediction: 2 / 3 / (4, 4, 4; 7, 8, 7) / 12–14 / 6.
- **Step 2 (yield and |F|).** At each p, ≥ 20,000 targets per curve: relations per target
  and |F|. Prediction: 0.50 (T_3); 0.167 ± 0.02 (S_4); |F| = 3·#{c irreducible}.
- **Step 3 (cost and distance).** Record counted F_q multiplications per solve (C_T3 and C₃),
  S_rel = (|F| + 1)(1/yield) C / (S_rho √ℓ c_add), with S_rho from the same-group rho runs, and the
  log–log slope against p. Price unpriced native work at 0, 0.1 and 1 GAE/op for every arm
  (AGENTS.md §5a); rank only if the order holds across the band. Report S_rel as distance to
  matched rho.
- **Prediction.** S_rel(T_3) = 2·10³–10⁴, slope 0 ± 0.1; S_rel(S_4) ≈ 33 at 2^20, slope ≈ −0.5;
  C_T3/C₃ ∈ [0.05, 0.4].
- **Kill (expected to fire).** S_rel(T_3) ≥ 10 at p = 2^20 and slope ≥ −0.1: lead closed. It
  survives only if S_rel(T_3) < 1 or the slope is ≤ −0.3, which would contradict the exponent
  tie. Deviations in quotient dimension (≠ 6) or yield (> 10 % from 0.5) are reported, not
  explained away.
- **Cost.** G arm: 10⁴ residuals × 4.8·10⁶ mults at about 1.2·10⁸ mults/s (§11.4's wall
  rate) ≈ 400 s per curve; T arm about 20 s; rho seconds. Total ≤ 2 CPU-hours, < 1 GB.

## 7. Ranking inputs

expected gain: none vs baseline rows | confidence: high that the test would kill, medium on
n = 5 | cost: 2–3 person-days (0.5 for steps 1–2) | tier: constant at best (n = 3), no
exponent regime that the k = 4 ladder does not already cover | depends on: new T_3 harness;
nothing else. Recommendation: do not spend the test budget unless a subfield-curve row family
is wanted for its own sake.

## 8. Open questions

- The T_3 per-residual cost in the repository's counted unit is a desk estimate; only the
  harness settles it.
- The T_5 4-point ideal degree and solving degree are unknown (bound 2^40; no one has solved
  it). The conclusion for n = 5 leans on D = 4096 for k = 4.
- Frobenius-orbit-folded bases (n-fold smaller F, free conjugate relations) were not
  costed beyond the √(2n) net factor; they are a constant lever and do not change §5.3.
- Unread: the Massierer thesis (index calculus chapter), the Galbraith–Gaudry survey (403),
  Diem–Scholten, the appendix of [GM-OR], and ePrint 2026/384, so the structured generic-group
  remark in the card was not used.

## 9. Sources

- https://arxiv.org/abs/1405.1059 — 200; PDF https://arxiv.org/pdf/1405.1059 — 200 (363,627 bytes), read in full.
- https://arxiv.org/abs/1403.0126 — 200; PDF https://arxiv.org/pdf/1403.0126 — 200 (334,485 bytes), read to the references.
- https://arxiv.org/pdf/1405.2733 — 200 (393,164 bytes), read partly.
- https://eprint.iacr.org/2014/318 — 200, abstract only. https://eprint.iacr.org/2014/158 — 200, abstract only.
  https://eprint.iacr.org/2014/333 — 200, abstract only.
- https://www.aimsciences.org/article/doi/10.3934/amc.2015.9.515 — 200, metadata only.
- https://eprint.iacr.org/2015/1022.pdf — HTTP 403. https://edoc-vmtest.ub.unibas.ch/32498 — DNS ENOTFOUND.
  https://researchspace.auckland.ac.nz/handle/2292/36116 — empty body. WebSearch ×3 (snippets only).
- Repository paths read: `/home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md`;
  `…/LEADS.md` (lines 1–80, 245–294); `src/cryptanalysis/gaudry_cubic.rs` (1–260, 525–600);
  `src/cryptanalysis/glv_gaudry.rs` (1–140); `research/ic_next10_20261014/01-fp4-ladder/README.md`;
  `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md` (1004–1264, 2181–2625);
  `RESEARCH_ISOGENY_FIBERS_ECDLP.md` (708–748, 780–784, 855–870, 1815–1821);
  `research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md` (425–464);
  `docs/ic/ECDLP_RESEARCH_TECHNIQUES.md`, `docs/ic/factor-base-encyclopedia/REPORT.typ`
  (excerpts); `/home/user/crypto-autoresearcher/focus/current_plan.md` (excerpts);
  `research/weak_curve_leaderboard_20261012/COVERAGE_AUDIT.md`, `VALIDATION.md` (excerpts).
- Desk checks (scratchpad, not committed, unverified): factor-base count against GM's |F| at
  three q; gcd-condition scan over 2,996 pairs; dense matrix sizes; ratio arithmetic.
