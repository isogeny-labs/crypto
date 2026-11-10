# Desk audit: hashed-factor-base ECDL by Boolean tensor contraction (Akshima et al., Bochum v0.2)

Date 2026-11-07. Slug `hetaudit`. Lead: `knowledge/leads/ecdlp/exotic/LEAD-EXO-hashed-factor-base-tensor-ecdl.yaml`,
audit step (a). Every number quoted from the repositories is `[pcc:unverified]`. Labels:
**[proved here]** re-derived in this audit from the pinned sources; **[cited]** taken from a source and not
re-derived; **[heuristic]** argued, not proved; **[checked numerically]** exact integer arithmetic in
`checks/hetaudit_checks.py` (output `checks/hetaudit_checks.out`, 0.05 s).

## 0. Sources actually read

| Source | Pin | How read |
| --- | --- | --- |
| Akshima, Bacho, Huang, Kiltz, Nishaburi, Nooripoor, Tittelbach, Wiedijk, *A Subexponential Time Algorithm for Elliptic Curve Discrete Logarithm*, v0.2 (2026-10-29) | `bochum-crypto/subexponential-ecdl` @ `1ee44e562b1b9a50ea339f3b8f81112b5a24b96f` (cloned, HEAD = pin; only file `subexponential-ecdl.pdf`, SHA-256 `82e27362…deaa11`) | `pdftotext -layout`, 1,248 lines, read in full (`sources/bochum.txt`) |
| OpenAI, *The Exponential Time Hypothesis is False*, 2026-10-25 | `openai/math` @ `7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a`, directory `preprints/The-Exponential-Time-Hypothesis-is-False-October-25-2026` (sparse clone; `git diff` HEAD..pin on that directory is empty) | TeX: `build/quantitative-reduction.tex` (Theorem D.1 statement, lines 26–47) and `build/quantitative-contraction.tex` (Appendix D.4, 619 lines) read in full; `main_full_version.tex` only grepped |
| Lead atlas, clustering note, planted-leak note, LEADS/SYNTHESIS/SKEPTIC_DIGEST, L06, L09 (`table2.json`), problem file, cryptanalysis `docs/hashed-ecdl.md`, crate sources `crates/hashed-ecdl-tensor/src/{encoding,reduction,admission}.rs`, Python companion receipts `experiments/hashed-ecdl-tensor-reference/prime-development/challenge-tensor.{txt,json}` | repo checkouts on this machine | read |

Copies with hashes: `sources/SHA256SUMS`. Neither PDF had been pinned in the repos before (LEADS §9 erratum 5
and L09 both record "Bochum text not on disk"); the earlier repo readings were from a private download.
Claims below that the repo notes make and that I could not reproduce are listed in §7.

## 1. The claim, precisely

### 1.1 The ETH-preprint theorem invoked (Theorem D.1 = Bochum Theorem 2) [cited, statement transcribed]

Fix positive constants C, A, b (the preprint's letter b; it is a polynomial exponent, unrelated to Bochum's
b = ⌈log p⌉). Let s ≥ 2 and let 𝓛 be the total description length of a **bipartite** Boolean tensor network
with at most Cs edges, maximum degree at most ⌈s^{9/10}⌉, polynomially many vertices, entries in
(ℤ/2^{P₀})[V]/(V^ℓ) with 1 ≤ P₀, ℓ ≤ Cs, and every entry computable from its description in time
(s+𝓛)^b 2^{A s^{9/10}} and space (s+𝓛)^b, uniformly over vertices and port assignments. Then the contraction
is computed deterministically in (s+𝓛)^{O(1)} 2^{O(s/√log s)} bit operations and (s+𝓛)^{O(1)} space; the
constants depend only on C, A, b.

