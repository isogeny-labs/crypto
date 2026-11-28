# M31 single-target ECDLP: curve structure and what it buys against a negation-map rho

2026-11-26 · branch `claude/m31-curve-structure-20261126` · PR #5252 · all numbers **[pcc:unverified]**
(producer evidence only: no pcc claim, no second-host replay, not through the validation gate).

## Result

- **B677 M31 curve** (p = 2^31 − 1, y² = x³ + 847514846x + 2087444724, n = 2147446831): it has **no usable
  non-generic structure.** Aut(E) = {±1}. End(E) = ℤ[π], an order of conductor 3 in K = ℚ(√−803827011). The smallest
  endomorphism outside ℤ has degree ≥ 2.0·10⁸. The isogeny class contains no curve with j = 0 or 1728. The embedding
  degree is 429489366 ≈ 2^28.7, and the curve is not anomalous. So the only automorphism lever is the negation map,
  which every curve has. The best equal-engineering single-target method on B677 is negation-map rho.
- **Negation-map rho vs the PR #5184 plain rho** (B677, 2000 paired solves of the frozen public Q):
  **0.716 [0.694, 0.739]** in group operations and 0.726 [0.703, 0.749] in walk field multiplications. Walk wall time
  is 0.943 [0.913, 0.975], indicative only. The prediction was 1/√2 = 0.707. This is a constant factor from the
  generic ±1 automorphism, not structure.
- **The size of the structural lever at M31 (j = 0 class).** No j = 0 curve over GF(2^31 − 1) has prime order. The best
  one is y² = x³ + 5, with #E = 13 · 165188041. On its prime subgroup, order-6 automorphism-orbit rho against
  negation-map rho gives **0.580 [0.562, 0.599]** in group operations (W = 16, DP 3) and 0.611 [0.594, 0.628] at the
  B677 engineering setting (W = 64, DP 5). In walk field multiplications the ratios are 0.635 [0.615, 0.656] and
  0.692 [0.673, 0.711]. The prediction was 1/√3 = 0.577. This is a **constant factor**: the exponent is still ½ and the
  per-step work is the same. j = 1728 curves over this field are supersingular, with #E = p + 1 = 2^31 and embedding
  degree 2. Pohlig–Hellman breaks them trivially, so they are not a candidate class.
- **Exponent-changing structure: none found on any of the three curves.**

## 1. Structure (`evidence/structure.json`, `native/src/bin/structure.rs`)

Everything below is exact integer arithmetic in Rust, using u64 and u128 values with no CAS. No Sage or PARI was available
offline, and package installation was not permitted. Each fact was computed two ways where a second way exists.

| fact | B677 | M31 y² = x³ − 3x + 21 | how verified (A / B) |
|---|---|---|---|
| #E | 2147446831 (prime) | 2147487637 (prime) | A: n prime, in Hasse window, nG = O (affine and Jacobian code paths); B: BSGS over the whole Hasse window for 3 random points, unique survivor = n |
| trace t | 36817 | −3989 | t = p + 1 − n |
| D = t² − 4p | −7234443099 = −3³·13·20610949 | −8574022467 = −3²·773·1232431 | trial division, product re-check, each factor Miller–Rabin prime |
| d_K, conductor of ℤ[π] | −803827011, f_π = 3 | −952669163, f_π = 3 | D = f_π² d_K, d_K ≡ 1 mod 4 |
| End(E) (3-volcano level) | **floor: End = ℤ[π]** (conductor 3) | **crater: End = O_K** (conductor 1) | A: number of F_p-roots of ψ₃ (gcd with x^p − x, and separately x^(p−1) − 1): 1 vs 4; B: size of E′(F_p)[3] on the quadratic twist (π acts as −1 on E[3] iff the twist has full 3-torsion): 3 vs 9 |
| j | 2145262698 | 1390704497 | A: 1728·4a³/(4a³ + 27b²); B: c₄³/Δ |
| Aut(E) | {±1} | {±1} | j ∉ {0, 1728} by both j formulas |
| smallest non-integer endomorphism degree | ≥ (abs(d_K) + 1)/4 = 200956753 | ≥ 238167291 | norm form of O_K |
| rational ℓ-isogenies, ℓ ≤ 47, ℓ ∤ D | 2 each for ℓ = 5, 19, 23, 29, 43; else 0 | 2 each for ℓ = 7, 11, 13, 17, 29, 41, 47; else 0 | roots of x² − tx + p mod ℓ; ℓ = 2 also by roots of x³ + ax + b (0, both methods) |
| embedding degree | 429489366 = (n − 1)/5 | 2147487636 = n − 1 | A: order of p mod n from the factorisation of n − 1, with pⁿ check; B: no k ≤ 5000 |
| anomalous | no | no | n ≠ p |
| twist order | 3³·5·23²·30071 | 3²·13·191·96097 | (not relevant for a validated single target) |
| public Q = 987654321·G | yes | | affine and Jacobian scalar multiplication |

What this rules out for B677:

