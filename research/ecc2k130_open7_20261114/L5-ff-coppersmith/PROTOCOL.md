# L5 protocol: function-field Coppersmith on binary S_3 — bound vs existence gap

Frozen 2026-11-14, before any run with the measured seed `20261114`.
Lane L5 of `research/ecc2k130_open7_20261114/` (rules: `../COMMON.md`, incl. Amendment 1: Rust only).
Instrument: `ffcop/` (Rust, no dependencies), binary `ffcop` built with
`CARGO_TARGET_DIR=$SP/lanes/L5/target` (private target dir: the shared flock queue was blocked
behind other lanes' workspace builds; this crate shares no target with any repository build).

## 0. Prior art and residual question

- Prime-field analogue measured: `crypto:research/coppersmith_s3_theta2_20261023/REPORT.md`
  (box lattice, LLL, l<=3; recovery stops at beta 0.12–0.16, HG at beta_det). `[pcc:unverified]`
- Binary F_2[t] analogue: idea `MP-lattices-5` in `crypto:research/ic_mega_pass_20261031/`
  (`ideas/lattices.json`, three skeptic verdicts `verdicts/MP-lattices-5.*.json`, ranking says
  "verification item only"). **Designed and reviewed, never run** (no `experiments/MP-lattices-5`).
  The design verdict's fixes are adopted here: per-(n,k) integer-l scans, generic null with the
  same support, planted vs unplanted, second-vector/extraction success, det identity as reducer
  unit test, n = 131 run directly.
- 2026-11-13 L2 lattice lab (`crypto:research/prime_leads_chase_20261113/L2-lattice/REPORT.md`):
  prime field, desk computation, KILL. Not binary.
- research-kb and the lead atlas are unavailable in this container: no novelty claim.

**Residual question.** In F_{2^n} = F_2[t]/(f) with factor base {x : deg_t x < l}, measure
(a) the existence threshold of 2-term decompositions of a random R, (b) the largest l at which an
exact (weak-Popov-reduced) Coppersmith lattice on S_3(X1, X2, x(R)) mod f^k recovers a planted
decomposition, (c) the gap between them, and (d) whether Koblitz structure (Frobenius conjugate
equations; Frobenius-orbit factor base) or the symmetry/coefficient structure of S_3 moves (b).

## 1. Pilot disclosure

Before freezing, a pilot with seed 999 (1–2 trials, n = 61 and 131, arms s3p/genp/gsp) was run to
debug the instrument. It showed S_3 recovering above the floor(det/N) guarantee and above the
generic null, with the symmetric generic null matching S_3. Those pilot rows are not data; they
informed the addition of the arms `gsp/gsu` and the high-k probe below.

## 2. Instruments and arms

Curve y^2 + xy = x^3 + 1 (ECC2K-130 family, a = 0) over F_2[t]/(f); f = lowest-weight
irreducible (trinomial t^n+t^k+1 with k <= n/2, else pentanomial); n = 131 uses
t^131 + t^13 + t^2 + t + 1 (the Certicom polynomial basis). S_3 = (x1x2+x1x3+x2x3)^2 + x1x2x3 + 1.

Lattice (`ffcop lattice`): box basis, multiplicity k, monomials X1^a X2^b with a, b <= 2k,
row (a,b) = X1^{a-2s}X2^{b-2s} g^s f^{k-s}, s = min(a/2, b/2); column scaling t^{(l-1)(a+b)}
(t^{(l-1)(a+2b)} for weights (1,2)). Reduction: Mulders–Storjohann weak Popov (exact).
"Short" row: degree < kn; then h(x1,x2) ≡ 0 mod f^k and deg h(x1,x2) < kn imply h(x1,x2) = 0
in F_2[t] (exact, no sqrt(dim) factor). Extraction (does not use the planted root): reduce the
first <= 8 short rows mod an irreducible q of prime degree D > bounds, resultant in X2 by
evaluation/interpolation over F_{2^D}, roots by gcd with X^{2^D} - X and trace splitting, second
coordinate from the gcd of all short rows; candidates verified by arm polynomial ≡ 0 mod f.

Arms (all from the same per-trial seed, run interleaved in this order in each trial):

| arm | polynomial | unknown bounds | role |
|---|---|---|---|
| s3p | S_3(X1,X2,c), c = x(P1+P2), deg x(Pi) < l | (L, L) | test arm (planted) |
| s3u | S_3(X1,X2,c), c = x(random point) | (L, L) | negative control (no planted root) |
| genp | X1^2X2^2 + r1X1^2 + r2X2^2 + r3X1X2 + r4, r1..r3 random, r4 planted at (x1,x2) | (L, L) | generic null (same support) |
| genu | same, r4 random | (L, L) | generic negative |
| gsp | r1 = r2 random (X1<->X2 symmetry), r3 random, r4 planted | (L, L) | symmetry ablation |
| gsu | same, r4 random | (L, L) | symmetry negative |
| e3p | e2^2 + c^2 e1^2 + c e2 + 1 in (e1,e2) = (x1+x2, x1x2); set a + 2b <= 4k | (L, 2L) | symmetric-variable arm |
| e3u | same, c random | (L, 2L) | negative |
| k2p | S_3(X1, Y, c), Y = x(φ Q2) = x(Q2)^2, c = x(P1 + φ Q2), deg x(P1), deg x(Q2) < l | (L, 2L) | Koblitz orbit factor base, Frobenius offset d = 1 |

L = l - 1. Univariate control (`ffcop uni`): monic random p of degree d in {2,3} with planted
root x0, deg x0 < l; rows X^j p^i f^{k-i} (i<k, j<d) and p^k; here one short row is a
rigorous guarantee once floor(det/N) < kn.
Koblitz conjugate augmentation (`ffcop kobaug`): H(X1,X2) = S_3(X1^2, X2^2, c^2 mod f) (the
x-relation of φR = φP1 + φP2 in the original small unknowns); generating set = box basis plus
g^i H^j f^{k-i-j} X^m (j >= 1, inside the box); compare reduced row-degree profiles.
Existence (`ffcop exist`, u64 field, n <= 63): enumerate x1 = 1, 2, 3, … (ascending degree),
solve the quadratic for x2 by half-trace, keep valid x-coordinates; hist[l] = ordered solutions
with max(deg)+1 = l (exact up to lmax), l_min = smallest l with a solution.

## 3. Grid (seed 20261114; N = trials per cell)

- LAT: n in {13, 19, 29, 37, 53, 61, 131}; k in {1,2,3,4,5}; l in 2..=min(ceil(n/2)+1, 36),
  plus l = ceil(n/2) for n = 131; all nine arms; N = 10.
- HIGHK: n in {61, 131}; k in {6, 7, 8}; l in ceil(n/8)..=ceil(n/5); arms s3p, gsp, genp; N = 3.
- UNI: d in {2,3}; n in {29, 61, 131}; k in {1,2,3,4}; l in 2..=min(ceil(n/d)+1, 58); N = 5.
- KOBAUG: n in {29, 61, 131}; k in {2, 3}; l in {floor(n/8), floor(n/7), floor(n/6), floor(n/6)+1}; N = 3.
- EXIST: n = 13 (N=500), 19 (N=300), 29 (N=200), 37 (N=60), all with lmax = ceil(n/2)+3;
  n = 53 with lmax = 24, N = 10 (early-stop beyond lmax).
- A/A: the LAT command for n = 61, k = 3 is run twice; outputs must be byte-identical.

## 4. Metrics (counted, host-independent)

Per lattice: dim, det degree, sum of reduced row degrees (must equal det: reducer identity),
weak-Popov property, number of short rows, sorted row degrees, simple transformations and
64-bit word XORs of the reduction, extraction pairs/resultant-zero counts, candidates, genuine
roots, `recovered` (planted pair among genuine roots; **decision metric**), `rows_vanish`
(diagnostic, uses the root). Existence: l_min, ordered counts per l, x1 evaluations, inversions.
Wall time is not recorded as a metric.

Derived: l_rec(n,k,arm) = largest l with recovered >= 5/N; l_rec8 = largest l with >= 8/N;
l_ex(n) = median l_min; gap Δ(n) = l_ex(n) - max_k l_rec(n,k,s3p); in units of L = l - 1, the
ratio (l_ex - 1)/(l_rec - 1).

## 5. Predictions (pre-registered)

- P1 (existence): mean ordered count of decompositions with deg < l ≈ 2^{2l-n} (within a factor 2
  where the expected count >= 4); median l_min ∈ {ceil(n/2), ceil(n/2)+1}.
- P2 (lattice): s3p recovers 10/10 at every l <= l_det(k) (floor(det/N) < kn) for n >= 29,
  k >= 2 (positive control); max over k <= 8 of (l_rec - 1)/n <= 1/6 + 1/n at n = 131
  (owner estimate 2l < n/3 per pair, i.e. L/n < 1/6).
- P3: s3p = gsp (same l_rec ± 1); genp lower than s3p by >= 1 at n >= 61.
- P4: no genuine root on any unplanted arm (s3u, genu, gsu, e3u).
- P5: K1 profiles identical (conjugate equations add nothing); K2 l_rec(k2p) < l_rec(s3p).
- P6: univariate control recovers 5/5 at every l <= l_det_uni.

## 6. Decision rule

Validity (else INCONCLUSIVE): reducer identity and weak-Popov property hold in every lattice;
P2's positive control holds; P6 holds; A/A byte-identical; genuine roots on unplanted arms are
counted and listed (a genuine root there is a real decomposition, so it is not a failure; more
than 2 in total triggers an audit before any verdict).

Main verdict on the item (owner claim: "small-roots bound 2l < n/3 vs existence 2l ≈ n"):
- **NEGATIVE (gap confirmed, route closed at measured level)**: for every n >= 53, max over
  k <= 8 of (l_rec - 1)/n <= 1/4 and Δ(n) >= n/4.
- **IMPROVES (gap materially narrower than the estimate)**: some n >= 53 has s3p recovery >= 5/N
  at (l - 1)/n > 1/4.
- Reported alongside, not a verdict change: whether the measured reach exceeds the owner's 1/6
  (P2) and by how much.
Koblitz sub-verdicts: K1 NO-CHANGE if every profile is identical, IMPROVES if any augmented
lattice has >= 2 short rows where the plain one has < 2. K2 NEGATIVE if l_rec(k2p) < l_rec(s3p)
for every n >= 53 and k, NO-CHANGE if equal somewhere, IMPROVES if greater anywhere.

Falsifier of the main verdict: one n >= 53 cell with >= 5/N recoveries at L/n > 1/4.

## 7. Scope

Only the box (and one e-form) shift set; multiplicity <= 8; one modulus per n; planted roots
uniformly random in the factor base. A measured ceiling for these lattices, not a theorem about
all lattices. No e2e solve exists; the cost comparison with rho is derived (§5a band).
