# Research Goals 2026-11-22 — Tranche 3: Absent Frameworks and Unexplored Prime-Field ECDLP Index-Calculus Leads

**Date anchor:** 2026-11-22  
**Status:** Planning record and desk-screen specification. **Nothing here is evidence.** No experiment has been executed, no hypothesis status has changed, and no security claim about any deployed curve is made or withdrawn. Every proposal below is subject to the standing NoveltyGate (`G30-028`), computation-of-record (`Gate A`), null-exchangeability (`Gate B`), and cost-gating (`G30-001`) rules before staffing or promotion.

**Provenance:** Compiled from a 2026-11-22 audit of the Isogeny Labs repositories (`isogeny-labs/crypto`, `isogeny-labs/cryptanalysis`, `isogeny-labs/crypto-autoresearcher`, and `isogeny-labs/workspace`), cross-checked against the 120+ closed constructive lanes in `workspace/docs/notes/subexp-algo-20261121/SUCCESSOR_H_CONSTRUCTIVE_SUBEXP_T.md`, `crypto/RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`, `crypto/RESEARCH_IC_LEADS_ASSESSMENT_20261018.md`, the `outsidebox-*-20261121` sweep, and the G31/G32 blind-spot batches (`research_goals_20261120_ic_blindspot_batch.md`, `research_goals_20261121_ic_tranche2_batch.md`).

---

## 0. Screening Summary and Non-Duplication Audit

Every item below was checked against `workspace/docs` and `crypto-autoresearcher/knowledge` (zero hits for the primary technical terms in G33-001 through G33-006) and verified to be structurally distinct from the closed lanes in `SUCCESSOR_H_CONSTRUCTIVE_SUBEXP_T.md`.

| ID | Lead / Framework | Cluster | Role | Distinct From Closed Lanes | Honest Prior |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **G33-001** | AG list decoding (Guruswami–Sudan / Koetter–Vardy) on `E(F_p)` | 08 BARRIERS / 05 RELATIONS | Barrier + desk-screen | Not `ALGFB`, `LOWDEG`, or `G31-001` | Barrier theorem (Johnson/Singleton cap) |
| **G33-002** | Dixon resultants, Bézoutians, and Chow forms for `S_4`/`S_5` | 02 GROEBNER | Engine / elimination | Replaces Macaulay/XL/F4/Sylvester | Constant/memory win; no exponent change |
| **G33-003** | Gowers `U^3`/`U^4` uniformity norms on `x(kP)` bases | 08 BARRIERS / 04 FACTORBASE | Barrier + metrology | Extends `CHARSUM` (`U^2` only) and `G31-001` | Barrier closing multi-term additive bias |
| **G33-004** | Singular-curve `p`-adic homotopy continuation (`E_0 → E`) | 02 GROEBNER / 07 TRANSFER | Desk-screen | Deforms `S_m` varieties, not point lifts (`PADSEC`) | Likely mixes via monodromy; cheap falsifier |
| **G33-005** | Survey Propagation / cavity diagnostics on Semaev-SAT | 02 GROEBNER / 09 HARNESS | Diagnostic | Diagnoses 1-RSB freezing behind `DECSAT` / Lead 2B | Explains CDCL stall; unlikely to beat `√p` |
| **G33-006** | Leakage-assisted Coppersmith–Semaev IC | Applied / Hybrid | Applied attack probe | Combines `COPSM` (`δ ≤ 1/6`) with side-channel/HNP (`HNPA` was plain-only) | Potentially practical in partial-leakage regimes |
| **G33-007** | Farey / rational-reconstruction factor bases (`x_i ≡ u_i/v_i mod p`) | 04 FACTORBASE / 02 GROEBNER | Desk-screen | Multi-homogeneous Kummer `X/Z` height box vs interval `[1, B]` | Likely invariant Coppersmith reach; 30-min check |
| **G33-008** | Sum-of-Squares (SoS) / Lasserre SDP relaxations on Semaev | 02 GROEBNER | Non-lattice probe | Outside LLL/BKZ (`COPSM`) and Gröbner (`SDEG`) | Likely broken by `k·p` rounding; clean negative |
| **G33-009** | Waring / CP low-rank splitting of `S_4`/`S_5` on `a = 0` curves | 02 GROEBNER / 05 RELATIONS | Engineering | Exploits `G31-004` `a = 0` sparsity (191 vs 439 monomials) | Constant-factor win for `secp256k1` family |
| **G33-010** | Banded / low-treewidth relation graphs vs `m = 3` LA wall | 08 BARRIERS | Impossibility lemma | Tests escaping `Θ(B²)` Wiedemann via nested dissection | 2-paragraph closure (like `G32-004`) |
| **G33-011** | Multi-curve simultaneous / approximate-GCD CRT-Semaev | 08 BARRIERS | Desk-screen | Cross-curve composite modulus `M = ∏ p_j` | Dominated by `β = 1/k`; 1-page closure |
| **G33-012** | Lean-verified prime-field algebraic-oracle lower bound (Winterhof + AGM) | 08 BARRIERS / Formal | Publication-grade theorem | Unifies 120+ closed lanes with ePrint 2026/384 | Highest-leverage prime-field publication |