- **Automorphism-orbit rho beyond ±1.** It needs j ∈ {0, 1728}, and B677 has neither.
- **Isogeny transfer to a curve with extra automorphisms.** This was the lever in workspace PR #54: crater/floor via a
  Vélu bridge, giving 0.611× wall. Isogenous curves share Frobenius and hence d_K = −803827011. A curve with
  Aut ⊋ {±1} needs d_K ∈ {−3, −4}, so no isogeny of any degree reaches one. The ascending 3-isogeny to the crater
  only changes End from ℤ[π] to O_K. The unit group stays {±1}.
- **GLV-style endomorphism acceleration.** The cheapest endomorphism outside ℤ has degree about 2·10⁸, so evaluating it
  once costs far more than the whole rho run of about 4.4·10⁴ steps. GLV also speeds up scalar multiplication, not
  r-adding walk steps.
- **MOV/FR and anomalous (Smart) transfers.** The embedding degree is about 2^28.7, and the trace is not 1.
- **Weil descent.** p is prime, so there is no subfield.

Rational ℓ-isogenies (ℓ = 5, 19, …) exist. They map to curves of the same d_K with Aut = {±1} and the same group
order, so they give no single-target lever.

**The j = 0 class over GF(2^31 − 1)** (`j0_class` in structure.json): 4p = 92324² + 3·4698². There are six twists, and
none has prime order. The best near-prime twist is y² = x³ + 5 with n = 2147444533 = 13·165188041, verified by killing
random points and by BSGS over the Hasse window. The next best is 3·109·6567391. For j = 1728, p ≡ 3 mod 4 makes it
supersingular, with #E(y² = x³ + x) = 2^31 confirmed by BSGS.

## 2. Measurement (`PROTOCOL.md`, frozen before data, two amendments)

**Engine.** `native/src/lib.rs` is a single Rust rho engine with three class policies:

- `plain`: classes {P}. It re-implements the PR #5184 `rho_batched.rs` policy: 32-jump r-adding walk, W = 64 walks,
  one batch inversion per step, Mersenne reduction, DP bits 5.
- `neg`: classes {±P}. It adds Wiener–Zuccherato look-ahead against fruitless 2-cycles and an x-only cycle ring
  (distances 2 and 4 every step, full 12 every 16 steps). A detected cycle is confirmed by replay and left
  deterministically by doubling its minimum point. The cost of the replay and the doubling is counted.
- `aut6`: classes {±ζⁱP}. The ζ-orbit minimum costs 1 multiplication, because β²x = −x − βx. Coefficients are scaled
  by λ with λG = (βx_G, y_G), checked on G.

Everything else is identical across arms: the jump table, the start schedule S_k = (k + 1)X, DP, and the restart rule.
Every field multiplication, squaring and inversion is counted. Each log is verified three ways: by the engine, by an
independent Jacobian d·G == Q, and against the evaluator-held log. All **24,000 / 24,000** solves verified.

**Seeds.** Policy isogeny-labs/experiment-seeds/v1. The seeds are in `evidence/seed_manifest.json`, sha256
c671ac47…; master 4d33315f72686f5f. Each stream key is SHA-256(master/domain/curve/index/arm/W/dp), using a
self-contained SHA-256 that passes the FIPS "abc" vector. The arm streams are independent, and `neg_aa` is a second,
independent `neg`. Targets are the frozen public Q for B677 and seeded k·G otherwise, with the logs kept on the
evaluator side. Arms are interleaved by rotating the order each trial. Everything ran on 1 thread on a shared
4-vCPU host.

**Replay.** A second full run of the panel reproduced all 24,000 rows bit-for-bit in every counted field. Only the wall
fields differ. The manifest is byte-identical.

### Paired ratios (ratio of means, 95% percentile bootstrap, 10,000 resamples, 2000 pairs each)

| comparison | predicted | group ops | walk field mults | walk wall (indicative) |
|---|---:|---|---|---|
| B677 neg / plain | 0.707 | **0.716 [0.694, 0.739]** | 0.726 [0.703, 0.749] | 0.943 [0.913, 0.975] |
| B677 A/A (neg_aa / neg) | 1 | 1.028 [0.997, 1.059] | 1.028 [0.997, 1.060] | 1.021 [0.990, 1.054] |
| M31(−3,21) neg / plain | 0.707 | 0.728 [0.705, 0.751] | 0.738 [0.715, 0.761] | 0.960 [0.929, 0.991] |
| M31(−3,21) A/A | 1 | 0.988 [0.958, 1.019] | 0.988 [0.958, 1.019] | 0.988 [0.957, 1.020] |
| j=0 neg / plain | 0.707 | 0.738 [0.718, 0.759] | 0.749 [0.728, 0.770] | 0.972 [0.944, 1.001] |
| j=0 A/A | 1 | 1.006 [0.977, 1.035] | 1.006 [0.978, 1.035] | 1.008 [0.978, 1.037] |
| **j=0 aut6 / neg** (W 64, DP 5) | 0.577 | **0.611 [0.594, 0.628]** | 0.692 [0.673, 0.711] | 0.767 [0.744, 0.792] |
| **j=0 aut6 / neg** (W 16, DP 3) | 0.577 | **0.580 [0.562, 0.599]** | 0.635 [0.615, 0.656] | 0.678 [0.656, 0.700] |
| j=0 aut6 / plain (W 64) | 0.408 | 0.451 [0.438, 0.464] | 0.518 [0.503, 0.533] | 0.746 [0.722, 0.771] |