Quantifiers that matter for the audit: the O(·) constants are **unspecified** functions of (C, A, b); the proof
(D.4) states "all estimates below hold for sufficiently large s, with the threshold depending only on the fixed
constants. A fixed finite range of smaller sizes is handled by direct summation" (quantitative-contraction.tex
l. 76–79). Direct summation costs 2^{Cs}. The explicit parameters (l. 80–105):
λ = ⌈log₂ s⌉, H = ⌈s/⌊√λ⌋⌉ (leaf threshold), r = 64, ε = 2^{-17}, Δ = ⌈s^{9/10}⌉,
u_d = ⌊ελ/2^d⌋ at depth d, a_d = 2Σ_{j≤d} u_j, L = 2^{a_d}, D_arr = 64^{2a_d}; children have ≤ M/8 edges;
and the standing bounds "for all sufficiently large s": λ^{4/5} ≤ u ≤ M, a ≤ 4ελ, L, D_arr ≤ s^{1/1000},
M ≥ 2D_arr²Δ, M ≥ L·D_arr, 2L ≤ Δ (eq. tensor-parameter-bounds / tensor-packing-conditions).

### 1.2 The Bochum reduction (Theorem 4, Lemmas 1–13) [cited, statement transcribed]

E/𝔽_p, p > 3, short Weierstrass, #E(𝔽_p) = r prime, b = ⌈log₂ p⌉. For every integer m ≥ 2 "not too large
compared with b" (Lemma 11/13: 2 ≤ m ≤ ⌈(log b)^{1/4}⌉, "for sufficiently large b"), there is a randomised
algorithm that returns ξ with Q = [ξ]P, never a wrong value, using in expectation O(m² b r^{1/m}) solver calls
on networks with O(mb) edges and degree O(b^{3/4}); O(m³ r^{3/m} b²) bit operations of Gaussian elimination;
space O(m² r^{2/m} b). Under Theorem 2 the expected time is 2^{O(b/m + mb/√log b)}; m = ⌈(log b)^{1/4}⌉ gives
2^{O(b/(log b)^{1/4})}. The constants are inherited from Theorem 2 and are unspecified. Fixed cofactor c:
same bound with A_i = [c]S_i certified inside the network (O(log c) extra additions) and [r]A = O checked
during factor-base enumeration.

The manuscript's own limitation (§1.2): "the proof of Theorem 2 yields a saving only for networks of size at
least 2^{2^17}", footnote 4: "an internal parameter u₀ = ⌊b/2^17⌋ (defined in D.4.2) is zero".

## 2. Size s of the Boolean encoding as a function of b and m

### 2.1 From the manuscript's constructions [proved here] [checked numerically]

Theorem 3 (integer multiplication, d-bit inputs). Parameters (Lemma 5): J = least power of two with
J² ≥ 64(d+1), k = J/4, n = ⌈d/k⌉, L = least power of two ≥ 2n, L₁ = 2^{⌊µ/2⌋}, L₂ = 2^{⌈µ/2⌉}, ring ℤ/(2^J+1),
ω = 2^{2J/L}. I re-checked Lemma 5 parts 1, 2 and 4 as integer inequalities for every d ≤ 5000 (all hold) and
part 3 (ω principal L-th root, Σ_i ω^{ji} = 0 for 0 < j < L) exactly for (J, L) ∈ {(16,8), (32,16), (64,32),
(128,16), (128,64)}. Lemma 7's edge bound 4d + 8L(J+1) < 1100(d+1) and degree bound 2·max(L₁,L₂)(J+1) <
136(d+1)^{3/4} follow from Lemma 5 as the paper says; I also counted the network exactly as constructed
(interface 4d bits; words: 4L forward, L products, L inverse stage 1, 2n−1 coefficients, 2n−1 carries, each
J+1 bits):

| d | J | k | n | L (L₁×L₂) | transform-vertex degree | edges as built | Lemma 7 bound |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 131 | 128 | 32 | 5 | 16 (4×4) | **1,032** | 15,230 | 17,036 |
| 132 | 128 | 32 | 5 | 16 (4×4) | 1,032 | 15,234 | 17,040 |
| 256 | 256 | 64 | 4 | 8 (2×4) | 2,056 | 16,958 | 17,472 |
| 521 | 256 | 64 | 9 | 32 (4×8) | 4,112 | 60,166 | 67,876 |