---

## 1. Frameworks Completely Absent from the Repositories

### G33-001: Algebraic-Geometric List Decoding on `E(F_p)` (Guruswami–Sudan / Koetter–Vardy)

* **Cluster:** 08 BARRIERS / 05 RELATIONS. **Type:** desk-screen + barrier theorem.
* **Mechanism:** Finding a low-degree rational function `f ∈ L(k·O)` on `E` whose zeros concentrate in a factor base `B` (plus target `Q`) is formally dual to list decoding a one-point AG code on `E(F_p)`. Guruswami–Sudan / Koetter–Vardy list decoding solves the dual problem in polynomial time via weighted multiplicity interpolation and Roth–Ruckenstein root-finding in the function field `F_p(E)`.
* **Cheapest falsifier:** Write down the exact Johnson / Singleton list-decoding radius for a one-point AG code of length `|B|` and genus `g = 1` over `F_p`, and test whether any choice of multiplicity weights `m_i` can force `m` zeros inside `B` with `|B| ≈ p^{1/m}` at cost `o(√p)`.
* **Kill / promotion criteria:** If the Johnson radius strictly caps the forced-zero agreement below the `m`-decomposition threshold for all `m ≥ 3`, promote as a formal **Cluster 08 barrier certificate** tying coding-theoretic list-decoding capacity to prime-field IC relation density.

### G33-002: Dixon Resultants, Bézoutians, and Chow Forms for Multihomogeneous `S_4`/`S_5`

* **Cluster:** 02 GROEBNER. **Type:** solver/elimination benchmark.
* **Mechanism:** Existing elimination engines (`matrix_f4_f2`, `f4_fp`, MutantXL, Sylvester resultants) expand homogeneous Macaulay matrices where matrix build dominates runtime (~60× reduction cost in `RESEARCH_IC_LEADS_ASSESSMENT_20261018.md` §3). Because Semaev `S_m` is multihomogeneous of degree `2^{m−2}` in each variable, **Dixon resultants** and **Bézoutian matrices** produce substantially smaller elimination matrices without Macaulay's extraneous degree inflation, and **Chow forms** directly encode the `(m−1)`-secant incidence from `G31-001`.
* **Cheapest falsifier:** Build the exact Dixon / Bézoutian matrix for `S_4` (`m = 3`) over `F_p` (and char-2 Koblitz `m = 3, 4` cells) and compare matrix dimensions, sparsity, and rank-extraction cost against `f4_fp` (`crypto#4891`) and `f4_gf2` (`T14`) on identical draws.
* **Kill / promotion criteria:** Kill if Dixon matrix build + rank extraction exceeds `f4_fp`/`f4_gf2` wall time and memory across all tested cells; promote as an engineering kernel if it reduces peak RAM on `(m = 4, ℓ = 6)` or `ℓ ≥ 7` cells.

### G33-003: Higher-Order Additive Combinatorics — Gowers `U^3` and `U^4` Uniformity Norms

* **Cluster:** 08 BARRIERS / 04 FACTORBASE. **Type:** metrology + barrier certificate.
* **Mechanism:** Existing character-sum checks (`CHARSUM`, `P1422`) only test `U^2` (linear Fourier) uniformity of `k ↦ x(kP)`. However, `S_4` (`m = 3`) and `S_5` (`m = 4`) decompositions depend on 3-term and 4-term additive patterns governed by the **Gowers `U^3` and `U^4` norms** (quadratic and cubic phase correlations).
* **Cheapest falsifier:** On toy prime-order curves (`p ∈ [2^12, 2^16]`), compute empirical `U^3` norms of the indicator function `1_B` for `x`-window bases, pullback bases (`f(x) = x²`, `x² + c`), and negation-matched random controls (`Gate B`).
* **Kill / promotion criteria:** If `U^3(1_B)` matches negation-closed random controls within sampling noise across all tested curves, file as a barrier certificate closing higher-order additive-correlation hypotheses for prime-field factor bases.