Mean steps: B677 plain is 61200, or S = 1.32√n, against the PR #5184 report of about 54–68k. B677 neg is 43800
(S = 0.945). j=0 aut6 at W 16 is 6917 (S = 0.538, against the ideal √(π/12) = 0.512). Full cell table and the
whole-solve rows: `evidence/summary.md`. Figure: `figures/ratios.svg`.

Reading the table:

- **Constant factor from automorphisms.** Every gain is in the constant: ±1 gives about 1/√2 and μ₆ gives about 1/√3,
  each within a few percent of the prediction.
  - At W = 64, DP 5, the DP latency of about W·2^DP ≈ 2000 steps is a fixed overhead. It compresses the aut6 ratio on
    a group this small, where aut6 needs about 8300 steps. At W = 16, DP 3 the measured ratio is the predicted 0.577.
  - aut6 pays 1 extra multiplication per step, about 7.9 against 6.9 for the other arms. That is why its field-op ratio
    sits above its group-op ratio.
- **Exponent.** No arm changes the exponent. The measured S stays flat because every arm is √n with a different
  constant.
- **§5a band.** We priced the uncharged work at 0, 0.1 and 1 GAE per operation: canonicalisations, cycle checks and
  DP-table operations. neg/plain keeps its order at 0.1 GAE (0.856 [0.830, 0.883]) and reverses at 1 GAE
  (2.08 [2.02, 2.15]). The walk-wall ratio of 0.94 shows that the real price of a canonicalisation plus a cycle check
  on this engine is about 0.3 GAE. aut6/neg is the same across the band, because both arms do the same uncharged work
  per step.
- **Wall time.** It is indicative only, on a shared host. Setup costs about 0.3 ms per solve: 64 scalar
  multiplications, each with its own inversions. That setup is identical in every arm and dilutes the "incl. setup"
  ratios. Batching it would move the total-mults ratios toward the walk-mults ratios.

**Cross-curve (unpaired; not a B677 speedup).** j=0 aut6 on the r = 165188041 subgroup against B677 neg gives steps
0.190 [0.184, 0.195]. This mixes the μ₆ factor with a 13× smaller prime subgroup, so it says that **choosing** a j = 0
M31 curve gives a weaker group. It does not say anything about B677.

## 3. Conclusion for B677 single target

B677 has no structure an attacker can use beyond the generic negation map. The honest best single-target attack is
negation-map rho, at 0.716× the group operations of the PR #5184 comparator. Every relation-source result should
therefore be compared with a rho that is about 0.72× (group ops) to 0.94× (walk wall, this engine) of
`rho_batched.rs`. The 18.5× gap reported for PR #5202 would widen to roughly 18.5/0.716 ≈ 26× in group operations.
That figure assumes the 18.5× was measured in the same unit; it is not re-measured here **[pcc:unverified]**. On a
structured M31 curve the automorphism lever is a measured constant factor of 0.58–0.61 over negation rho, which is
√3 and no more. Crossing an isogeny gap to reach such a curve is impossible from B677.

## 4. Limits

- One host (4 vCPU, shared with two other agents). Wall times are indicative. There is no second-host replay, and the
  validation gate (`VALIDATION.json`, matched ecbench rho reference) was not run. Nothing here is a leaderboard row.
- The `plain` arm re-implements the PR #5184 policy. It does not compile that file byte-for-byte. Its mean of 61.2k
  steps falls inside the 54–68k that #5184 reports.
- The structure facts use two methods in our own code. No external CAS was available. A Sage/PARI cross-check of
  #E, D and the End level is the cheapest independent confirmation.
- The research-kb MCP server was not available in this session, so no "not tried" claim is made. Prior lab work was
  read: `workspace:docs/notes/crater-floor-20261121/README.md`, PR #54.
- Engine history: the first native run (`evidence/v1_engine/`) had a negation arm that was slower than plain in wall
  time. The cause was branch mispredictions and a full-state cycle ring scanned every step. The engine was fixed
  (Amendment 2) and the panel rerun. Group-op ratios are unchanged within noise: v1 gave 0.715 against 0.716 now.

## Reproduce

```
cd native && CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo build --release
./target/release/structure > ../evidence/structure.json
./target/release/panel ../evidence 2000          # refuses a different pre-call manifest
./target/release/analyze ../evidence              # needs evidence/panel.jsonl (gunzip -k panel.jsonl.gz)
./target/release/rho neg 847514846 2087444724 2147446831 625037715 2100627251 976979610 1228979761 0x1234 64 5
```