Modular multiplication (Lemma 8) = two integer multiplications (xy and θq) plus adders and canonicity checks.
Point addition (Corollary 2), counting only the multiplications the paper's own constraint list needs: validity
of P₁, P₂ and P₃ (x·x, u₁·x, c_a·x, y·y: 4 each), two zero tests (aζ, ae, ζe: 3 each), slopes
(λ(x₂−x₁), λ·2y₁, x₁·x₁, λ², λ(x₁−x₃): 5) → ≥ 23 modular = **≥ 46 integer multiplications per addition**;
one more modular multiplication (by α_i modulo the (b+1)-bit hash prime s) per slot for membership. Hence
the manuscript's own gadgets give, per slot,

  s/(m·b) ≥ 46 · 15,230 / 131 ≈ **5.3·10³ edges per slot-bit at b = 131**, ≈ 3.0·10³ at b = 256,

before adders, comparisons, selectors and the equality trees that share words (Corollary 1). So
s = a·m·b with a ≈ 3–6·10³ from the paper's own construction; s is **linear in b and in m** (this is the whole
point of the one-split Fourier gadget: Θ(√d) vertices of degree Θ(d^{3/4}), total Θ(d) edges).

### 2.2 Cross-check against the implementation's recorded graphs [checked numerically against repo receipts, pcc:unverified]

* ECCp-131, m = 2, seed 1 (Python companion receipt `prime-development/challenge-tensor.json`): 1,564,506 edges,
  654,523 variables, 71,723 constraints, maximum degree 1,032; multiplication blocks "34 × 131-bit and
  4 × 132-bit integer products, ring 129 bits, transform length 16". My Theorem 3 parameters at d = 131 give
  exactly ring J+1 = 129, L = 16, and degree 2·4·129 = **1,032**: the implementation's degree is the paper's
  transform-vertex degree, reproduced to the unit. 1,564,506/(2·131) = 5,971 edges per slot-bit, within 12% of
  the 5.3·10³ floor above; 38 products × 15,230 = 579k edges are the multiplications, the rest adders/trees.
  Lemma 7's upper bound for 38 products (5.56M) is above the recorded total: consistent.
  The 4 × 132-bit products are the two hash slots (s has 132 bits), 2 integer products each: consistent with
  Lemma 8 applied to Lemma 11.
* ECC2K-130 (2,694,680 edges, degree 1,032) is a binary-curve graph from the same Python companion. The
  manuscript treats prime fields only; the characteristic-2 adaptation is the lab's own (LEADS L01). Degree
  1,032 is again the d = 131 Fourier parameter. I did not reproduce its edge count (no manuscript construction
  for it exists).
* Both receipts record `root_u = 0`, λ = 21, leaf threshold H = ⌈s/⌊√21⌋⌉ = 391,127, "paper_recursion_eligible:
  false", status `budget_exhausted` at a 2·10⁶-operation budget. That is exactly what §3 predicts.
* The crate's own m = 2 graphs at b = 5..16 (L09 `table2.json`): 57,952 … 128,448 edges, degree 132 or 260, i.e.
  2·4·33 (J = 32) and 2·4·65 (J = 64): again the Theorem 3 degrees. The ~58k-edge floor at b = 5 is the J ≥ 16
  minimum (Lemma 5 part 1 forces J ≥ 16 and L ≥ 2n even for tiny d), so small-b slopes are not asymptotic
  readings, as L09 already says.

Conclusion of §2: s(m, b) = a·m·b with 3·10³ ≲ a ≲ 6·10³ in the paper's own construction, degree
≈ 2·max(L₁,L₂)(J+1) = Θ(b^{3/4}) with the exact values above. The counts in the repo notes are consistent with
the manuscript; no discrepancy found.

## 3. Can Theorem D.1's hypotheses hold for this encoding at any cryptographic size?