### G33-004: Singular-Curve `p`-adic Homotopy Continuation (`E_0 → E`)

* **Cluster:** 02 GROEBNER / 07 TRANSFER. **Type:** desk-screen + toy probe.
* **Mechanism:** Unlike `PADSEC`/`PADJET` (which lifted *points* via Hensel/Teichmüller), deform a singular curve `E_0: y² = x³` (where the group law is additive and `S_m` splits into elementary rational factors) into the target curve `E_1: y² = x³ + ax + b` via `E_t: y² = x³ + t·a·x + t·b`, tracking Semaev roots as power series in `t` (`p`-adic or formal homotopy continuation).
* **Cheapest falsifier:** At `m = 2, 3` over small primes (`p ~ 1009`), check whether the start system at `t = 0` isolates small-height branches or whether monodromy around discriminant singularities immediately permutes all `(2^{m−2})^m` branches together.
* **Kill / promotion criteria:** Kill immediately if path tracking requires tracking `Ω((2^{m−2})^m)` branches or if branch mixing destroys the small-`x` restriction before `t = 1`.

### G33-005: Survey Propagation / Cavity Diagnostics on Semaev-SAT Encodings

* **Cluster:** 02 GROEBNER / 09 HARNESS. **Type:** diagnostic.
* **Mechanism:** `DECSAT` and `cryptanalysis` PRs `#1845`/`#1847` (Lead 2B CDCL(`F_p`)) observe exponential CDCL scaling (`α ~ m`). Running **Belief Propagation (BP)** and **Survey Propagation (SP)** on the factor graph of the Semaev-CNF / split-`S_3` circuits directly distinguishes whether CDCL stalls due to a **1-RSB frozen-variable phase** (structural hardness of the encoding) or due to poor branching heuristics.
* **Cheapest falsifier:** Run standard SP / BP-guided decimation on 10–20 small unsatisfiable and planted-satisfiable `m = 3` split-`S_3` CNF instances from Lead 2B and measure the fraction of frozen variables (nonzero SP warning biases).
* **Kill / promotion criteria:** If SP detects a frozen 1-RSB core at all nontrivial bit widths, archive as a diagnostic closing SAT/CDCL heuristic tuning; if variables are unfrozen (RS phase), test BP-guided variable ordering inside CDCL.

### G33-006: Leakage-Assisted Coppersmith–Semaev Index Calculus (Applied Hybrid)

* **Cluster:** Applied / Hybrid Cryptanalysis. **Type:** desk-screen + comparative benchmark.
* **Mechanism:** `COPSM` closed unassisted Coppersmith on Semaev (`δ_max = 1/6` for `S_3`, `1/80` for `S_5`), while `HNPA` closed *plain* ECDLP-to-HNP conversion. In real-world side-channel or fault scenarios where an implementation leaks partial `x`-coordinate bits, biased ECDSA nonces, or constrained-weight scalars, the hybrid question is unmeasured: how many leaked bits shrink a target's effective box into the Coppersmith-reachable `S_3`/`S_4` regime (`δ ≤ 1/6`), and is there any leakage window where Semaev+Coppersmith outperforms standard HNP/BKZ?
* **Cheapest falsifier:** Derive the exact leaked-bit threshold `L` required for `S_3` + Jochemsz–May (`δ ≤ 1/6`) to decompose a target on P-256 / secp256k1 and compare directly against standard HNP/BKZ lattice recovery at the same leakage `L`.
* **Kill / promotion criteria:** Close if standard HNP/BKZ strictly dominates at every leakage level `L`; promote to an applied assurance prototype if Semaev+Coppersmith wins in any partial-`x` / single-trace leakage regime.

---

## 2. Additional Prime-Field Leads Beyond the Closed List

### G33-007: Farey / Rational-Reconstruction Factor Bases (`x_i ≡ u_i / v_i mod p`)

* **Cluster:** 04 FACTORBASE / 02 GROEBNER. **Type:** desk-screen.
* **Mechanism:** Replace the affine interval `x_i ∈ [1, B]` with small projective fractions `x_i ≡ u_i / v_i (mod p)` with `|u_i|, |v_i| ≤ H ≈ √B = p^{δ/2}` (`|F| ≈ (6/π²)H² ≈ B`). Clearing denominators in `S_{m+1}(u_1/v_1, …, u_m/v_m, x_Q)` yields a multi-homogeneous system in variables of half the bit-length that natively lives on the Montgomery/Kummer `X/Z` line.
* **Cheapest falsifier:** Compute the asymptotic Jochemsz–May shift-lattice determinant bound for the multi-homogeneous `(u_i, v_i)` system at `m = 2, 3` and compare against Probe B/C (`δ_max = 1/6`).
* **Kill / promotion criteria:** Kill if `δ_max ≤ 1/6` at `m = 2` (and `≤ 1/24` at `m = 3`); promote to a Probe-B-style LLL test only if projective multi-homogeneity strictly raises `δ_max`.

