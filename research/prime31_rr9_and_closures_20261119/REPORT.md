# Workstream 3 Report — Non-Enumerative `Prime31` `RR9` Extraction & Scoped Closures of Prime-Field Auxiliary Covers and Vertical-Isogeny Rarity

- **Date**: `2026-11-19`
- **Repository / Branch**: `isogeny-labs/crypto` — `cursor/prime31-rr9-and-closures-3858` (PR #4816, parent trunk `isogeny-labs/workspace#32`)
- **Standalone Native Rust Crate**: `research/prime31_rr9_and_closures_20261119/` (`0` Python in research/benchmark/verification paths; `0` edits to shared `src/cryptanalysis/` files) [pcc:unverified]
- **Deterministic Artifacts**: `research/prime31_rr9_and_closures_20261119/RESULTS.json` (`run_all_and_emit`), `tests/verification.rs` (`78/78` unit + integration tests pass with `RUSTFLAGS="-D warnings"`) [pcc:unverified]

---

## Executive Summary

1. **Lead P2-6 (`Prime31` `RR9` Non-Enumerative Extraction — Negative / Dominance-Closed vs Negation-Map Pollard $\rho$ & Shared BSGS)**:
   - On canonical `Prime31` ($p = 2^{31} - 1 = 2{,}147{,}483{,}647$, prime group order $n = 2{,}147{,}446{,}831$), exact triangular elimination of $(v_2, v_1, v_0)$ from the degree-9 Riemann–Roch identity $D_R(x) V(x)^2 - (x - r) A(x)^2 - 2 s A(x) V(x) = M_8(x)$ reduces the 7-variable chart $(a_0..a_3, v_0..v_2)$ to **4 variables** $(a_0, a_1, a_2, a_3)$ [pcc:unverified].
   - Fixed linear exterior-4 Plücker compression ($\Lambda^4(\mathbb{F}_p^8)$, dimension $\binom{8}{4} = 70$) is full rank **`70/70/70`** across planted positive and UNSAT residuals, ruling out linear subspace projection [pcc:unverified].
   - Both live **nonlinear non-enumerative extractors** — **(a) Reduced 4-Variable Sparse Krylov / Wiedemann** ($A = D_2 M^T D_1 M$ + Berlekamp–Massey + Horner kernel + $a_3$-eigen-splitting on the $(7,5,3,1)$-weighted Macaulay matrix at $W = 42$, compressing columns $4.51\times$ from $11{,}440$ to $2{,}538$ and peak memory $\sim 250\times$ from $51.5\text{ MB}$ dense pivots to $< 200\text{ KB}$) and **(b) Shared-Coefficient Quotient / Remainder Tower Extraction** (`DirectCubicQuotient`, `Reduced4VarQuotient`, and `YunSymmetricQuotientTower`) — achieve **100% exact set-level fibre agreement** (`0` forbidden factor-base tuple-enumeration calls) across blinded `B = 4` complete-fibre controls (single, 4-way opposite-sign collision, $8 P_1$ multiplicity-8 point, UNSAT) and `B = 8` controls (single, `corank = 2` distinct-abscissa collision, UNSAT, and a 16-residual blinded sweep), as well as a live end-to-end non-enumerative whole-solve (`[d]G == Q` on $E(\mathbb{F}_{257})$, $r = 281$, $d = 173$) [pcc:unverified].
   - **Whole-Solve Cost Verdict on `Prime31`**: Charging all failed/UNSAT random walks, the fastest exact non-enumerative `RR9` extractor at $B = 8$ (`DirectCubicQuotient`, $442$ Macaulay columns, $2.270 \times 10^6$ $\mathbb{F}_p$ ops / $7.35\text{ ms}$ per walk, $p_{\text{hit}} = 256/n = 1.192 \times 10^{-7}$) requires $1.714 \times 10^{14}$ expected $\mathbb{F}_p$ ops — **`9.886 \times 10^8\times` slower** than native negation-map Pollard $\rho$ ($173{,}387.8$ mean $\mathbb{F}_p$ muls, $51{,}863.5$ walk steps $= 1.263\sqrt{\pi n / 4}$, $5.07\text{ ms}$, $8/8$ `[d]G == Q`), **`1.211 \times 10^9\times` slower** than cold 1-target shared BSGS ($141{,}544.4$ $\mathbb{F}_p$ muls), and **`3.965 \times 10^9\times` slower** than online shared BSGS ($43{,}236.4$ $\mathbb{F}_p$ muls) (`SparseWiedemann4Var` at $1.764 \times 10^9$ muls/walk is $7.680 \times 10^{11}\times$ slower than $\rho$) [pcc:unverified]. Scaling $B \in \{12, 16, 24, 31\}$ raises $p_{\text{hit}}$ up to $0.610$, but introducing the $B - 8$ quotient variables $q_0 \dots q_{B-9}$ inflates F4 Macaulay width from $442$ cols ($B=8$) to $9{,}633 \to 42{,}619 \to 49{,}464 \to 120{,}696$ cols (`censored_timeout` at $2\text{ s}$/walk; even the strictly censored $2\text{ s}$-budget lower bound remains $\ge 1{,}188\times$ slower than negation-map $\rho$ at $B = 31$) [pcc:unverified].

2. **Lead P1-4 Part (a) (Same-Field Auxiliary Plane-Quartic Cover Audit & NIST P-192 Closure — Closed Asymptotically & Concretely)**:
   - Native Rust verification across a 4-prime toy ladder ($p \in \{1009, 2003, 4001, 10007\}$), explicit smooth plane quartic $\operatorname{Pic}^0(C)(\mathbb{F}_{1009})$ ($3{,}245.24$ $\mathbb{F}_p$ muls/add, BSGS recovers $k = 323$), and native 192-bit `[u64; 3]` NIST P-192 (`[r]G == O`, $\phi_* \circ \phi^* = [2]_E$ for $\gamma \in \{1, 7\}$) confirms exact Poincaré Jacobian inflation $\#\operatorname{Jac}(C)(\mathbb{F}_p) = r \cdot \#\operatorname{Prym}(\mathbb{F}_p) = \Theta(p^3)$ (`+384.0` bits on P-192) and Transport Conservation [pcc:unverified].
   - On NIST P-192, Diem's plane-quartic 2LP index calculus requires $\widetilde{O}(p) \ge 2^{192.00}$ steps ($\ge 2^{203.66}$ $\mathbb{F}_p$ muls at $3{,}245.24$ muls/add) vs $2^{95.83}$ steps ($2^{99.15}$ $\mathbb{F}_p$ muls) for negation-map Pollard $\rho$ on $E(\mathbb{F}_p)$ — an exact **`96.17`-bit unweighted step gap** (**`104.52`-bit $\mathbb{F}_p$-mul-weighted gap**) against the cover route [pcc:unverified].

3. **Lead P1-4 Part (b) (Deuring-Weighted Large-Prime Vertical-Isogeny Rarity & Conductor Cap — Certified Closed)**:
   - Exact native Kronecker–Hurwitz class-number enumeration across a 12-prime geometric ladder $p \in [10{,}007,\, 999{,}983]$ (cross-checked $100\%$ against direct reduced binary-quadratic-form enumeration for $1 \le n \le 1{,}000$) satisfies $\sum_{|t| \le 2\sqrt{p}} H(4p - t^2) = 2p$ (`sum_6h == 12p`) on **`12/12` primes** and certifies $\max\{\ell \mid f_0\} \le f_0 \le \lfloor 2\sqrt{p/3}\rfloor \le 2\sqrt{p}$ with **`0` violations** (attained sharply at $p = 398{,}113$, where $\max f_0 = 728 = \lfloor 2\sqrt{p/3}\rfloor$) [pcc:unverified].
   - The $p$-averaged fraction of ordinary curves over $\mathbb{F}_p$ whose Frobenius conductor $f_0 = [\mathcal{O}_K : \mathbb{Z}[\pi]]$ has a prime divisor $\ell \ge L$ fits $\bar{\mu}(L) \approx c / (L \ln L)$ with **`c = 0.8454` (`R^2 = 0.9650`)**, matching Gekeler's local Euler product $1 - \prod_{\ell \ge L}(1 - 1/(\ell^2 - 1)) \sim 1/(L \ln L)$ ($c \to 1$), while every $\mathbb{F}_p$-rational isogeny preserves $\#E'(\mathbb{F}_p) = \#E(\mathbb{F}_p)$ by Tate's theorem (`0` group-order reduction) [pcc:unverified].

---

## Section 1 — Lead P2-6: Non-Enumerative `Prime31` `RR9` Extraction

### 1.1 Algebraic Reduction (`7 -> 4` Variables) & Exterior-4 Plücker `70/70/70` Obstruction

Let $E / \mathbb{F}_p : y^2 = f(x) = x^3 + a x + b$ be canonical `Prime31` ($p = 2^{31}-1$, $a = 847{,}514{,}846$, $b = 2{,}087{,}444{,}724$, prime order $n = 2{,}147{,}446{,}831$, $G = (625{,}037{,}715,\, 2{,}100{,}627{,}251)$). An 8-point relation $\sum_{j=1}^8 \varepsilon_j P_{i_j} = R = (r, s)$ ($\varepsilon_j \in \{\pm 1\}$) is witnessed by a section $F(x, y) = (x - r) A(x) + (y + s) V(x) \in \mathcal{L}(9\mathcal{O} - (-R))$ with $\deg A \le 3$ and monic $V(x) = x^3 + v_2 x^2 + v_1 x + v_0$. Dividing the norm identity $U(x)^2 - f(x) V(x)^2$ by $(x - r)$ using $f(x) - s^2 = (x - r) D_R(x)$ where $D_R(x) = x^2 + r x + (r^2 + a)$ yields the exact monic degree-8 abscissa polynomial:
\[
M_8(x) \;=\; D_R(x)\,V(x)^2 \;-\; (x - r)\,A(x)^2 \;-\; 2 s\,A(x)\,V(x) \;=\; x^8 + m_7 x^7 + \dots + m_0.
\]
At $B = 8$, $M_8(x) = H_{\mathcal{F}}(x) = \prod_{i=1}^8 (x - x_i)$ is fixed, and the top three coefficients $(m_7, m_6, m_5) = (h_7, h_6, h_5)$ are triangular linear in $(v_2, v_1, v_0)$ with invertible constant pivot $+2 \not\equiv 0 \pmod p$:
\[
v_2 = \tfrac{1}{2}(a_3^2 - r + h_7), \qquad v_1 = \text{poly}(a_2, a_3), \qquad v_0 = \text{poly}(a_1, a_2, a_3).
\]
Assigning weights $\operatorname{wt}(a_0, a_1, a_2, a_3) = (7, 5, 3, 1)$ gives weighted degrees $\operatorname{wt}(v_2, v_1, v_0) = (2, 4, 6)$ and weighted generator degrees $(14, 12, 10, 8, 6)$ for the 5 remaining equations $g_0, \dots, g_4$ (`src/rr9_wiedemann.rs`) [pcc:unverified].

- **Plücker `70/70/70` Audit**: Evaluating the residual-conditioned nodal operator on $\Lambda^4(\mathbb{F}_p^8)$ produces rank **`70/70/70`** (`exterior_dim = 70`) on planted positive, `G`, and `[2]G` residuals (`RESULTS.json`), confirming no fixed linear hyperplane survives projection [pcc:unverified].
- **Sharp Weighted Macaulay Regularity Threshold**: Scanning weight bounds $W \in \{36, 39, 40, 41, 42\}$ on `Prime31` pinpoints **$W = 41$** ($2{,}341$ cols, $4{,}140$ rows, $\text{nnz} = 151{,}259$) as the exact threshold where UNSAT collapses to `corank = 0` (`unit_pivot_found = true`) while positive instances retain `corank = d` (`pos_extracted = true`); we run $W = 42$ ($2{,}538$ cols, $4{,}594$ rows, $\text{nnz} = 168{,}274$, a **`4.51x` column reduction** vs the $11{,}440$-column unreduced 7-variable degree-9 Macaulay matrix) [pcc:unverified].

| Weight Bound $W$ | Rows | Cols | Nonzeros (`nnz`) | Positive Corank | Positive Extracted | UNSAT Corank | UNSAT Unit-Certified |
|---:|---:|---:|---:|---:|:---:|---:|:---:|
| 36 | 2,351 | 1,523 | 84,617 | 7 | true | 7 | false |
| 39 | 3,333 | 1,982 | 121,104 | 2 | true | 2 | false |
| 40 | 3,720 | 2,156 | 135,550 | 1 | true | 1 | false |
| **41** | **4,140** | **2,341** | **151,259** | **1** | **true** | **0** | **true** |
| **42** | **4,594** | **2,538** | **168,274** | **1** | **true** | **0** | **true** |

[pcc:unverified]

### 1.2 Head-to-Head at `B = 8` & Blinded `B = 4, 8` Complete-Fibre Verification

We evaluate four non-enumerative extractors at $B = 8$ on `Prime31` (plus `YunSymmetricQuotientTower` across universal degree-8 multiplicity partitions $\sum_{k=1}^8 k d_k = 8$ at $B = 4$, where repeated factor-base points are unavoidable) against a sealed `grader_only::CompleteFibreOracle` (`0` candidate tuple-enumeration calls recorded by atomic guard `FORBIDDEN_TUPLE_ENUM_CALLS`):
1. **`SparseWiedemann4Var`**: Symmetrized preconditioned operator $A = D_2 M^T D_1 M \in \mathbb{F}_p^{2538 \times 2538}$, $2 N = 5{,}076$ Krylov moments, Berlekamp–Massey over $\mathbb{F}_p$, Horner kernel extraction $w = (P_{\min}(A)/A) v$, and multi-seed Horner + $a_3$-shift eigen-splitting when `corank > 1` [pcc:unverified].
2. **`Dense4Var`**: Incremental dense row-echelon elimination + back-substitution on the identical $4{,}594 \times 2{,}538$ matrix ($51.5\text{ MB}$ dense pivot table) [pcc:unverified].
3. **`DirectCubicQuotient`**: Polynomial quotient formulation $Q(x) M_8(x) = H_{\mathcal{F}}(x)$ in $7 + (B - 8)$ variables (`0` redundant $m_k$ variables; at $B = 8$, $Q(x) \equiv 1$ so the 8 equations in 7 variables are **pure quadratic**, solving in $442$ F4 columns) [pcc:unverified].
4. **`Reduced4VarQuotient`**: Eliminates $(v_2, v_1, v_0)$ symbolically from $Q(x) M_8(x) = H_{\mathcal{F}}(x)$ before Grevlex F4 ($4 + (B - 8)$ variables; eliminating $(v_2, v_1, v_0)$ raises unweighted $a_3$-degree to $12$, solving at degree $5$ post-interreduction in $547$ F4 columns with `degree_cap >= 16`) [pcc:unverified].

Every extracted 7-chart $[a_0..a_3, v_0..v_2]$ is decoded into a signed multiset via local Taylor/Hermite vanishing orders of $F(x, y)$ on $E$ (`decode_and_verify_7chart_relation`, handling repeated abscissas, $\pm P_i$ opposite pairs, and $x_i = r$ without ambiguity) and checked by curve-addition replay $\sum \varepsilon_j P_{i_j} == R$ [pcc:unverified].

| Blinded Case (`Prime31`) | Expected Fibre Size | Extractor | Decoded Size | Exact Set Match | Charged $\mathbb{F}_p$ Ops / Muls | Peak Cols | Wall Time (ms) |
|---|---:|---|---:|:---:|---:|---:|---:|
| `B4_complete_single_fibre` | 1 | `YunSymmetricQuotientTower` | 1 | true | 3,942,550,373 | 124 | 35,578.6 |
| `B4_complete_4way_collision_fibre` | 4 | `YunSymmetricQuotientTower` | 4 | true | 3,945,106,933 | 124 | 28,178.7 |
| `B4_complete_8x_repeated_point_fibre` | 1 | `YunSymmetricQuotientTower` | 1 | true | 3,942,074,401 | 124 | 27,142.0 |
| `B4_complete_unsat_G` | 0 | `YunSymmetricQuotientTower` | 0 | true | 3,942,059,524 | 124 | 20,864.3 |
| `B8_positive_single` | 1 | `SparseWiedemann4Var` | 1 | true | 2,642,367,352 | 2,538 | 2,708.9 |
| `B8_positive_single` | 1 | `Dense4Var` | 1 | true | 1,726,818,874 | 2,538 | 1,735.6 |
| `B8_positive_single` | 1 | `DirectCubicQuotient` | 1 | true | 2,270,638 | 442 | 103.5 |
| `B8_positive_single` | 1 | `Reduced4VarQuotient` | 1 | true | 2,445,218 | 547 | 57.0 |
| `B8_positive_corank2_collision` | 2 | `SparseWiedemann4Var` | 2 | true | 4,398,370,303 | 2,538 | 4,528.7 |
| `B8_positive_corank2_collision` | 2 | `Dense4Var` | 2 | true | 1,727,269,736 | 2,538 | 1,660.5 |
| `B8_positive_corank2_collision` | 2 | `DirectCubicQuotient` | 2 | true | 2,273,289 | 442 | 126.6 |
| `B8_positive_corank2_collision` | 2 | `Reduced4VarQuotient` | 2 | true | 2,462,354 | 547 | 51.9 |
| `B8_negative_unsat_G` | 0 | `SparseWiedemann4Var` | 0 | true | 1,763,844,012 | 2,538 | 1,763.1 |
| `B8_negative_unsat_G` | 0 | `Dense4Var` | 0 | true | 1,541,476,763 | 2,538 | 1,527.2 |
| `B8_negative_unsat_G` | 0 | `DirectCubicQuotient` | 0 | true | 2,270,486 | 442 | 130.0 |
| `B8_negative_unsat_G` | 0 | `Reduced4VarQuotient` | 0 | true | 2,422,148 | 547 | 15.9 |
| `B8_batch16` (8 SAT + 8 UNSAT) | 16 cases | `DirectCubicQuotient` | `16/16` | true | `2.27M` / call | 442 | `~7.3` |

[pcc:unverified]

### 1.3 Live End-to-End Non-Enumerative Whole-Solve & `Prime31` Scaling Across `B = {8, 12, 16, 24, 31}`

- **Live End-to-End Whole-Solve Proof (`E(F_257)`, $y^2 = x^3 + x + 7$, prime order $r = 281$, planted $d = 173$)**: Running `DirectCubicQuotient` ($B = 8$, `0` forbidden calls) on random walks $R_k = [u_k]G + [v_k]Q$ collects $9$ independent relations mod $281$ in $15$ walks ($8$ SAT + $7$ failed/UNSAT walks all charged $= 36{,}310{,}512$ $\mathbb{F}_p$ ops, $63.5\text{ ms}$) and solves `ModLinearSystem` to recover **`d = 173` (`[173]G == Q`)**, compared against $108$ muls for shared BSGS and $2{,}785$ muls for negation-map $\rho$ [pcc:unverified].
- **`Prime31` Native Generic Baselines (`8/8` planted scalars in `[1, n-1]`, `100%` `[d]G == Q`)**:
  - **`NegationSharedBsgs` ($m = 32{,}768$)**: Cold setup $32{,}769$ curve adds / $98{,}308$ $\mathbb{F}_p$ muls ($2.93\text{ ms}$); online query mean $14{,}350.5$ giant steps / **`43,236.4` $\mathbb{F}_p$ muls ($1.27\text{ ms}$)**; cold 1-target total **`141,544.4` $\mathbb{F}_p$ muls ($4.21\text{ ms}$)** [pcc:unverified].
  - **`negation_pollard_rho` ($E / \{\pm 1\}$ 32-branch r-adding walk + zero-lookahead Wiener–Zuccherato 1/2-cycle breaking + stateless 4-register Brent cycle guard)**: Mean **`51,863.5` walk steps** ($1.263 \times \sqrt{\pi n / 4} = 41{,}068.25$), **`173,387.8` $\mathbb{F}_p$ muls ($5.07\text{ ms}$)** [pcc:unverified].

| $B$ | Distinct 8-Tuples $2^8 \binom{B}{8}$ | Hit Prob $p_{\text{hit}}$ | Expected Walks for $B+1$ Rels | `DirectCubicQuotient` Vars / Max Cols / Status | `Reduced4Var` Vars / Max Cols / Status | PR #4692 Frozen Vars / Max Cols | Charged Whole-Solve $\mathbb{F}_p$ Ops (LB if censored) | Ratio vs `Prime31` Negation $\rho$ Muls | Ratio vs Shared BSGS Online Muls |
|---:|---:|---:|---:|---|---|---|---:|---:|---:|
| **8** | 256 | $1.19 \times 10^{-7}$ | $7.55 \times 10^7$ | `7` / `442` / `complete_unsat` | `4` / `547` / `complete_unsat` | `52` / `30,611` (`timeout`) | $1.714 \times 10^{14}$ (exact) | **$9.89 \times 10^8\times$** | **$3.96 \times 10^9\times$** |
| **12** | 126,720 | $5.90 \times 10^{-5}$ | $2.20 \times 10^5$ | `11` / `9,633` / `censored_timeout` | `8` / `27,304` / `censored_timeout` | `56` / `33,604` (`timeout`) | $\ge 1.394 \times 10^{14}$ (LB) | $\ge 8.04 \times 10^8\times$ | $\ge 3.22 \times 10^9\times$ |
| **16** | 3,294,720 | $1.53 \times 10^{-3}$ | $1.11 \times 10^4$ | `15` / `42,619` / `censored_timeout` | `12` / `41,826` / `censored_timeout` | `60` / `45,192` (`timeout`) | $\ge 1.274 \times 10^{13}$ (LB) | $\ge 7.35 \times 10^7\times$ | $\ge 2.95 \times 10^8\times$ |
| **24** | 188,280,576 | $8.39 \times 10^{-2}$ | $297.8$ | `23` / `49,464` / `censored_timeout` | `20` / `25,103` / `censored_timeout` | `68` / `74,653` (`timeout`) | $\ge 3.427 \times 10^{10}$ (LB) | $\ge 1.98 \times 10^5\times$ | $\ge 7.93 \times 10^5\times$ |
| **31** | 2,019,513,600 | $0.610$ | $52.5$ | `30` / `120,696` / `censored_timeout` | `27` / `25,103` / `censored_timeout` | `75` / `47,825` (`timeout`) | $\ge 2.061 \times 10^8$ (LB) | $\ge 1.19 \times 10^3\times$ | $\ge 4.77 \times 10^3\times$ |

[pcc:unverified]

**Structural Root Cause of Lead P2-6 Closure**: At $B = 8$, $Q(x) \equiv 1$ keeps the polynomial system small ($442$ F4 cols or $2{,}538$ weighted-Macaulay cols), but $2^8 \binom{8}{8} = 256$ gives $p_{\text{hit}} \approx 1.19 \times 10^{-7}$, requiring $\sim 7.55 \times 10^7$ walks ($\sim 10^9\times$ worse than $\rho$). Raising $B > 8$ increases $p_{\text{hit}}$ only by introducing $B - 8$ unknown quotient coefficients $q_0 \dots q_{B-9}$ in $Q(x) M_8(x) = H_{\mathcal{F}}(x)$, turning a 7-variable quadratic system into a $(B-1)$-variable cubic complete intersection whose Gröbner/Macaulay complexity grows exponentially in $B - 8$ [pcc:unverified].

---

## Section 2 — Lead P1-4 Part (a): Same-Field Auxiliary Plane-Quartic Cover Audit & NIST P-192 Closure

For any short-Weierstrass prime-field curve $E / \mathbb{F}_p : y^2 = x^3 - 3x + b$ with prime order $r$, the degree-2 bielliptic/plane-quartic cover $C / \mathbb{F}_p$ ($z^4 - \gamma = (x-\alpha_1)(x-\alpha_2)(x-\alpha_3)$ on the hyperelliptic model $H$, embedded canonically in $\mathbb{P}^2$ as a smooth genus-3 plane quartic) induces pullback $\phi^* : E(\mathbb{F}_p) \to \operatorname{Jac}(C)(\mathbb{F}_p)$ and norm/pushforward $\phi_* : \operatorname{Jac}(C)(\mathbb{F}_p) \to E(\mathbb{F}_p)$ satisfying $\phi_* \circ \phi^* = [2]_E$ and Poincaré isogeny decomposition $\operatorname{Jac}(C) \sim E \times \operatorname{Prym}(C/E)$ (`\dim \operatorname{Prym} = g - 1 = 2`) [pcc:unverified].

### 2.1 Toy Cover Ladder (`p = 1009, 2003, 4001, 10007`) & Explicit `Pic^0(C)(F_1009)` Cost Audit

| Prime $p$ | $b$ | $\gamma$ | Prime Order $r = \#E(\mathbb{F}_p)$ | $\#C(\mathbb{F}_p)$ | Exact $\#\operatorname{Prym}(\mathbb{F}_p)$ | Exact $\#\operatorname{Jac}(C)(\mathbb{F}_p) = r \cdot \#\operatorname{Prym}$ | Inflation $\log_2(\#\operatorname{Jac}/r)$ | $\phi^*,\phi_*$ Hom & $\phi_*\phi^*=[2]$ | Transport Conservation |
|---:|---:|---:|---:|---:|---:|---:|---:|:---:|:---:|
| 1,009 | 4 | 16 | 967 | 918 | 970,720 | 938,686,240 | $+19.89$ bits | PASS | PASS |
| 2,003 | 1 | 3 | 2,039 | 2,034 | 4,000,080 | 8,156,163,120 | $+21.93$ bits | PASS | PASS |
| 4,001 | 9 | 5 | 4,079 | 4,082 | 16,023,852 | 65,361,292,308 | $+23.93$ bits | PASS | PASS |
| 10,007 | 7 | 9 | 10,193 | 10,218 | 100,409,808 | 1,023,477,172,944 | $+26.58$ bits | PASS | PASS |

[pcc:unverified]

On the explicit smooth plane quartic $C / \mathbb{F}_{1009}$ (`src/cryptanalysis/nist_cover_quartic.rs`), native Khuri-Makdisi $\operatorname{Pic}^0(C)(\mathbb{F}_{1009})$ verification confirms projective smoothness (`smoothness_verified = true`), `#C(F_1009) = 918`, exact BSGS recovery of planted $k = 323$ on $\phi^*(E)[967]$, and exact per-operation arithmetic counts over $250$ group additions/doublings: **`3,245.24` $\mathbb{F}_p$ muls**, **`42.95` $\mathbb{F}_p$ invs**, and **`3,509.32` $\mathbb{F}_p$ adds** per `Pic^0` addition (`5,117.14` $\mathbb{F}_p$ muls per `Pic^0` doubling; counting `1 inv ~ 130 muls` on 192-bit fields matches the $\sim 8{,}800$–$8{,}900$ effective muls/op figure) [pcc:unverified].

### 2.2 Actual 192-bit NIST P-192 Verification & Asymptotic/Concrete Gap Closure

- **Native 192-bit Verification**: Using `[u64; 3]` modular arithmetic (`src/cover_and_deuring.rs`), $G \in E_{\text{P-192}}(\mathbb{F}_p)$, $[r]G = \mathcal{O}$, and explicit cover norm $\phi_*(\phi^*(P)) = [2]P$ over $\mathbb{F}_{p^2}$ pass for all test multiples $[k]G$ under both $\gamma = 7$ and $\gamma = 1$ [pcc:unverified].
- **Exact P-192 Work Comparison**:
  - Group inflation: $\#\operatorname{Jac}(C)(\mathbb{F}_p) = r \cdot \#\operatorname{Prym}(\mathbb{F}_p) \in [2^{576.00 - 10^{-28}},\, 2^{576.00 + 10^{-28}}]$ (**`+384.00` bits** of 2-dimensional Prym cofactor over $r \approx 2^{192.00}$); generic $\rho$ on $\operatorname{Jac}(C)(\mathbb{F}_p)$ costs $\Theta(p^{3/2}) \approx 2^{288}$ steps, while Transport Conservation blocks un-projected cofactor masking without knowing $r$ or annihilating $\operatorname{Prym}(\mathbb{F}_p)$ [pcc:unverified].
  - Diem (2006) plane-quartic double-large-prime index calculus over $\mathbb{F}_p$: $\widetilde{O}(p) \ge 2^{192.00}$ unweighted steps and **$\ge 2^{203.66}$ $\mathbb{F}_p$ muls** (even at unit polylog constant and charging $3{,}245.24$ $\mathbb{F}_p$ muls per `Pic^0` op) [pcc:unverified].
  - Native negation-map Pollard $\rho$ on $E_{\text{P-192}}(\mathbb{F}_p)$: $\sqrt{\pi r / 4} = \mathbf{2^{95.83}}$ steps (**$2^{99.15}$ $\mathbb{F}_p$ muls** at $10$ $\mathbb{F}_p$ muls/step) [pcc:unverified].
  - **Closure Gap**: **`96.17` bits** unweighted (**`104.52` bits** $\mathbb{F}_p$-mul-weighted) in favor of $E(\mathbb{F}_p)$ negation-map Pollard $\rho$ — **closed asymptotically ($\widetilde{O}(p)$ vs $O(p^{1/2})$) and concretely** for all prime-field elliptic sources [pcc:unverified].

---

## Section 3 — Lead P1-4 Part (b): Deuring-Weighted Large-Prime Vertical-Isogeny Rarity & Conductor Cap

For an ordinary trace $t \in (-2\sqrt{p}, 2\sqrt{p}) \setminus \{0\}$ ($\Delta_\pi = t^2 - 4p = f_0^2 d_K < 0$), Deuring's theorem weights the $\mathbb{F}_p$-isomorphism classes of curves with trace $t$ (weighted by $2/\#\operatorname{Aut}_{\mathbb{F}_p}(E)$) by the Kronecker–Hurwitz class number $H(4p - t^2) = 2 \sum_{d \mid f_0} h_{\text{w}}(f_0^2 d_K / d^2)$, and an ascending/vertical prime-$\ell$ $\mathbb{F}_p$-rational isogeny changing the endomorphism conductor exists iff $\ell \mid f_0 = [\mathcal{O}_K : \mathbb{Z}[\pi]]$ [pcc:unverified].

### 3.1 Exact 12-Prime Ladder (`10,007 .. 999,983`): Trace Identity & Conductor Cap

- **Direct Oracle Cross-Check**: `six_hurwitz_class_number(n)` matches direct enumeration of reduced positive-definite binary quadratic forms $a x^2 + b x y + c y^2$ of discriminant $-n$ (`six_hurwitz_direct_forms(n)`) for **all $1 \le n \le 1{,}000$** (`100%` exact match) [pcc:unverified].
- **Exact Kronecker–Hurwitz Identity & Conductor Cap (`12/12` primes, `0` violations)**: Since $d_K \le -3$ and $t \ne 0$ for ordinary curves over prime fields $\mathbb{F}_p$ ($p \ge 5$), $3 f_0^2 \le |d_K| f_0^2 = 4p - t^2 \le 4p - 1$, proving the sharp universal cap:
\[
\max\{\ell \text{ prime} : \ell \mid f_0\} \;\le\; f_0 \;\le\; \left\lfloor 2\sqrt{\frac{p}{3}} \right\rfloor \;<\; 2\sqrt{p}.
\]

| Prime $p$ | Hasse $\lfloor 2\sqrt{p}\rfloor$ | Exact $\sum_t 6 H(4p-t^2)$ | Expected $12 p$ | Identity Exact | Max Observed $f_0$ | Max Prime $\ell \mid f_0$ | Sharp Cap $\lfloor 2\sqrt{p/3}\rfloor$ | Cap Certified |
|---:|---:|---:|---:|:---:|---:|---:|---:|:---:|
| 10,007 | 200 | 120,084 | 120,084 | true | 26 | 23 | 115 | true |
| 15,877 | 252 | 190,524 | 190,524 | true | 132 | 17 | 145 | true |
| 25,117 | 316 | 301,404 | 301,404 | true | **183** | 149 | **183** | true |
| 39,821 | 399 | 477,852 | 477,852 | true | 190 | 61 | 230 | true |
| 63,103 | 502 | 757,236 | 757,236 | true | 289 | 113 | 290 | true |
| 100,003 | 632 | 1,200,036 | 1,200,036 | true | 323 | 103 | 365 | true |
| 158,489 | 796 | 1,901,868 | 1,901,868 | true | 283 | 283 | 459 | true |
| 251,179 | 1,002 | 3,014,148 | 3,014,148 | true | 577 | 577 | 578 | true |
| 398,113 | 1,261 | 4,777,356 | 4,777,356 | true | **728** | 617 | **728** | true |
| 630,967 | 1,588 | 7,571,604 | 7,571,604 | true | 882 | 659 | 917 | true |
| 800,011 | 1,788 | 9,600,132 | 9,600,132 | true | 969 | 397 | 1,032 | true |
| 999,983 | 1,999 | 11,999,796 | 11,999,796 | true | 334 | 257 | 1,154 | true |

[pcc:unverified]

### 3.2 $p$-Averaged Tail Decay $\bar{\mu}(L) \sim c / (L \ln L)$ & Tate Group-Order Invariance

| Threshold $L$ | Eligible Unclipped Primes | Empirical Mean $\bar{\mu}(L)$ | Gekeler Euler Product $1 - \prod_{\ell \ge L}(1 - \frac{1}{\ell^2 - 1})$ | Fitted $c / (L \ln L)$ ($c = 0.8454$) | Scaled $\bar{\mu}(L) \cdot L \ln L$ |
|---:|---:|---:|---:|---:|---:|
| 5 | 12 | 0.09614 | 0.09021 | 0.10505 | 0.7737 |
| 7 | 12 | 0.06588 | 0.05065 | 0.06206 | 0.8973 |
| 11 | 12 | 0.03531 | 0.03045 | 0.03205 | 0.9314 |
| 17 | 12 | 0.01551 | 0.01645 | 0.01755 | 0.7471 |
| 23 | 12 | 0.00871 | 0.01027 | 0.01172 | 0.6283 |
| 31 | 12 | 0.00485 | 0.00721 | 0.00794 | 0.5158 |
| 47 | 12 | 0.00300 | 0.00432 | 0.00467 | 0.5420 |
| 67 | 11 | 0.00243 | 0.00296 | 0.00300 | 0.6841 |
| 97 | 9 | 0.00215 | 0.00192 | 0.00191 | 0.9542 |
| 137 | 8 | 0.00122 | 0.00126 | 0.00125 | 0.8254 |
| 197 | 6 | 0.00103 | 0.00080 | 0.00081 | 1.0743 |
| 277 | 5 | 0.00058 | 0.00054 | 0.00054 | 0.8974 |

[pcc:unverified]

- **Tail Fit & Closure**: Zero-intercept OLS against $1 / (L \ln L)$ over $L \in [11, 277]$ yields **`c = 0.8454` with `R^2 = 0.9650`**, tracking Gekeler's exact local density $1 - \prod_{\ell \ge L}(1 - 1/(\ell^2 - 1)) \sim 1 / (L \ln L)$ ($c_{\text{Gekeler}} = 1.0$) [pcc:unverified]. Furthermore, any $\mathbb{F}_p$-rational isogeny $\psi : E \to E'$ preserves trace $t(E') = t(E)$ and group order $\#E'(\mathbb{F}_p) = p + 1 - t = \#E(\mathbb{F}_p)$ (Tate's isogeny theorem), while Vélu / $\sqrt{\text{élu}}$ evaluation costs $\widetilde{O}(\ell)$ ($\text{deg }\Phi_\ell = \ell + 1$), closing vertical-isogeny preconditioning as an ECDLP shortcut on prime-order curves [pcc:unverified].

---

## Reproducibility Commands

```bash
export CARGO_TARGET_DIR=/tmp/target-crypto-rr9-3858
export RAYON_NUM_THREADS=1

# 1. Compile all binaries and run all 78 unit + integration tests with 0 warnings:
RUSTFLAGS="-D warnings" cargo test --release \
  --manifest-path research/prime31_rr9_and_closures_20261119/Cargo.toml \
  -- --test-threads=1

# 2. Regenerate deterministic RESULTS.json:
RUSTFLAGS="-D warnings" cargo run --release \
  --manifest-path research/prime31_rr9_and_closures_20261119/Cargo.toml \
  --bin run_all_and_emit
```
