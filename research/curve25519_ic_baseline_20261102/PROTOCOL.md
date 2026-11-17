# Curve25519-family index-calculus baseline — protocol

Frozen 2026-11-02 before any ecbench data was taken. Amendments, if any, are
appended below with their reason and never rewrite this text.

## Question

What does the lab's existing prime-field index calculus cost on curves that
carry Curve25519's structure, and how far is it from matched Pollard rho?
This is the **first baseline** for the Curve25519 family (AGENTS.md "Success
criterion": name the baseline before optimising). It is not an improvement
claim and not a security claim about Curve25519.

## Curves

Curve25519 is `y² = x³ + 486662x² + x` over `F_p`, `p = 2^255 − 19`. Its
facts are re-derived from first principles by
`cargo run --release --bin curve25519_family -- audit`
(output: `analysis/audit-255.json`). No index calculus runs at 255 bits, so
the measured curves are **miniatures built by Curve25519's own selection
rule** (`curve25519_family family`, `analysis/family.json`):

1. `p` = the largest prime below `2^k` with `p ≡ 5 (mod 8)` (as `2^255 − 19`).
2. Montgomery form `B = 1`, `A ≡ 2 (mod 4)`, `A ≥ 6` (integral `a24 = (A−2)/4`).
3. `A² − 4` a non-square, so `T₀ = (0,0)` is the only rational 2-torsion
   point and the 2-Sylow subgroup is cyclic `Z/8`, as for Curve25519.
4. `#E = 8r`, `#E' = 4r'` (twist), `r`, `r'` prime, `r > 2^(k−3)`.
5. `A` the smallest value meeting 1–4; base point the smallest `u ≥ 2` of
   order `r` (Curve25519: `u = 9`).

Point counts are by baby-step giant-step on curve and twist, unit-tested
against the Legendre-sum count; the Montgomery summation polynomial `S₃` is
checked on 200 random triples per curve and unit-tested to vanish exactly on
`x(P ± Q)`.

Sizes: `k ∈ {20, 24, 28, 32}` (`r` of 18, 22, 26, 30 bits). Each session
also carries a **cofactor-1 control**: `prime_search` at the bit length of
that miniature's `r`, so the rho reference costs the same on both curves
and any difference in IC ÷ rho isolates the cofactor (and Montgomery-model)
structure.

## Arms (identical on both curves of a session)

| arm | role | method |
|---|---|---|
| `rho-neg` | reference | `rho.negation` |
| `abs-a`, `abs-2a` | baseline | `ic.pipeline`, `prime-abscissa:size=N`, `mitm:negation_folded=1` |
| `proj-a`, `proj-2a` | baseline | `ic.pipeline`, `prime-projected-abscissa:size=N,scan_cap=16N`, `mitm:negation_folded=1` |
| `abs-2a-aa` | control (A/A) | copy of `abs-2a` |
| `rho-neg-aa` | control (A/A) | copy of `rho-neg` |

`N_a = round(0.63 · r^{1/3})`, the lab's validated prime-order optimum
(`docs/ic/LEADERBOARD.md`, "Prime-field baseline": 162, 407, 1024 at 24,
28, 32 bits) scaled to the subgroup. `mitm:negation_folded=1` is the
incumbent exact oracle that holdout selected at every size. All arms are
existing lab methods; nothing new is introduced in the pipeline.

Targets: 24 public targets per curve, 2 measured rounds, 1 warm-up,
alternating arm order, seeds in the specs. Specs are emitted by
`curve25519_family spec <k> 24 <seed>`, seed `2026110200 + k`.

## Unit, isolation, pricing

`S` = counted cost in GAE per `√r` (ecbench). Isolation **L0**, counted units
only: no wall-clock figure is claimed (AGENTS.md §12: levels gate wall time,
never counts). Every IC-vs-rho ratio is reported at the §5a band `w = 0, 0.1,
1` GAE per unpriced native operation (`ecbench band`); an order between two
arms is stated only where it holds across the band.

## Predictions (preregistered)

Mechanism. `prime-abscissa` points lie anywhere in `E(F_p) ≅ Z/8 × Z/r`, so
only 1/8 of their pair sums land in the order-`r` subgroup that holds the
targets: per-probe yield ≈ `2N²/(8r) = 2N²/p`. `prime-projected-abscissa`
multiplies each base point by the cofactor first, so every pair sum is a
subgroup point: yield ≈ `2N²/r`.

- **P1** (cofactor penalty). On the miniature, `abs-*` needs ≈ 8× the
  relation-phase trials of `proj-*` at equal `N`; on the cofactor-1 control
  the ratio is ≈ 1 (projection by `h = 1` is the identity).
- **P2** (control matches). Miniature `proj-*` IC ÷ rho is within a factor
  1.5 of control `abs-*` at the same `N`.
- **P3** (distance). Rho stays ahead at every size and weight (the lab's
  prime-order ratio is 7.7–33×); the ratio is reported as distance remaining.

Falsification: P1 fails if the miniature `abs/proj` trial ratio is below 4
or the control's is outside `[0.67, 1.5]`.

## Baseline definition (the ratchet's first entry)

Per size, the **Curve25519-family baseline** is the arm with the lowest mean
`S` at `w = 0` among `abs-a, abs-2a, proj-a, proj-2a` on the miniature, with
its `w = 0.1, 1` values. This is an in-sample choice among four existing
recipes and is disclosed as such; any later candidate must beat it on fresh
targets (a new `target_seed`) in a new frozen protocol.

## Acceptance

Every record verified; `ecbench verify --replay 12` audit passes on every
session; A/A ratio within its interval. A failed or timed-out arm is
reported, not dropped.

## Execution route and its limits

CLAUDE.md names Iso Lab on EKS as the default route and an S3 resume
checkpoint before runs. The resume checkpoint was pushed
(`s3://setec-astronomy-artifacts/checkpoints/crypto/claude__curve25519-index-calculus-pw5d6r/20261102T010052Z-1b3c6210ac90/`).
This container has no `aws` CLI and no Iso Lab identity, so the sessions run locally at L0 with counted units only (the same
route as `research/ic_residual_fused_20261031`, Amendment 1). A second-host
replay on isolab-eks is an open item before any leaderboard row.