### 3.1 The admission arithmetic, re-derived [proved here] [checked numerically]

All from quantitative-contraction.tex l. 80–105 and l. 360–364, with C = 1 (s = the edge count):

1. The recursion exists only if u₀ = ⌊λ/2^17⌋ ≥ 1, i.e. **λ ≥ 2^17**, i.e. s ≥ 2^{2^17−1} ≈ 2^{131071}. Below
   that the algorithm is "direct summation" over all 2^s edge assignments (l. 78, and Bochum footnote 4). This is
   the manuscript's "2^{2^17}" figure. It is a necessary condition, not the proof's regime.
2. The standing bound λ^{4/5} ≤ u (eq. tensor-parameter-bounds) at depth 0: λ/2^17 ≥ λ^{4/5} ⇔ λ^{1/5} ≥ 2^17
   ⇔ **λ ≥ 2^85**. Boundary verified exactly: at λ = 2^85, u₀ = 2^68 = λ^{4/5}. This reproduces the
   clustering note's item 1 and LEADS §1 fact 4 independently.
3. The same bound at every *active* call. Children have ≤ M/8 edges and a call is active while M > H = s/⌊√λ⌋,
   so the recursion necessarily reaches depth d_max with 8^{d_max} ≈ √λ, d_max ≈ log₂λ/6 (the preprint's own
   "d ≤ log₂λ/6 + O_C(1)", l. 96). There u_d ≈ ελ^{5/6}, and λ^{4/5} ≤ u_d ⇔ λ^{1/30} ≥ 2^17 ⇔
   **λ ≥ 2^510** (with general C: λ ≥ 2^510 C^{10}). Boundary verified exactly: at λ = 2^510, d_max = 85,
   u_85 = 2^408 = λ^{4/5}. So the parameter regime the proof *states* is entered no earlier than
   s ≥ 2^{2^510}, not 2^{2^85}; the clustering note's figure is the depth-0 condition only.
