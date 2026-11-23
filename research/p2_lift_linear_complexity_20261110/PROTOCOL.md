# F2: linear complexity of the p²-lift defect along the group (protocol, frozen 2026-11-10)

Follow-up to `research/p2_lift_defect_20261101/` (H110), whose README closes
only "the frozen coordinate-section scalar-polynomial bridge" and "does not
rule out other lift-derived invariants". Class: **boundary diagnostic**, no
pipeline `S`, no rho comparison, no leaderboard row. Frozen before any
sequence was computed; `FREEZE.sha256` binds this file and the tool source.

## 0. Setting and the one identity this rests on

`E / F_p : y² = x³ + ax + b`, a point `G` of prime order `r`, `r ∤ p`.
Reduction gives `0 → E₁(Z/p²) → E(Z/p²) → E(F_p) → 0` with
`E₁(Z/p²) ≅ F_p` by the formal logarithm `t/p` (H110 PROTOCOL §"typed
correspondence"). Because `r` is prime to `p`, the sequence splits on `⟨G⟩`
uniquely: `h(Q)` = the unique lift of `Q` of order `r` (Silverman's
torsion lift). H110's section `s₀` (fixed `X = x`, `Y` Hensel-lifted) is a
set map, and H110 measured `τ₀(Q) = log_formal([r] s₀(Q))`.

**Identity (derived).** Write `s₀(Q) = h(Q) + f(Q)` with `f(Q) ∈ E₁ ≅ F_p`.
Then `[r] s₀(Q) = [r] f(Q)` (as `[r] h(Q) = O` and `E₁` is central), so
`τ₀(Q) = r · f(Q) (mod p)` and `f = τ₀ / r`. H110's "scalar word" is `r·f`;
its `torsion_u(Q)` is the `x`-digit of `h(Q)`: `X(h(Q)) = x + p · torsion_u(Q)`.
Every "other lift-derived observable" of the first formal layer is a function
of `f` and of coordinate-rational functions of `Q`. What H110 did **not**
measure is the translation structure of `f` along the group.

**Mechanism tested.** Let `a_k = f([k]G)`, `k ∈ Z/r`, `a₀ = 0`, an
`r`-periodic sequence over `F_p`. Since `gcd(r, p) = 1`, `x^r − 1` is
squarefree over `F_p`, and the linear complexity `L(a)` (length of the
shortest linear recurrence over `F_p`) equals the number of nonzero
components of the discrete Fourier transform of `a` over `F_{p^m}`,
`m = ord_r(p)` (Blahut's theorem; standard, **not formalized**). `x^r − 1 =
(x − 1) · Π` of `(r − 1)/m` irreducibles of degree `m` (`r` prime), so
`L(a) = [mean(a) ≠ 0] + m · c(a)` with `c(a)` the number of nonzero
degree-`m` components, `0 ≤ c ≤ (r − 1)/m`. If `c(a) ≪ (r − 1)/m`, then
`f([k]G) = Σ_j Tr(α_j ζ_j^k)` is a sparse exponential polynomial in `k` and
the ECDLP transfers to a DLP in `μ_r ⊂ F_{p^m}^*` (a MOV-shaped transfer
through the lift rather than a pairing; `m` is the embedding degree). That
is the only mechanism by which the first formal layer could carry
non-generic information about `k`, given the identity above.

## 1. Question

Is `c(f₀)` (and `c(torsion_u)`) below the null on any curve, where the null
is a uniformly random `r`-periodic sequence over `F_p`?

## 2. Instrument

One native Rust tool, its own std-only crate at `tool/` in this directory
(`tool/Cargo.toml`, `tool/src/main.rs`; no dependencies; arithmetic mod `p`
and `p²` in `u128`), with:

- Hensel section `s_u` exactly as H110 `run.py` `hensel_lift` (`X = x + p·u`,
  `Y = y + p·v_u`, `v_u = (c_Q + (3x² + a)u)/(2y) mod p`,
  `c_Q = (x³ + ax + b − y²)/p mod p`; asserts `y ≢ 0`).
- `[r] s_u(Q)` by the split `[s]L + [r − s]L` with two splits that must agree
  (H110 `mixed_kernel_sum`), `τ_u = (t/p) mod p`, `t = −X·Z/Y mod p²`.
- `f_u = τ_u · r^{−1} mod p`; `torsion_u = −τ₀ / (τ₁ − τ₀)`; the lift at
  `u = torsion_u` is checked to have `τ = 0` (H110's own control, kept).
- Berlekamp–Massey over `F_p` on the doubled sequence `a_0 … a_{2r−1}`
  (period `r`, so `2r` terms determine `L ≤ r` exactly), returning `L` and
  the minimal polynomial degree; `c = (L − [mean ≠ 0]) / m` must be an
  integer (checked; a non-integer is an instrument failure).
- `m = ord_r(p)` by exponentiation.

## 3. Rows

| set | curves | why |
|---|---|---|
| H | the seven H110 curves `E101 … E907` (`p, a, b, r` from H110 `run.py` `CURVES`), generator = H110 `first_point` | continuity with H110's 28 rows |
| R12, R14, R16 | 4 prime-order curves each: `p` = the largest prime below `2^bits`, `(a, b)` from seed `F2/curve/<bits>/<i>`, `i = 0..3`, first `(a,b)` with `4a³ + 27b² ≠ 0` and `#E` prime (`#E` by exhaustive point count), `j ∉ {0, 1728}`, `#E ≠ p` | fresh instances, no selection on the result |
| S | 2 supersingular curves `y² = x³ + b`, `p ≡ 2 (mod 3)`: for bits ∈ {15, 16}, `p` = the largest prime `≡ 2 (mod 3)` below `2^bits` whose `p + 1` has a prime factor `r ≥ 2^12` (stepping down over primes that fail; the skipped primes are recorded), `#E = p + 1 = c·r`, `G` of order `r`; `b` from seed `F2/ss/<i>` (amended before the full run: `6 | p + 1` makes `r ≥ 2^12` impossible below `2^14`, so the original "p ≥ 2^14" wording with bits 14/15 was unsatisfiable) | embedding degree `m = 2`: the only curves where the null allows vanishing components with non-negligible probability and where the positive control can be built in `F_{p²}` |

Generator multipliers `{1, 2, 3, 5}` on every curve (A/A: decimation by a
unit of `Z/r` permutes the DFT components, so `L` and `c` must be identical
across multipliers; any difference is an instrument bug, not a result).

## 4. Observables per row (sequences of period `r`)

| id | sequence `a_k`, `k ∈ Z/r` | role |
|---|---|---|
| O1 | `f₀([k]G)` | **gated** |
| O6 | `torsion_u([k]G)` (x-digit of the canonical torsion lift) | **gated** |
| O2 | `f₁([k]G)` (section `u = 1`) | reported |
| O3 | `f₁ − f₀` (a rational function of `(x, y)` alone: coordinate-only) | negative control of the same shape |
| O4 | 200 uniformly random sequences per curve, seed `F2/null/<curve>/<j>` | empirical null of `(L, c)` |
| O5 | on set S only: `Tr_{F_{p²}/F_p}(α ζ^k)`, `ζ ∈ μ_r ⊂ F_{p²} = F_p[i]/(i² + n)`, `n` the smallest positive integer with `−n` a non-residue, seed `F2/plant/<curve>` | positive control: must give `L = 2`, `c = 1` |
| O7 | `a − mean(a)` for O1 | positive control for the trivial component: `L` drops by exactly 1 iff `mean ≠ 0` |

## 5. Null (exact)

For a uniformly random `r`-periodic sequence each degree-`m` component is
zero with probability `p^{−m}`, independently, so `c ~ Binomial((r−1)/m,
1 − p^{−m})` and `P(c ≤ (r−1)/m − 1) ≤ ((r−1)/m) · p^{−m}`. Both the exact
formula and the 200-sequence empirical null are reported per curve.

## 6. Predictions (frozen)

P1. On every curve in H, R12, R14, R16 (large `m`): `c(O1) = c(O6) = (r−1)/m`
(no vanishing component), `L ∈ {r − 1, r}`.
P2. On S (`m = 2`): `c(O1) = c(O6) = (r−1)/2` on both curves.
P3. O3 (coordinate-only) behaves as P1/P2 (it is the expected null shape).
P4. O5 gives `L = 2` exactly; O7 drops `L` by exactly 1 when `mean ≠ 0`.
P5. A/A: identical `(L, c)` across the four multipliers on every row.
Proposer's prior that any gated observable breaks P1 or P2: ≤ 0.02.

## 7. Decision rules (first applicable line)

1. **Instrument failure:** P4 or P5 fails, or any H110 self-check fails
   (split disagreement, `τ(torsion_u) ≠ 0`, non-integer `c`). Fix, re-freeze,
   re-run; nothing below is read.
2. **Lead alive:** on any curve with `((r−1)/m)·p^{−m} < 10^{−3}`, a gated
   observable has `c < (r−1)/m` (null probability `< 10^{−3}`); or on S a
   gated observable has `c ≤ (r−1)/2 − 3` (null `≲ 10^{−12}`). Then: replicate
   on 8 fresh curves of the same shape before any further claim; record as
   "structure in the first formal layer" with the measured `c`.
3. **Closed:** otherwise. Reading: the first formal layer's defect has a full
   DFT support on every curve measured; a sparse-exponential-polynomial
   transfer through the `p²`-lift is closed for the fixed-`x` section, the
   `u = 1` section and the canonical torsion lift's `x`-digit, at these sizes.

Rule 3 does not establish a generic lower bound, says nothing about higher
formal layers (`p³`, H112) or about sections that fix other coordinates,
and is a measurement at toy sizes (`r ≤ 2^16`).

## 8. Cost accounting

Per target: one `s_u` lift (one inversion mod `p`), two `[s]`-chains mod `p²`
(≈ `2 log₂ r` doublings and additions, one inversion each in affine form),
for `u ∈ {0, 1}`. Per row: Berlekamp–Massey `O(r²)` field operations. Counts
(lifts, mod-`p²` inversions, BM operations) are written to the result file.
No wall time is a result; the run is exact integer computation.

## 9. Execution route

Exact integers, no timing: run locally first (`cargo run --release --manifest-path tool/Cargo.toml --
--out results/run.json`), record host and
binary SHA-256, then submit the same binary through isolab-eks by the
GitHub-identity route (`isolab-experiment.yml`) if available from this
session and diff the two result files byte for byte. A run without the EKS
diff is "local evidence; §9a confirmatory run open" (as
`research/lift_dense_followups_20261027/`).

## 10. Outputs

`results/run.json` (every row: curve, multiplier, observable, `L`, `c`,
`mean≠0`, `m`, null parameters, counts, checks), `README.md` with the
decision-rule reading and `[pcc:unverified]` labels, one SVG graph
(`c/((r−1)/m)` per observable per curve, with the null band), `REPORT.pdf`
(research-visuals), session-log entry.

## 11. Disclosure: what was seen before the freeze

The predictions of §6 and the rules of §7 were written (commit `4da4a01f06`)
before the tool existed. While building the tool the implementing agent ran
`--quick` (set H, multiplier 1, 10 null sequences), set R12 and set S with 3
null sequences each; every row had `c = N` and O5 gave `L = 2`, consistent
with P1, P2 and P4. Those outputs are not committed and are not scored. §3's
set-S selection rule and §4's O5 modulus were made precise in this amendment
(the originals were unsatisfiable or ambiguous); no prediction or rule
changed. The frozen full run (`--null 200`, all sets, all multipliers) is the
one that is read.
