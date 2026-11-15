# SIBLING-POINTS Stage 2: prime-field curve shapes at the engine's largest size

Status: frozen before any data of this stage (2026-11-11). Every number below is an expectation or
a prior measurement, labelled `[pcc:unverified]`.

## Question

The owner's target is ECDLP over prime fields. Stage 1 (`../stage1-eks/README.md`, PR #3853) measured
sibling points on one family of prime-field curves: the five SNC-panel curves, which have cofactor 8.
At k = 32 (r = 2^29), sib8-p256 / tw2cj-lazy was 0.488 / 0.578 / 0.984 at w = 0 / 0.1 / 1, and
sib8-p256 / rho-neg in bit-ops was 0.982 [0.911, 1.063] [pcc:unverified].

Stage 2 asks two things:
- Does the gain carry over to prime-order prime-field curves of the standard shapes?
  - a = -3, the NIST P-curve shape.
  - random a, b.
  - j = 0 with b = 7, the secp256k1 shape. Here the matched rho uses the order-6 automorphism.
- Does a 512-point pool help at the largest r the engine accepts? `wwalk sib` needs r < 2^31;
  `ecbench plan` refuses r = 2^33.5 and above for the sibling arms, and refuses `lazy=1, tw=2` at
  r = 2^37.5.

## Design

- Specs `spec-a3.json`, `spec-gen.json`, `spec-j0b7.json`, one per shape. Each uses five
  prime-order curves from `prime_orbit_shape` with 31-bit p, giving r between 2^30.1 and 2^31.
- 8 public targets per curve, 6 rounds plus 1 warmup, alternating order, L0.
- The panel runs locally on this session's host: the container's AWS keys are still rejected by
  STS, and the Iso Lab GitHub route failed before any pod ran in Stage 1.
- Arms:
  - `rho-neg`, the reference.
  - `rho-neg-aa`, the A/A control.
  - `rho-cubic`, j0b7 only: `rho.cubic`, A = 6, the matched rho for that shape.
  - `tw2cj-lazy`, the baseline.
  - `sib8-p64`, `sib8-p256`, `sib8-p512`.
  - Every IC arm uses factor base `prime-projected-abscissa:size=2,scan_cap=32`; the oracle strings
    are in the specs.
  - A development smoke at 34 to 38 bits (scratchpad, not data) is what found the size limits.
    No arm of these specs was run before the freeze.
- Analysis, as in Stage 1:
  - `ecbench verify --replay 8`.
  - `ecbench compare` (paired bootstrap 95 percent interval) at w = 0, 0.1 and 1.
  - `ecbench band --ranks`.
  - `stats_bitops.py` (Stage 1's script, same statistic) for bit-op ratios.

## Pre-registered expectations [pcc:unverified]

- **F1.** sib8-p256 / tw2cj-lazy, GAE, w = 0: at most 0.56 on each of a3 and gen. The order
  "sib8-p256 lower at every weight" holds pooled on at least one of a3 and gen.
- **F2.** sib8-p512 / sib8-p256, GAE, w = 0: below 1 on each shape. The set-up is a smaller share
  at r = 2^30.5 than at k = 20, where p256 cost more than p64.
- **F3.** sib8-p256 / rho-neg, bit-ops: the interval contains 1 or lies below it on a3 and gen.
  The band order against rho-neg stays unresolved.
- **F4.** j0b7: rho-cubic / rho-neg about 0.58, from the sqrt(3) gain of A = 6 over A = 2. Every
  IC arm is above rho-cubic at every weight. This is the distance remaining against the matched rho
  for the secp256k1 shape.

## Admission

Each item is checked per spec. A failure voids that spec's comparisons; it is reported, not re-run
under the same name.
- Every record verified.
- `verify --replay 8` reproduces.
- The A/A ratio is 1.

## Not claimed

- No leaderboard row and no `benchmarks/README.md` row: that needs the validation gate.
- No security claim about P-256, secp256k1 or any standard curve. These are 31-bit curves of the
  same shape.