4. Where the bound is *used*: only once, l. 363 ("since M > H and u ≥ λ^{4/5}, this extra factor 2^u dominates
   every fixed power of λ"). That step needs only u_d = ω(log λ) (2^{u_d} above the hidden polynomial in λ), so
   the proof could be rewritten with a weaker admission condition; even then u_d ≥ 1 at depth d_max needs
   λ ≥ 2^{17}·λ^{1/6}·O(1), i.e. λ ≳ 2^{21}, s ≳ 2^{2^21}. **Under no reading does the recursion engage below
   s ≈ 2^{131071} edges.** [proved here from the stated parameters; whether a different parameter choice could
   lower ε is outside the audit: ε = 2^{-17} is what the preprint fixes, and the Bochum theorem cites that proof.]

### 3.2 Consequence for the Bochum network [proved here]

For the relation network s = a·m·b with a ≤ 10⁴ and any b ≤ 2^{20}, m ≤ 2^{10}: s ≤ 2^{44}, λ ≤ 44 ≪ 2^17.
Hence at every size anyone can write down, Theorem D.1's algorithm applied to this encoding is direct
summation, cost 2^{Cs} ≥ 2^{s}: at ECCp-131, 2^{1,564,506} operations against Pollard rho ≈ 2^{65.5} and
naive enumeration of a two-slot decomposition 2^{65.5}·poly. The implementation's `root_u = 0` /
`budget_exhausted` receipts and `admission.rs` ("on a 64-bit build λ ≤ 64, so root_u = λ/131072 is zero for
every publicly representable graph") agree.

The L09 "margin" −16.1 … −16.3 bits is 0.2·log₂λ − 17 for λ ∈ [10.2, 22]; I reproduced both endpoints
(−16.33, −16.11). The clustering note's steps 2–6 (what remains if admission were granted) I did not
re-derive; step 1 alone suffices for §3 and I confirm it, with the correction that full-depth admission is
2^510, not 2^85.

### 3.3 The LEAD-ICB obstruction, applied to this encoding [proved here]

`LEAD-ICB-decomposition-encoding-and-sat` / KN-LIT-caa51d: a #SAT count costing 2^{N/√log N} loses to
enumeration of the m·l unknowns at every l once the CNF has N = ω(l√log l) variables, as a Tseitin schoolbook
encoding does (N = Θ(l²)). The Bochum encoding has s = a·m·b = a·m²·l edges, **linear in l**. For fixed m the
ratio (solver exponent)/(enumeration exponent) = a·c·m²/((m−1)√log₂ s) → 0 as l → ∞, so the literal
ω(l√log l) obstruction does **not** apply: asymptotically, for fixed m, the counting bound beats naive
enumeration per attempt. That is precisely what the Fourier gadget buys and the atlas question is answered
in the negative. What closes the lead instead is §3.1–3.2 (non-admission at every writable size) and the
constants in §4.

## 4. Granting the counting bound exactly as stated: does the composed algorithm beat enumeration or rho?

Take Theorem D.1 as the formula c·s/√log₂ s valid at every size (it is not: §3), s = a·m·b, c = 1, and
charge the relation phase m²b·r^{1/m} calls and the linear algebra m³r^{3/m}b². [checked numerically]

| b | a (edges per slot-bit) | best m | solver per attempt (bits) | calls (bits) | linear algebra (bits) | total (bits) | rho (bits) | enumeration per attempt (bits) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 128 | 1 (impossible floor) | 3 | 131 | 53 | 147 | 184 | 64 | 85 |
| 128 | 2,600 (paper's gadgets) | 2 | 1.5·10⁵ | 73 | 209 | 1.5·10⁵ | 64 | 64 |
| 128 | 6,000 (implementation) | 2 | 3.4·10⁵ | 73 | 209 | 3.4·10⁵ | 64 | 64 |
| 192 | 1 | 3 | 190 | 75 | 212 | 265 | 96 | 128 |
| 192 | 2,600 | 2 | 2.2·10⁵ | 106 | 306 | 2.2·10⁵ | 96 | 96 |
| 256 | 1 | 3 | 248 | 97 | 277 | 345 | 128 | 171 |
| 256 | 2,600 | 2 | 3.0·10⁵ | 138 | 403 | 3.0·10⁵ | 128 | 128 |
| 256 | 6,000 | 2 | 6.6·10⁵ | 138 | 403 | 6.6·10⁵ | 128 | 128 |

Even with the impossible a = 1 (one edge per interface bit, no gadgets) and c = 1, the composed algorithm is
above rho by 120–217 bits at b = 128–256 and above per-attempt enumeration; the linear algebra alone
(3b/m) exceeds rho for every m ≤ 6. With the paper's own gadget constant the exponent is ~10⁵ bits.

Crossover conditions [proved here, reproducing the Lean `blackbox_balance`/`no_crossover` arithmetic of
LEADS §2 independently]: minimising b/m + a·c·m·b/√log₂ b over m gives E_min = 2b√(ac)/(log₂ b)^{1/4};
E_min < b/2 ⇔ **log₂ b > 256 (ac)²**. With ac ≥ 1 (and a ≥ 2 is forced by the interface alone) that is
b > 2^{256} bits of group order; with a = 2,600 it is log₂ b > 1.7·10⁹. The solver beats naive per-attempt
enumeration at fixed m only when √log₂ s > a·c·m²/(m−1): with a = c = 1, m = 2 that is s > 2^{16} (so the
asymptotic claim is coherent), with a = 2,600 it is log₂ s > 1.1·10⁸. **Answer: neither.** At 128–256 bits
the composed algorithm beats neither enumeration nor rho under any constant ≥ 1, and under the literal D.4
parameters it is brute force 2^s.

## 5. Correctness of the reduction

Re-derived from the manuscript text [proved here unless marked]:

* **Lemma 9** (hash properties): for distinct z ≠ z′ and distinct t ≠ t′ in 𝔽_s exactly one (α, β) ∈ 𝔽_s^× × 𝔽_s
  maps z ↦ t, z′ ↦ t′ (α = (t−t′)/(z−z′) ≠ 0), so (H(z), H(z′)) is uniform on the s(s−1) ordered distinct
  pairs and Pr[A, B ∈ F_i] = τ(τ−1)/(s(s−1)) ≤ ρ². Injectivity of A ↦ H_i(enc A) needs enc(A) < 2p ≤ s and
  α ≠ 0: holds, so |F_i| ≤ τ. Correct.
* **enc(A) = 2x + (y mod 2)** is injective on affine points of odd-order curves: y and p − y have opposite
  parity for y ≠ 0; y = 0 is a 2-torsion point, absent from a group of odd prime order r (and, in the cofactor
  case, from G). Correct. The crate's `hash_member` builds exactly this word (LSB of y, then the x bits) and
  reduces it modulo the certified prime s of width s.bits() = 132 at ECCp-131. Consistent.
* **Lemma 10** (coverage): µ ≥ (1−1/n)L^m/n; E[X²] ≤ µ(1 + Σ_{j<m} C(m,j) n^{m−j−1} ρ^{m−j}) =
  µ(1 + ((L+1)^m − 1)/n) ≤ µ(1 + (L+1)^m/n); Cauchy–Schwarz gives Pr[X>0] ≥ (1−1/n)/(n/L^m + (1+1/L)^m) ≥
  (98/99)/(1 + e^{1/10}) = 0.4702 > 2/5 using L^m ≥ r > n, L ≥ 10m, n ≥ 99. The pair bound uses Lemma 9(2)
  position-wise for the m−j disagreeing positions and independence across bases. Correct.
* **Corollary 2** (point addition): canonical representation (ι, x, y) with ι·ι ≡ ι, ιx ≡ ιy ≡ 0, curve
  equation × (1−ι); zero tests aζ ≡ 0, ae ≡ 1−ζ, ζe ≡ 0 determine (ζ, e); the five selectors are mutually
  exclusive and exhaustive (P₁ = O; P₂ = O ≠ P₁; P₂ = −P₁; x₁ ≠ x₂; x₁ = x₂ with y₂ ≠ −y₁, which on the curve
  forces y₂ = y₁ ≠ 0 so 2y₁ is invertible). Slope/sum formulas (10)–(12) are the standard affine law; λ = 0
  forced in inactive cases; all witnesses determined → parsimonious. Correct. The crate's `point_add` mirrors
  this (σ₂…σ₅ from the same zero tests, all branches evaluated unconditionally, inactive slope forced to 0,
  validity of both inputs and the output). Consistent.
* **Lemma 11/Fig. 2**: each A_i nonidentity (hash gadget asserts ι_i = 0), intermediate sums may be O, final
  sum equals the target constant. A satisfying assignment is therefore a genuine identity A₁ + … + A_m = R in
  E(𝔽_p), hence in G when the cofactor is 1. Correct.
* **Cofactor c**: A_i = [c]S_i computed inside the network by O(log c) additions, so every A_i lies in the
  r-Sylow subgroup G = [c]E(𝔽_p) (r ∤ c since r > c prime) whatever the group structure; each A has exactly
  #E[c](𝔽_p) = c preimages, so the count is non-parsimonious by the factor c^m, which Lemma 1's prefix search
  tolerates (it only needs C(u) > 0 tests). The enumeration side keeps only hash preimages with [r]A = O, so
  the matrix columns are exactly the points the network can produce. Correct. The crate's `clear_cofactor`
  implements the double-and-add chain and `decomposition_network_impl` switches to it when the cofactor ≠ 1.
* **Lemma 12** (extraction): conditioning on the target sequence leaves each v_j uniform (u_j is determined by
  R_j and v_j); the decomposition and the row dependency are deterministic in (seed, targets), so the
  coefficients c_j are independent of the v_j; a nontrivial 𝔽_r-combination of uniform v_j is uniform;
  Pr[V = 0] = 1/r. The relation algebra: Σ_j c_j M_{j,(i,A)} ≡ 0 (mod r) for every column kills every
  factor-base point because [kr]A = O, giving [U]P + [V]Q = O, U + Vξ ≡ 0. Sign convention ξ = −U/V correct.
  Any returned ξ satisfies Q = [ξ]P, so "never returns a wrong value" holds (the output can be verified by one
  scalar multiplication anyway). Correct.
* **Lemma 13** (cost): N ≤ mτ = O(m r^{1/m} + m²); goodness Pr[f ≥ 1/5] ≥ 1/4 from E[f] ≥ 2/5; waiting time
  T for N+1 successes has E[T] ≤ 5(N+1), Markov gives Pr[T > 20(N+1)] ≤ 1/4; trial success ≥ (1/4)(3/4)(1−1/r)
  > 1/6; Wald/geometric restarts give E[cost] = E[C]/p_succ. Gaussian elimination O(N³) field operations =
  O(m³ r^{3/m}) · O(b²) bit operations; 2^{3b/m} is within 2^{O(b/m)}. Correct.
* **Remark 1** (fitting Theorem D.1): subdividing every edge doubles s, keeps degree ≥ 2, keeps the count;
  ℓ = 1, P₀ = s+1 ≤ Cs with C ≥ 2 and 2^{s+1} > count. Degree O(b^{3/4}) ≤ ⌈(s')^{9/10}⌉ for the actual sizes
  (1,032 ≤ 1,564,506^{0.9} ≈ 3.7·10⁵). Correct.
* **Lemma 1** (prefix search): m+1 contractions suffice via C(u1) = C(u) − C(u0). Correct.
* **Implementation checks [cited, pcc:unverified, not rerun]**: cryptanalysis `docs/hashed-ecdl.md` and
  `research/hashed_ecdl_speed_20261029/README.md` report 34 passing native tests and identical complete seeded
  certificates under prefix counts and optimised search, with scalar replay and corrupted-certificate controls;
  the crate today has 37 `#[test]` functions (`point_network_handles_every_pair_and_rejects_counterfeit_outputs`,
  `cofactor_tensor_projection_finds_only_subgroup_points`, `hash_membership_and_cofactor_clear_are_coordinate_predicates`,
  `tensor_relation_pipeline_recovers_and_replays_without_scalar_leakage`, …). I did not build or run them (desk
  audit; shared machine). `reduction.rs::verify` re-derives the relation algebra from the certificate; its
  existence is consistent with §5, its correctness is not re-audited here.

**No correctness error found in the reduction.** Two presentation slips in the manuscript:

(a) Footnote 4 writes "u₀ = ⌊b/2^17⌋"; in the preprint u₀ = ⌊ελ⌋ with λ = ⌈log₂ s⌉ and s the **network's**
edge count. With the manuscript's b = ⌈log p⌉ the formula is wrong (and would make u₀ > 0 at b ≥ 2^17 bits,
when actually λ ≈ log₂ b + 13 there). The sentence around it ("networks of size at least 2^{2^17}") is right.

(b) The limitations paragraph quotes 2^{2^17} as where the proof "yields a saving". That is the u₀ ≥ 1
condition. The parameter bounds the proof states (λ^{4/5} ≤ u at every active call) are met only from
s ≥ 2^{2^85} (depth 0) and s ≥ 2^{2^510} (full depth); see §3.1. The manuscript understates its own
threshold; it does not affect the asymptotic statement.

## 6. Verdict for the Coordinator

**(ii) with (i).** The manuscript's reduction is correct as far as this audit reaches (every lemma in §5
re-derived; Theorem 3's arithmetic checked exactly; no quotable error, only the two slips above). The
subexponential statement therefore stays exactly as conditional as the authors say: **the one remaining
dependency is Theorem D.1 of openai/math @ 7ef5c0cc, Appendix D.4, with its unspecified constants**, which
nobody in the program has verified and which the lab's two readings "found no gap" in.

Independently of D.1's truth, the lead is **closed as a research direction under the following cost model**:
exponent of the composed algorithm (relation phase + linear algebra, solver charged at the D.1 bound) against
rho's b/2 and against per-attempt enumeration (m−1)b/m, with the D.4 parameters as published (ε = 2^{-17},
r = 64, H = s/⌊√λ⌋). Under that model:

* at every writable size the solver is direct summation, 2^s with s ≈ 6·10³·m·b (2^{1,564,506} at ECCp-131);
* taking the asymptotic formula at face value with constant 1, the algorithm is below rho only for
  log₂ b > 256 (ac)², i.e. group orders of more than 2^{256} bits even at the impossible a = c = 1, and
  log₂ b > 10⁹ with the manuscript's own gadget constant;
* the LEAD-ICB ω(l√log l) obstruction does **not** apply (the encoding is linear in l); the closure rests on
  admission and constants, not on Tseitin blow-up. The atlas text that asks the audit to "check whether the
  obstruction applies" should be corrected to say so.

What would reopen it (`revisit_when`): a contraction theorem with explicit constants and admission at
λ = O(1) (not 2^17), or a relation network with o(b) edges per slot (impossible for a sum of m points, each
needing its 2b coordinate bits on the interface), or Remark 2's lever (exponent s/(log s)^α with α ≫ 1/2).
None is in sight; none is a measurement this lab can make.

## 7. Repo notes I could not reproduce, or that need correction

1. Clustering note item 1 / LEADS §1 fact 4 / SYNTHESIS: "D.1 needs λ ≥ 2^85, i.e. s ≥ 2^{2^85} at depth 0" —
   reproduced, but it is the depth-0 condition only; the stated bounds at the deepest active call need
   λ ≥ 2^510. Both galactic; the note should say "at least 2^{2^85}".
2. Atlas lead `next_action` (a): "check whether the obstruction in LEAD-ICB applies: #SAT exponent outgrows
   enumeration once the CNF has ω(l√log l) variables" — it does not (s linear in l). KN-LIT-caa51d's
   "Against enumeration" paragraph is correct for Tseitin encodings and should name the Bochum gadget as the
   exception that is instead killed by admission and constants.
3. KN-LIT-eb8749 says Theorem 4 has "solver calls on graphs with O(mb) edges" — correct; it also records that
   Lemmas 10–12 were "NOT re-read" there and relies on the 2026-10-31 reading. This audit re-read them: no error.
4. LEADS §2 erratum 3 (edges per slot-bit 1.17·10⁴ / 11.9k per bit at b = 131): the receipt gives 1,564,506
   edges for m = 2, which is 11.9k per *bit* and 5.97k per *slot-bit*. Both numbers are the same fact; the
   manuscript's own gadget count gives ≥ 5.3k per slot-bit, so the implementation is within 12% of the paper's
   floor and not padded.
5. "34 tests" (atlas `best_measured`): the crate now has 37 `#[test]`s in cryptanalysis (and more in crypto,
   which has grown 8 source files beyond the cryptanalysis mirror; the "identical sources" statement in the
   atlas mechanism field is out of date). Not rerun here.
6. The ECC2K-130 graph has no manuscript construction behind it (prime fields only); the atlas lists it under a
   prime-field lead. Its 1,032 degree is the same d = 131 Fourier parameter, so the Python companion reused the
   integer gadget for a binary curve, which is the thing LEADS L01 says "does not carry over to GF(2)[x]
   without a characteristic-2 variant". Whether that companion graph encodes a correct binary relation was not
   audited here and is not claimed anywhere I read.
7. The repo's two earlier D.1/D.4 readings (`research/prime_one_shot_20261029/continuation/tensor-review/`)
   state "u_j is eventually at least λ^{4/5}" without the threshold; consistent with this audit, nothing to
   correct, but no reading has checked the D.4 proof's correctness and neither has this one (out of scope:
   the task was the Bochum reduction and the admission arithmetic).
