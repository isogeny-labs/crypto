# L11: an odd-order Kummer factor base `{P : x(P) ∈ lo + g·μ_d}`, d | (p−1)/4 (design only)

**Class:** design document for a new algebraic-track membership (a tower of
odd-degree rules). No run, no code, no measurement was made for this lead. Every
number below is either (a) arithmetic on the family primes, (b) read from existing
census and engine data (cited per cell), or (c) a modelled quantity, labelled as
such. Rho remains ahead (T12: the best PKM arm is 6.06·10⁴× behind rho-neg at
w = 0.1 on k = 20); nothing here changes that order or is a leaderboard claim.

**Status:** design, not yet frozen. Freezing = the executing session writes
`scripts/odd_kummer_census.py` and `scripts/analyse.py` exactly as section 6
specifies, records their sha256 in this file, and only then runs. No `data/`
exists. Wave-4 brief, lead directory
`research/curve25519_leads_wave4_20261104/l11_odd_kummer_design`, branch
`claude/curve25519-index-calculus-i674j4`; no git operations by this lead.

Read first: `research/curve25519_leads_wave3_20261103/t10_seed_and_guess/README.md`
(cost model, winner cell), `research/curve25519_t10_winner_ecbench_20261104/README.md`
(end-to-end confirmation), `research/curve25519_torsion_leads_20261103/LEADS.md`
(T2: the 2-descent base, coverage ×2.0 but lost in F4 on degree).

## 0. Summary

`prime-kummer` (`src/cryptanalysis/ic_framework/prime_tower.rs`) uses
`V = g·μ_{2^t}`, which needs `2^t | p − 1`; every family miniature has
`p ≡ 5 (mod 8)`, so `t ≤ 2` and the form is useless here. But
`p − 1 = 4 · (odd)` and the odd part is smooth-ish on three of the five curves
(k = 20: `262143 = 3³·7·19·73`). A multiplicative subgroup `μ_d` of odd order
`d | (p−1)/4` gives a base of `d` abscissae (≈ d/2 columns) whose membership is a
tower of **odd-degree** rules `y_j^{e_j} = y_{j+1}` (e_j ∈ {3, 7, 19, …}), which
the engine's `SquareRule` tower (`src/cryptanalysis/f4_fp_tower.rs`, bit-mask
monomials) cannot express.