### G33-008: Sum-of-Squares (SoS) / Lasserre SDP Relaxations on Bounded-Box Semaev

* **Cluster:** 02 GROEBNER. **Type:** non-lattice solver probe.
* **Mechanism:** Step outside both LLL/BKZ and Gröbner bases by formulating `S_{m+1}(x_1, …, x_m, x_Q) − k·p = 0` with real polynomial box constraints `x_i(B − x_i) ≥ 0` as a degree-`2d` Lasserre moment-matrix / Positivstellensatz SDP.
* **Cheapest falsifier:** Run degree-4 and degree-6 SoS relaxations (`cvxpy`/SCS) on planted `S_3` instances from Probe B at `δ ∈ {0.14, 0.18, 0.22}` (straddling Coppersmith's `1/6` ceiling) and check whether the rank-1 moment extraction recovers `(x_1, x_2)`.
* **Kill / promotion criteria:** Kill if degree-6 SoS fails to recover planted roots above `δ = 0.16` or if SDP size scales worse than LLL; record as a formal closure of convex/SDP relaxations.

### G33-009: Waring / CP Low-Rank Splitting of `S_4` and `S_5` on Sparse (`a = 0`) Curves

* **Cluster:** 02 GROEBNER / 05 RELATIONS. **Type:** algebraic engineering.
* **Mechanism:** `G31-004` WP2 measured that `S_4` collapses from 439 to **191 monomials** on `y² = x³ + 7` (`a = 0`, `secp256k1` shape). Computing the exact **matricization / CP / Waring rank** of `S_4(x_1, x_2, x_3, x_Q) = ∑_{j=1}^r f_j(x_1, x_2)·g_j(x_3, x_Q)` tests whether `a = 0` admits a low-rank split-variable representation that accelerates batched evaluation or split-MITM.
* **Cheapest falsifier:** Construct the exact `(x_1, x_2)` vs `(x_3, x_Q)` coefficient matrix of `S_4` (and `S_5` specialized to `a = 0`, `a = −3`, and generic `(a, b)`) over `F_p` and compute its exact rank.
* **Kill / promotion criteria:** Kill if rank equals the full degree-box dimension (no compression); promote to a fast evaluation kernel if rank drops by `≥ 2×` on `a = 0` or `a = −3`.

### G33-010: Banded / Low-Treewidth Relation Graphs vs the `m = 3` Linear-Algebra Wall

* **Cluster:** 08 BARRIERS. **Type:** two-paragraph impossibility lemma (modeled on `G32-004`).
* **Mechanism:** `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §2.4 rules out `m = 3` even with a free `S_4` oracle because unstructured sparse Wiedemann costs `Θ(B²) = n^{2/3}` at `B = n^{1/3}`. Restricting relation triples to a banded window `|i − j| ≤ w` forces treewidth `O(w)` and drops nested-dissection Gaussian elimination to `O(B·w²)`, at the price of reducing relation supply from `B³/p` to `B·w²/p`.
* **Cheapest falsifier:** Minimize `cost(B, w) = (p / (B·w²))·B·T_dec + B·w²` over `(B, w)` subject to `w ≤ B`.
* **Expected outcome:** Minimizing over `w` yields `cost ≥ 2·√(p·B·T_dec) ≥ 2·√(p·T_dec)·B^{1/2}`, which strictly worsens `m = 3` for every `B > 1` — closing graph-locality LA escapes in one clean lemma.

### G33-011: Multi-Curve Simultaneous / Approximate-GCD CRT-Semaev

* **Cluster:** 08 BARRIERS. **Type:** 1-page desk-screen / closure.
* **Mechanism:** Given `k` prime-field curves `E_1/F_{p_1}, …, E_k/F_{p_k}`, lift their `S_3`/`S_4` polynomials modulo `M = ∏ p_j` over a shared small-`x` box, or run approximate-GCD / Howgrave–Graham to find a root vanishing modulo *one* unknown factor `p_j ≈ M^{1/k}` of `M`.
* **Cheapest falsifier:** Evaluate the multi-modulus Coppersmith reach `β² / d` at `β = 1/k` against `k` independent single-curve runs.
* **Expected outcome:** `β² = 1/k²` shrinks Coppersmith reach quadratically in `k` while amortization is at most linear in `k` — closing cross-curve CRT amplification cleanly.

### G33-012: Lean-Verified Prime-Field Algebraic-Oracle Lower Bound (Winterhof + AGM)

* **Cluster:** 08 BARRIERS / Formal Verification (`crypto-lean`). **Type:** theorem + publication.
* **Mechanism:** Unify the 120+ closed prime-field lanes in `SUCCESSOR_H_CONSTRUCTIVE_SUBEXP_T.md` with (i) Winterhof / Niederreiter / Mérai polynomial-degree lower bounds on the discrete-log interpolation map `x(kP) ↦ k` over `F_p`, and (ii) the Algebraic Group Model (AGM) + ePrint 2026/384 (`Ω(min(√q, 1/δ))`).
* **Deliverable:** A machine-checked Lean 4 barrier theorem (extending `aburan28/crypto-lean`) formalizing the exact hypotheses under which any prime-field IC algorithm with `Ω(B)` factor-base membership degree and Coppersmith-bounded decomposition is bounded below by `Ω(√p)`.

---

## 3. Recommended Sequencing

1. **Immediate zero-compute closures (1 day):** Write and merge the desk-screens for **G33-010** (banded-LA lemma), **G33-011** (CRT-Semaev), **G33-007** (Farey Coppersmith determinant), and **G33-001** (AG list-decoding Johnson radius). All four either surface a genuine anomaly immediately or close recurring proposals permanently.
2. **Fast toy probes (1–2 days):** Run **G33-009** (`S_4` `a = 0` bilinear rank — 10-line Sage/Python script), **G33-003** (`U^3` uniformity on `x`-window vs negation-matched null), and **G33-006** (leakage-assisted Semaev+Coppersmith vs HNP/BKZ threshold).
3. **Solver / binary crossover (higher effort):** Benchmark **G33-002** (Dixon/Bézoutian vs `f4_fp`/`f4_gf2`) alongside the already-scoped `f4_gf2` × `equivariant_f4` stack (`T19`/`T22`), and run **G33-005** (SP freezing diagnostic) on Lead 2B CNFs.
4. **Flagship formalization:** Staff **G33-012** (Lean 4 algebraic-oracle lower bound + resistance atlas) as the synthesis publication for the prime-field track.

---

## 4. Proposed Pull Request Metadata

* **Suggested branch:** `research/g33-prime-field-ic-tranche3-20261122`
* **Suggested path:**
  * In `isogeny-labs/crypto-autoresearcher`: `research_goals_20261122_ic_tranche3_batch.md` (matches G31/G32 naming)
  * Or in `isogeny-labs/workspace`: `docs/plans/2026-11-22-prime-field-ic-tranche3-leads.md`
* **PR Title:** `research(ic): G33 tranche-3 prime-field ECDLP index-calculus blind spots and falsifier specs`
* **PR Description:**

```markdown
## Summary
Adds **G33 (Tranche 3)**: twelve prime-field ECDLP index-calculus frameworks and leads verified absent from `workspace/docs` and `crypto-autoresearcher/knowledge`, and distinct from the 120+ closed lanes in `SUCCESSOR_H_CONSTRUCTIVE_SUBEXP_T.md`.

- **Absent frameworks (G33-001..006):** AG list decoding (Guruswami–Sudan / Koetter–Vardy), Dixon resultants / Bézoutians / Chow forms, Gowers `U^3`/`U^4` uniformity norms, singular-curve `p`-adic homotopy continuation, Survey Propagation / 1-RSB diagnostics on Semaev-SAT, and leakage-assisted Coppersmith–Semaev IC.
- **Additional prime-field leads (G33-007..012):** Farey rational-reconstruction factor bases (`u_i/v_i`), SoS/Lasserre SDP relaxations, `a = 0` Waring/CP bilinear rank of `S_4`/`S_5`, banded/low-treewidth `m = 3` LA impossibility lemma, multi-curve CRT-Semaev desk-screen, and a Lean 4 Winterhof+AGM algebraic-oracle lower bound.

## Scope & Guardrails
- Planning record only; **no experiment results or security claims**.
- Every lead carries an explicit cheapest falsifier, kill criterion, and honest prior, subject to `G30-001`, `G30-028` (NoveltyGate), and `Gate A`/`Gate B`.
```