The reason to want it is not the tower itself but a structural fact (section 2.2):
for odd d every element of `μ_d` is a square, so in the **Montgomery frame**
(`u ∈ g·μ_d`, i.e. `lo = A/3` in the engine's Weierstrass abscissa) every base
point has the same 2-descent class (`χ(u(P)) = χ(g)`), hence the same parity of
its cofactor label in Z/8. Every signed pair then passes the mod-2 cancellation
test for free: expected m = 2 coverage ≈ 2× a random base of the same size.
T2 measured exactly this ×2 (mean q ≈ 2.0–2.5 at k = 20, section 2.3) by encoding
`u = w²` *algebraically* and lost, because the squaring raised the F4 degree.
`μ_d` carries the same constraint **without any extra equation**: it is a
property of the membership set, not of the system. Whether the odd-degree tower's
own F4 penalty eats that factor is the open question, and it cannot be answered
without the `PowerRule` engine change (section 4). Section 6 therefore specifies a
pure-Python exact census (the frame_census log table) as the only work before
any engine code, and section 8 the numeric go/no-go that the census decides.

## 1. Question

Q-A (coverage, census-decidable now). On the family miniatures, do odd-Kummer
bases `{P : x_W(P) ∈ lo + g·μ_d}` in the Montgomery frame (`lo = A/3`) reach an
exact m = 2 subgroup coverage per column that the selected 2-power towers do not:
mean coverage ≈ 2× random, and a selected coset (over (p−1)/d cosets) with
`covered/cols` above the T10 winner's 9.55 at a comparable column count?

Q-B (engine, after a go). With a `PowerRule` tower, is the F4 work per target of
an odd tower with small prime factors (d = 21, 27, 63) within a factor κ ≤ 1.5 of
the modelled 2-power cost at equal quotient size, so that the modelled solve cost
`E = cols · r · mpt / covered` beats the T10 winner (E₀ = 7.26·10⁸ at k = 20)?

This protocol preregisters Q-A fully and the go/no-go for Q-B. Q-B's own
predictions are written only after a go, in a separate protocol.

## 2. Facts computed now from existing data

### 2.1 `p − 1` across the family (arithmetic; `r`, `A`, `p` from the family rule)

| k | A | p | p mod 8 | p − 1 | odd part (p−1)/4 | odd divisors d ≤ 600 |
|--:|--:|--:|--:|:--|:--|:--|
| 16 | 1058 | 65,437 | 5 | 2²·3·7·19·41 | 16,359 = 3·7·19·41 | 3, 7, 19, 21, 41, 57, 123, 133, 287, 399 |
| 18 | 78 | 262,133 | 5 | 2²·13·71² | 65,533 = 13·71² | 13, 71 |
| 20 | 926 | 1,048,573 | 5 | 2²·3³·7·19·73 | 262,143 = 3³·7·19·73 | 3, 7, 9, 19, 21, 27, 57, 63, 73, 133, 171, 189, 219, 399, 511, 513 |
| 22 | 43186 | 4,194,301 | 5 | 2²·3·5²·11·31·41 | 1,048,575 = 3·5²·11·31·41 | 3, 5, 11, 15, 25, 31, 33, 41, 55, 75, 93, 123, 155, 165, 205, 275, 341, 451, 465 |
| 24 | 318 | 16,777,213 | 5 | 2²·3·23·89·683 | 4,194,303 = 3·23·89·683 | 3, 23, 69, 89, 267 |

Checks: `8r − (p+1)` = 234, −382, 1050, 3546, 1578 (all inside the Hasse bound,
consistent with `#E = 8r`). Since `4 | p − 1` on every curve, mixed orders
`d = 4·d_odd` also exist (two `SquareRule` levels on top of the odd tower):
k = 20: 12, 28, 36, 76, 84, 108, 228, 252, 292, 532; k = 16: 12, 28, 76, 84, 164,
228, 492, 532; k = 22: 12, 20, 44, 60, 100, 124, 132, 164, 220, 300, 372, 492;
k = 18: 52, 284; k = 24: 12, 92, 276, 356. They are listed for completeness and
are **not** part of the first census (section 6): with d_odd odd the base is no
longer inside one square class (μ_4 ∋ −1, i, −i; i is a non-square when
p ≡ 5 mod 8), so the ×2 mechanism of 2.2 is lost.

**Single-prime d** (19, 73, 13, 71, 23, 89, 41, 31, 11, …) give a one-level
tower `y^d = 1` whose normal form has monomials `y^0 … y^{d−1}`: a system of
degree d−1 per summand and no guessing (nothing above the level to fix). They
are excluded from engine consideration; the census still records d = 19 and 73
at k = 20 as coverage controls (same mechanism, no usable tower).

### 2.2 The 2-descent lemma (derivation, to be checked exactly by P1)

The family curves are Montgomery `v² = u³ + A u² + u` (B = 1, the instrument's
model: `frame_census.py` docstring), `E(F_p)` cyclic of order 8r (the instrument
aborts otherwise), and the engine's abscissa is `x_W = u + A/3`. For the
2-isogeny with kernel `⟨T⟩`, `T = (0,0)`, the connecting homomorphism
`δ : E(F_p) → F_p^×/F_p^{×2}` is `P ↦ u(P)` for `u(P) ≠ 0` and `T ↦ b = 1`
(the curve is `u(u² + Au + 1)`, so the constant coefficient is 1). Its kernel
`φ̂(E'(F_p))` has index `|ker φ̂(F_p)| · #E/#E' = 2` in `E(F_p)`. In
`Z/8 ⊕ Z/r` the index-2 subgroup is unique, `= 2E(F_p)`. Hence

> `χ(u(P)) = +1  ⟺  P ∈ 2E(F_p)  ⟺  log(P) mod 8 is even.`

For odd d, `μ_d ⊂ F_p^{×2}` (every ζ ∈ μ_d is `ζ = (ζ^{(d+1)/2})²`), so for
`u ∈ g·μ_d`, `χ(u) = χ(g)`: **all base points share one label parity.** The
coverage count is `|{R ≠ O ∈ E[r] : R = ±P_i ± P_j}|`, and `±l_i ± l_j ≡ 0 (mod 8)`
requires first `l_i ≡ l_j (mod 2)`, which now always holds. If the labels are
otherwise uniform within their parity class, a signed pair cancels with
probability 1/4 instead of 1/8: `E[covered] ≈ c(c−1)/2` instead of `c(c−1)/4`
(c = columns; i < j; both signs of a sum counted, as the instrument does).

Define the **coverage quality** `q = covered / (c(c−1)/4)`: 1 = random,
2 = the lemma's expectation, > 2 = selection.

In a frame `lo ≠ A/3` (e.g. `x_W ∈ g·μ_d`, `lo = 0`), `u = g·ζ − A/3` has no
fixed square class: q ≈ 1 expected. The census compares both.

### 2.3 Reference cells (existing data, k = 20)

q as defined above, cols ≥ 8 rows only.

| base | source | cols | covered | covered/cols | q |
|:--|:--|--:|--:|--:|--:|
| T10 winner: isogeny t = 5, seed 1307 | `t10_seed_and_guess/README.md` | 22 | 210 | 9.55 | 1.82 |
| T9 best: isogeny t = 6, seed 1308 | same | 36 | 498 | 13.83 | 1.58 |
| isogeny t = 5, seed 0 (incumbent's base) | same | 18 | 72 | 4.0 | 0.94 |
| isogeny census t = 5, 2000 seeds | `curve25519_isogeny_seed_20261103/data/k20.jsonl` | mean 16.0 | mean 70.1 | — | mean 1.138, sd 0.215, max 2.756 |
| isogeny census t = 6 | same | mean 32.1 | mean 269.6 | — | mean 1.063, sd 0.093, max 1.630 |
| translation frames t = 5, K = 2000 | `curve25519_torsion_leads_20261103/data/k20.json` (`N[0]`) | — | mean 70.9, max 184 | — | mean 1.148, sd 0.212, max 2.571 |
| translation frames t = 6 | same | — | mean 270.5, max 582 | — | mean 1.068, sd 0.096, max 1.644 |
| **T2 base u = w², t = 4 / 5 / 6 / 7** | same (`t2_N0`, `t2_cols`) | — | — | — | **mean 2.487 / 2.271 / 2.131 / 2.048; max 7.14 / 5.29 / 3.48 / 2.43** |

The T2 row is the lemma measured by a different base: forcing `u` square doubles
q at every t (and the decline towards 2 with t is the sum-collision correction).
The 2-power towers (isogeny, translation) sit at q ≈ 1.06–1.15 mean; their
selected cells reach q 1.6–1.8 at the column counts that matter (22–36).

Reference F4 cells (T10, `data/cells`, seed-independent): per-target mul-adds at
G = t−1 are 52,816 (t = 5, 16 branches, quotient 2·32 = 64 monomials per branch),
146,464 (t = 6, 32 branches, 128), 423,619.64 (t = 7, 64 branches, 256). Per
branch: 3,301 / 4,577 / 6,619 = `3301 · (Q/64)^α`, α = 0.47–0.53. **Model used
below: per-branch cost `3301 · (Q/64)^{0.5}`, mpt = branches × per-branch × κ,
κ = the unknown odd-degree penalty** (κ = 1 would mean an odd level costs the
same as 2-power levels of equal quotient size).

## 3. The membership tower

### 3.1 Definition

Fix `d = e_0 · e_1 ⋯ e_{L−1}` with the prime factors listed **in ascending order
from the bottom** (the smallest at the level next to x), a coset representative
`g ∈ F_p^×` (the coset `g·μ_d` depends only on `g` modulo `μ_d`, i.e. on
`i ∈ Z/((p−1)/d)` with `g = w^i`, `w` a primitive root), and a frame offset
`lo` (default `lo = A/3`, the Montgomery frame; `lo = 0` is the Weierstrass
frame; any `lo` is a translation frame as in `bits@LO`). For summand i of the
Semaev system (m = 2: i ∈ {1, 2}; the target `x_R` is a constant per system):

```
x_i          = lo + g · y_{i,0}
y_{i,0}^{e_0} = y_{i,1}
y_{i,1}^{e_1} = y_{i,2}
  …
y_{i,L−1}^{e_{L−1}} = 1                      (top level: a constant rule)
```

The zero set of the composed map is exactly `lo + g·μ_d` (d distinct abscissae;
`μ_d` has d elements because d | p−1), the quotient ring per summand is
`F_p[y_0,…,y_{L−1}]/(rules)`, of dimension d with monomial basis
`{∏ y_j^{a_j} : 0 ≤ a_j < e_j}`, maximal total degree `Σ_j (e_j − 1)`.
`d = 3^a · 7^b · 19^c · 73^e` (k = 20) has `L = a+b+c+e` levels and maximal
degree `2a + 6b + 18c + 72e`.

The Semaev polynomial `S_3(x_1, x_2, x_R)` is quadratic in each `x_i`;
`x_i² = (lo + g y_{i,0})²` stays in normal form when `e_0 ≥ 3` (unlike the bit
tower, where `x² ` already reduces to the next level). So the input system has
degree ≤ 2 per summand and the whole degree is produced inside F4.

**Guessing (hybrid, as `guess=G` for bits/isogeny).** Fixing the top G levels of
summand 1 fixes `y_{1,L−G}` to one of `d / d_G` values (`d_G = e_0 ⋯ e_{L−G−1}`);
summand 1 then ranges over a coset of `μ_{d_G}`. Branches `B_G = d/d_G`, quotient
per branch `Q_G = d · d_G`. T10's `argmin G = t − 1` (one free level left)
translates to **`d_G = e_0`, `B = d/e_0`, `Q = e_0 · d`**: this is why the
smallest prime belongs at the bottom (the residual summand is as small as it can
be) and why d with `e_0 = 3` are the candidates.

**Branch cost per target** (modelled): `mpt_model(d) = (d/e_0) · 3301 · (e_0 d/64)^{0.5} · κ`.

### 3.2 Design table, k = 20 (arithmetic and the model of 2.3; κ = 1 column)

"needed" = the `covered/cols` a coset must reach for the modelled solve cost to
equal the T10 winner's (`Φ_w = covered/(cols·mpt) = 210/(22·52,816) = 1.807·10⁻⁴`),
at κ = 1 / 1.5 / 2.

| d | factors (bottom→top) | L | max deg per summand | cols ≈ d/2 | cosets (p−1)/d | branches d/e₀ | Q = e₀·d | mpt_model (κ=1) | covered/cols needed (κ = 1 / 1.5 / 2) |
|--:|:--|--:|--:|--:|--:|--:|--:|--:|:--|
| 9 | 3·3 | 2 | 4 | 4.5 | 116,508 | 3 | 27 | 6,432 | 1.2 / 1.7 / 2.3 |
| 21 | 3·7 | 2 | 8 | 10.5 | 49,932 | 7 | 63 | 22,926 | 4.1 / 6.2 / 8.3 |
| **27** | **3·3·3** | 3 | 6 | 13.5 | 38,836 | 9 | 81 | 33,423 | **6.0 / 9.1 / 12.1** |
| 57 | 3·19 | 2 | 20 | 28.5 | 18,396 | 19 | 171 | 102,520 | 18.5 / 27.8 / 37.1 |
| 63 | 3·3·7 | 3 | 10 | 31.5 | 16,644 | 21 | 189 | 119,126 | 21.5 / 32.3 / 43.1 |
| 133 | 7·19 | 2 | 24 | 66.5 | 7,884 | 19 | 931 | 239,213 | 43.2 / 64.8 / 86.5 |
| 171 | 3·3·19 | 3 | 22 | 85.5 | 6,132 | 57 | 513 | 532,708 | 96.3 / 144.4 / 192.6 |
| 189 | 3·3·3·7 | 4 | 12 | 94.5 | 5,548 | 63 | 567 | 618,996 | 111.9 / 167.8 / 223.7 |
| 219 | 3·73 | 2 | 74 | 109.5 | 4,788 | 73 | 657 | 772,078 | 139.5 / 209.3 / 279.1 |
| 399 | 3·7·19 | 3 | 26 | 199.5 | 2,628 | 133 | 1,197 | 1,898,691 | 343.2 / 514.7 / 686.3 |
| 511 | 7·73 | 2 | 78 | 255.5 | 2,052 | 73 | 3,577 | 1,801,515 | 325.6 / 488.4 / 651.2 |
| 513 | 3·3·3·19 | 4 | 24 | 256.5 | 2,044 | 171 | 1,539 | 2,768,031 | 500.3 / 750.4 / 1000.5 |
| 19, 73 | single prime | 1 | 18, 72 | 9.5, 36.5 | 55,188; 14,364 | — | — | excluded (no guess level, degree d−1) | — |

For comparison, the 2-power cells behind the model: t = 5: 16 cols (mean) / 22
(winner), 16 branches, Q = 64; t = 6: 32 / 36, 32 branches, Q = 128; t = 7:
64 / 83, 64 branches, Q = 256.

Reading: a random base at d = 27 (q = 1) has `covered/cols ≈ (c−1)/4 ≈ 3.1`; the
lemma (q = 2) gives ≈ 6.2, i.e. the κ = 1 break-even **at the mean**; a selected
coset at q ≈ 4 (T2's maxima: 5.3 at 8 cols, t = 5; 3.5 at 28 cols, t = 6) gives
≈ 12.5, the κ = 2 break-even. d = 21 behaves the same one step lower. d ≥ 57 needs q far above
what any census has shown at 30+ columns (q_max ≈ 1.6–3.5), and its 19-level
carries degree 18 by itself. **The lead lives or dies at d ∈ {21, 27} (and
63 as the stretch cell); everything else is control.** Also, 4-level d = 189
with 95 columns sits where t = 7 sits (Q 567 vs 256) and is only interesting if
κ < 1, which nothing suggests.

The κ in the model stands for everything the 2-power fit does not know about an
odd level: a `y³` rewrite produces two surviving monomials where `y²` produces
one, the degree-6 ceiling at d = 27 is above t = 5's 5, and F4's solving degree
(T10: 4 at G = t−1, 5 otherwise) may rise. κ is a measured quantity of Q-B, not
a free parameter of this protocol: the go/no-go of section 8 is evaluated at
κ = 1.5 with the κ = 1 and κ = 2 columns reported beside it.

## 4. The code change: a general `PowerRule` (specification, no code written)

Today (`src/cryptanalysis/f4_fp_tower.rs`): a monomial is a `u128` key whose low
64 bits are the **complement of a bit mask** over ≤ 64 tower variables (exponent
0 or 1 each), bits 64..96 four free-variable exponents, bits 96..104 the total
degree; `SquareRule { next, a, b, c, d }` rewrites `y_j² = a·y_j·y_next + b·y_next
+ c·y_j + d` (`mul_var_into`, `mask_product_into`, `merge_masks`, `from_raw`,
`TowerRing::check`). `pkm_semaev.rs` builds one `SquareRule` per (summand,
level), refuses `guess` for `Membership::Kummer` (line ≈1023), and lists
memberships `Kummer | Isogeny | Bits | SquareBits | EdwardsBits | T0Bits`.
`curve25519_family f4cost` accepts the forms `bits[@lo]`, `square-bits[@lo]`,
`edwards-bits`, `t0-bits[@lo]`, `isogeny[@seed]`; its `support` census has a
`kummer` form restricted to `μ_{2^t}`.

Required change, in four parts (estimate: 400–700 lines plus tests; **not** part
of this lead; done only after a go):

1. **Monomial encoding (`f4_fp_tower.rs`).** Replace the bit mask by a packed
   mixed-radix digit vector: variable j holds `w_j = ⌈log₂ e_j⌉` bits (2 bits for
   e = 3, 3 bits for e = 7, 5 bits for e = 19), stored complemented so that
   integer comparison stays grevlex (degree field first, then last variable
   first). Keep the 64-bit tower field: m = 2 summands × (3 levels × 2 bits) = 12
   bits at d = 27; d = 63 needs 2 × (2+2+3) = 14; d = 189, 2 × 9 = 18; the 64-bit
   budget allows every d in 3.2 with m ≤ 3. `deg = Σ digits` replaces
   `count_ones`. Free variables and the degree field are unchanged.
2. **`PowerRule { next: Option<u8>, e: u8, coef: [[u64; 2]; e−1] }`** meaning
   `y_j^{e} = Σ_{i<e} y_j^i · (coef[i][0] · y_next + coef[i][1])` (top rule:
   `coef[i][0] = 0`). `SquareRule` is the case e = 2 with `coef = [[b, d],
   [a, c]]`, so all existing memberships keep their rules verbatim and all
   existing `f4cost` numbers must reproduce (regression gate: T10 cell
   `mpt(6,4,0) = 710,094.0` and `mpt(5,4,0) = 52,816.0` to 10⁻³, same gate as
   T10's P0). `mul_var_into` increments digit j and applies the rule when it
   reaches e_j; `mask_product_into` adds digit vectors with per-variable
   carries in bottom-to-top order (each carry lowers the monomial in grevlex, so
   the same termination argument as `check`'s "next is a later variable" holds);
   `merge_masks` is unchanged; `from_raw` reduces exponents ≥ e_j.
3. **`Membership::OddKummer { factors: Vec<u8>, g: u64, lo: u64 }`** in
   `pkm_semaev.rs`: rules `y_{i,j}^{e_j} = y_{i,j+1}` (coef all zero except
   `coef[0][0] = 1`), top `y^{e_{L−1}} = 1` (`coef[0][1] = 1`); `x_i = lo + g·y_{i,0}`
   (as `Pol::var(n, i·L).scale(g).add(lo)`); the abscissa list `v` =
   `lo + g·ζ_d^j`; **guessing by levels** as in 3.1 (branches enumerate
   `y_{1,L−G} ∈ (g·ζ)^{…}`-cosets, the isogeny-form branch code generalised from
   `2^G` to `d/d_G`); verification of a hit is unchanged (abscissa in `v`).
   Spec string: `pkm-semaev:m=2,mode=full,guess=G` with the factor base
   `prime-kummer:d=27,g=G,lo=LO` (extend `prime-kummer` in `prime_tower.rs` to
   accept `d` odd, with `ζ_d = h^{(p−1)/d}`; keep `t` for the 2-power case).
   Reject `d ∤ p − 1`, `g = 0`, and any `e_j > 31` (digit width).
4. **`curve25519_family`**: `f4cost <k> kummer@<d>,<g>,<lo> <L> <G> …` (tower
   height is `L`; `G` counts levels from the top) and a `kummer-census <k> <d>
   <lo> <i_lo> <i_hi>` that prints per-coset `columns, covered_points, J` in the
   `iso-census` format, so the Python census of section 6 can be cross-checked
   by the Rust instrument on a sample of rows.

Correctness gate for the change (before any cost is read): the exhaustive
decomposable-set agreement test that already exists for the isogeny form
(`isogeny_guessing_agrees_with_grid`) run on `OddKummer` at k = 16, d = 21, G ∈
{0, 1}, and `hits_verified = hits` on every f4cost cell.

## 5. Expected F4 behaviour (modelled; becomes Q-B's preregistration after a go)

Per summand: d monomials, degree ≤ Σ(e_j − 1): 6 at d = 27 (vs 5 at t = 5 with
32 monomials), 8 at d = 21, 10 at d = 63 (vs 5 at t = 6 / 64 monomials), 12 at
d = 189. With one free level (`d_G = e_0 = 3`): quotient 81 / 63 / 189 / 567
monomials per branch against 64 / 128 / 256 for t = 5 / 6 / 7; branches 9 / 7 /
21 / 63 against 16 / 32 / 64.

Modelled per-target cost (κ = 1, from 3.2): 33.4·10³ (d = 27), 22.9·10³ (21),
119·10³ (63), 619·10³ (189). The future engine protocol will preregister, for the
first cell `k = 20, kummer@27,g*,lo=A/3, L = 3, G = 2, 100 targets, target seed
2026111527, cap 3600, m = 2, full, lifts 0, max_nnz 30,000,000`: `mpt ∈ [33·10³,
100·10³]` (κ ∈ [1, 3]), `solving_degree_max ∈ {4, 5, 6}`, `undecided = 0`,
`hits_verified = hits`, and columns/covered equal to the census row of g*. Those
numbers are written here so that they are on record before any engine exists;
they are **not** decided by this lead.

## 6. The census (the only work before engine code): exact design

### 6.1 Instrument (reused, frozen)

`research/curve25519_frame_search_20261103/scripts/frame_census.py` (#3359):
`build(p, A, n)` → `(logu, r)`, exact sign-folded full logs in cyclic Z/8r
indexed by Montgomery `u` (`A3 = A·3⁻¹ mod p`, `u = x_W − A3`);
`coverage(logs, n, r)` → `(covered, cols, J)` with `covered = |{R ≠ O ∈ E[r] :
R = ±P_i ± P_j}|` (the engine's `covered_points`), `cols` = abscissae carrying
points, `J` = signed pairs whose cosets cancel (log-free selector; L14 recorded
Spearman(J, covered) = 1.000 at t = 4–6, k = 20). The L14 glue
(`l14_digit_set_frames/scripts/digit_census.py`: `logs_of`, `evaluate`) is the
template for the new script.

### 6.2 New glue (to be written and frozen with this protocol; pure Python, `-I`)

`scripts/odd_kummer_census.py` and `scripts/analyse.py` in this lead directory.
Algorithm of the census, fixed here:

1. `logu, r = build(p, A, n)`; `A3 = A·3⁻¹ mod p`; `w` = the smallest primitive
   root mod p (the first h ≥ 2 with `h^{(p−1)/q} ≠ 1` for every prime q | p−1).
2. Frames (Weierstrass offsets `lo`): `u` = A3 (Montgomery frame); `w0` = 0;
   `t1, t2, t3` = three draws of `random.Random(20261104 + 7).randrange(p)`,
   drawn once per curve and shared by all d.
3. For each d in the curve's list (6.3): `ζ = w^{(p−1)/d}`; coset index set
   `I_d` = all `i ∈ [0, (p−1)/d)` if `(p−1)/d ≤ K`, else `random.Random(20261104
   + d).sample(range((p−1)/d), K)`; **K = 2000**. For `i ∈ I_d`: `g = w^i`;
   for each frame: abscissae `x_j = lo + g·ζ^j mod p`, j < d; `logs =
   sorted({logu[(x_j − A3) mod p]} \ {0})`; `(covered, cols, J) = coverage(logs,
   n, r)`; `q = covered/(cols(cols−1)/4)` (NaN if cols < 2); label histograms
   `logs mod 2` (even, odd counts) and `logs mod 8`; `chi_g = χ(g) = (−1)^i`.
   Row: `(d, frame, i, g, cols, covered, J, q, even, odd, hist8, chi_g)`.
4. Costs counted per row (reported, never hidden): Legendre symbols = d;
   `[r]P` labels = cols (1.5·log₂ r group operations each, #3359 convention).
   The log table itself is 4r group additions (one-time per curve, an
   instrument cost, as in #3359 and L14).
5. Output `data/k{k}_d{d}.json` per (curve, d), complete files skipped on
   relaunch (container restarts); `data/k{k}_frames.json` with `w`, `A3`, `t1–t3`.

Commands (the whole lead; `scripts/run.sh` runs exactly these, then the analysis):

```
python3 -I scripts/odd_kummer_census.py --k 20 --p 1048573 --A 926 --n 1049624 \
  --d 9 19 21 27 57 63 73 133 171 189 219 399 511 513 --K 2000 --seed 20261104 --out-dir data/
python3 -I scripts/odd_kummer_census.py --k 16 --p 65437 --A 1058 --n 65672 \
  --d 21 57 133 399 --K 2000 --seed 20261104 --out-dir data/
python3 -I scripts/odd_kummer_census.py --k 22 --p 4194301 --A 43186 --n 4197848 \
  --d 15 25 33 55 75 93 165 --K 2000 --seed 20261104 --out-dir data/
python3 -I scripts/analyse.py          # -> data/summary.json, data/analysis.txt, data/selected_cosets.json
```

k = 18 has no composite odd d ≤ 600 (13, 71 only) and k = 24 only 69 = 3·23 and
267 = 3·89 (a 23-level, degree 22): neither is run; k = 24 may be added later as a
control at d = 69 only.

### 6.3 Cells and budget

k = 20: 14 values of d × 5 frames × ≤ 2000 cosets = 140,000 evaluations; the
largest `coverage` call is on 2·256 logs (a 513² array); the whole census is one
Python process, no engine, no bench lock, estimated well under an hour on one
core (L14's 12,000 evaluations of ≤ 64 abscissae ran in minutes; here the work
is dominated by the d ≥ 171 rows, ≈ 4 × 10⁴ rows × ≤ 2.6·10⁵ array elements).
k = 16 and k = 22 add 4 and 7 values of d. Log tables: 4r additions = 3.3·10⁴
(k = 16), 5.2·10⁵ (k = 20), 2.1·10⁶ (k = 22) Python group additions. If the k = 22
table exceeds the restart window, its run is split per d (the table is rebuilt
each launch; that is instrument cost, counted once in the report).

Primary cells (the predictions of section 7 are decided on them): k = 20,
d ∈ {21, 27, 63}, frame `u`. Everything else is control or replication.

### 6.4 Instrument check (P1 and the fixed references; must hold before anything else is read)

`analyse.py` recomputes, with the same `logu`, the L14/#3359 reference `bits
lo = 0` coverage at t = 4, 5, 6 (must equal 12, 42, 162 exactly at k = 20) before
reading any odd-Kummer row. If that fails, the run is an instrument failure:
report, fix nothing else, stop.

## 7. Preregistered predictions and decision rules

All rules are evaluated by `scripts/analyse.py` on `data/k{k}_d{d}.json`; the
script is frozen with this protocol. q is as defined in 2.2; "mean" is over
the coset rows of one (d, frame) with cols ≥ 8; "max" likewise. Failed
predictions stay failed; nothing is retuned after data.

1. **P1 (the lemma, exact).** At k = 20, in frame `u`, every census row has all
   base-point labels of one parity, and that parity is even when χ(g) = +1 and odd
   when χ(g) = −1. Rule: the number of frame-`u` rows (all d) with `even > 0 and
   odd > 0`, plus the number with parity ≠ the χ(g) rule, is 0. One violating row
   fails P1 (and section 9 applies).
2. **P2 (the ×2 shift).** For every d ∈ {21, 27, 57, 63, 133, 171, 189, 219, 399,
   511, 513} at k = 20: mean q in frame `u` ∈ [1.8, 2.6], and mean q in each of
   the frames `w0, t1, t2, t3` ∈ [0.95, 1.35]. Rule: all listed d satisfy both
   bands; one (d, frame) outside fails P2. (Expected shape, from T2: frame-`u`
   mean q decreasing in d from ≈ 2.3 at d = 21 towards ≈ 2.05 at d ≥ 189.)
3. **P3 (selection on top of the shift).** For each d ∈ {21, 27, 63}: max q in
   frame `u` ≥ 1.5 × max q over the pooled rows of frames `w0, t1, t2, t3` at the
   same d. Rule: holds for all three d; one failure fails P3.
4. **P4 (columns are Legendre-random; no column saving from μ_d).** For every
   d ≥ 21 at k = 20 and every frame: mean cols / d ∈ [0.45, 0.55]. Rule: all
   (d, frame) inside the band.
5. **P5 (the lead cells).** Best coset by `covered/cols` in frame `u`: at d = 27,
   `covered/cols ∈ [8, 16]` (point 11); at d = 21, `covered/cols ∈ [6, 13]`
   (point 9); at d = 63, `covered/cols ∈ [16, 30]` (point 22). Rule: each cell
   is decided separately; P5 holds if all three hold.
6. **P6 (log-free selection works).** Spearman(J, covered) over frame-`u` rows
   ≥ 0.99 at each d ∈ {21, 27, 57, 63}. Rule: all four.
7. **P7 (family replication).** At k = 16 (d = 21, 57) and k = 22 (d = 15, 25,
   33, 55, 75): mean q in frame `u` ∈ [1.8, 2.6] and mean q in each translation
   frame ∈ [0.95, 1.35], for every listed d. Rule: all seven (d, frame) sets.
8. **P8 (both parity classes gain alike).** In frame `u` at d = 27 and d = 63:
   mean q over cosets with χ(g) = −1 divided by mean q over cosets with
   χ(g) = +1 ∈ [0.85, 1.18]. Rule: both d.

P1–P4 and P8 test the mechanism; P5 and P6 test whether the mechanism is
*selectable* to the level the model needs; P7 tests that it is a family fact,
not a k = 20 accident.

## 8. Go/no-go before any engine work (decided by `analyse.py` from the census alone)

For each d ∈ {21, 27, 63, 189} take the frame-`u` coset maximising
`covered/cols` (ties: fewer cols, then smaller i) and compute the modelled merit
ratio `ρ_κ(d) = [covered/(cols · mpt_model(d, κ))] / Φ_w`, with `mpt_model` of
3.1 and `Φ_w = 1.807·10⁻⁴`. Report ρ at κ = 1, 1.5, 2 for every d.

- **GO** (write the `PowerRule` change and a Q-B protocol): P1 and P2 hold, and
  `max_d ρ_{1.5}(d) ≥ 1.3` (a modelled win that survives the 1.07–1.19× model
  error T12 measured against T10).
- **PARK** (no engine work now; revisit if L12's trace replay changes the
  per-branch cost model): P1 and P2 hold and `0.7 ≤ max_d ρ_{1.5}(d) < 1.3`.
- **NO-GO** (close the lead; record it in LEADS.md as closed by census): P1 or
  P2 fails, or `max_d ρ_{1.5}(d) < 0.7`, or `max_d ρ_1(d) < 1` (even a free odd
  level would lose).

In `covered/cols` terms (3.2): GO needs ≥ 8.1 at d = 21, ≥ 11.8 at d = 27, ≥ 42
at d = 63 or ≥ 218 at d = 189. The P5 point predictions (9, 11, 22) sit at
ρ_{1.5} ≈ 1.45, 1.2, 0.68: **the design's own expectation is a marginal GO/PARK
at d = 21–27, decided by the selected coset, and a NO-GO everywhere else.** That
is written down now so that an outcome in the PARK band is not read as a win.

A GO authorises only the engine change and its correctness gate (section 4) and a
new preregistered f4cost protocol; it does not authorise an ecbench session.

## 9. Failure handling

- P1 fails (mixed parities in frame `u`): the lemma or an assumption behind it
  (B = 1, cyclic Z/8, `x_W = u + A/3`) is wrong for this family. Stop; check the
  assumption against `frame_census.build` (the group generator and `A3`) and
  the T2 rows (`t2_coset_hist`); do not proceed to P2–P8 or section 8 until the
  discrepancy is explained in the report. If the parity rule is merely inverted
  (χ(g) = −1 ↔ even), P1 still fails as written; the mechanism survives and the
  census continues, with the failure recorded.
- P2 fails only in the translation frames (q ≠ 1 there): the frame is not the
  only source of structure; report the frame-wise means and treat the `u`-frame
  gain as "relative to the translation frames of this census", never as ×2 over
  random.
- P2 fails in frame `u` on the low side (mean q < 1.8): the mod-4/8 labels are
  anti-correlated within the square class; the ×2 mechanism is weaker than the
  lemma says. Section 8 is still evaluated (NO-GO by rule), and the per-d
  `hist8` is reported as the exploratory explanation.
- P3 or P5 fails on the low side while P2 holds: the shift is real but not
  selectable at K = 2000. Section 8 decides; a larger K is **not** run in this
  lead (that would be a post-hoc search; a follow-up may preregister it with the
  selection cost charged).
- P6 fails: selection must use logs, i.e. costs a discrete log per candidate;
  report, and make the go/no-go conditional on the search cost per curve (one
  log table of 4r group additions, which is already the instrument cost).
- P7 fails at one curve: report the family split; the k = 20 go/no-go stands,
  flagged "not replicated".
- Container restart mid-census: relaunch `run.sh`; per-(k, d) files are
  deterministic and complete ones are skipped. The log table is rebuilt (counted
  once).
- Instrument check (6.4) fails: instrument failure; stop.

## 10. Scope, costs and distance remaining

- **Scope.** Design plus an exact pure-Python census. No engine run, no Rust,
  no ecbench, no bench lock, no timing, no leaderboard change. Writes only inside
  this lead directory. The census is a mechanism measurement in exact counts
  (coverage, columns, Legendre symbols, `[r]P` labels, log-table additions).
- **Unpriced work.** None is priced here because nothing is solved. In the
  eventual comparison, the per-curve cost of the coset search (K·d Legendre
  symbols plus Σ cols labels at 1.5·log₂ r group operations, or one log table of
  4r additions) must be charged or amortised exactly as T12 charged the isogeny
  seed search (6.94·10⁶ group operations = 1.92·10⁴ in S at k = 20), at w = 0,
  0.1 and 1.
- **Distance remaining.** The T10 winner is 6.06·10⁴× behind rho-neg at w = 0.1
  and 17.1× at w = 0 (T12, k = 20, 16 targets). The most this design models,
  under its own point predictions, is ρ ≈ 1.2–1.5 at κ = 1.5 (and ≈ 2.2 at
  κ = 1), i.e. a best case of about 2× on the F4 term: the arm would remain
  ≥ 3·10⁴× behind rho-neg at w = 0.1 and would not move the w = 0 order (the
  w = 0 term is columns/covered; at d = 27 that is ≈ 13.5/150 ≈ 0.09, about the
  winner's 0.105). Rho remains ahead on every track; nothing here is a claim.
- **Deviations from the standing rules, declared now:** no Conductor task and
  no S3 resume checkpoint (design only, no run launcher invoked, `conductor` not
  installed on this host); no git; the research-visuals report is produced only
  if the census runs (then: one figure, q per d per frame with the reference
  rows of 2.3, and the PDF).

## 11. Other family curves: odd part of p − 1 and the usable d (summary of 2.1)

| k | odd part of p − 1 | composite odd d ≤ 600 with e₀ ≤ 7 (tower candidates) | single-prime d (coverage controls only) | verdict for L11 |
|--:|:--|:--|:--|:--|
| 16 | 3·7·19·41 | 21 (3·7), 57 (3·19), 133 (7·19), 399 (3·7·19) | 7, 19, 41 | replication curve: d = 21, 57 |
| 18 | 13·71² | none | 13, 71 | **no odd-Kummer base exists**; only mixed 52 = 4·13, 284 = 4·71 |
| 20 | 3³·7·19·73 | 9, 21, 27, 63, 133, 171, 189, 399, 513 (+ 57, 219, 511 with e₀ = 3 but a 19/73-level) | 19, 73 | primary curve: d = 21, 27, 63 |
| 22 | 3·5²·11·31·41 | 15 (3·5), 25 (5·5), 33 (3·11), 55 (5·11), 75 (3·5·5), 93, 123, 155, 165 (3·5·11), 205, 275, 341, 451, 465 | 5, 11, 31, 41 | replication curve: d = 15, 25, 75 (e₀ ∈ {3, 5}) |
| 24 | 3·23·89·683 | 69 (3·23, a 23-level), 267 (3·89) | 23, 89 | effectively none (every tower carries a level ≥ 23) |

The family is uneven: on the real Curve25519 (`p = 2²⁵⁵ − 19`,
`p − 1 = 4 · (2²⁵³ − 5)`) the only odd prime ≤ 73 dividing the odd part is 3
(checked by arithmetic now: 5, 7, 9, 11, …, 73 do not divide it), so d = 3 is the
only small odd order and it is a single level; this lead can at best inform the
miniature benchmarks, not the 255-bit curve, unless the full factorisation of
2²⁵³ − 5 (not computed here) offers a composite d with small factors. That limitation is
part of the design and is stated here before data.

## 12. Files (at freezing: this file only)

- `PROTOCOL.md` (this file).
- To be added at freezing: `scripts/odd_kummer_census.py`, `scripts/analyse.py`,
  `scripts/run.sh`, with sha256 recorded in a "Frozen at" line here.
- After the run: `data/k{16,20,22}_d*.json`, `data/k*_frames.json`,
  `data/summary.json`, `data/analysis.txt`, `data/selected_cosets.json`,
  `data/SHA256SUMS`, `README.md` with the P1–P8 table and the section 8 verdict.
