# Protocol: binary composite IC with the tuned S4 root finder, against matched rho

Study `bcr-20261126/v1`. Frozen before any evaluation data. The machine-readable
form, with every seed stream, is [`protocol.json`](protocol.json). Its SHA-256
is recorded in [`SEEDS.sha256`](SEEDS.sha256) and in the commit that freezes it.
Seed policy: `isogeny-labs/experiment-seeds/v1`. Status: `[pcc:unverified]`, L0,
one shared 4-vCPU cloud container.

## Question

The workspace goal is "Binary composite index calculus to Pollard-rho parity".
The new ingredient comes from workspace PRs #106 and #139
(`docs/notes/summation-rootfind-20261124/REPORT_1C.md`): the factored S4 root
finder, with S4 = S3(u, x_R, X)·S3(u′, x_R, X), two Artin–Schreier steps per
pair, a pair table and batched inversions. Per quartic it costs 0.07–0.22× the
subspace gcd.

We put it in the relation step of the binary-composite IC and ask:

1. Does it change the single-target cold cost of the IC arm, measured against
   matched negation-map rho on the same curves and the same public targets?
2. How far is that cost from parity at each rung?

## Instances

- **Field.** `F_{2^n}` with n = 3·n′, n′ = 6..11 (n = 18, 21, 24, 27, 30, 33).
  The modulus is the lowest-weight, lexicographically first irreducible
  trinomial or pentanomial.
- **Curve.** `y² + xy = x³ + ax² + b`. `b` is uniform with `b` in no proper
  subfield, and `a ∈ {0,1}` is uniform. The group order is `h·ℓ` with
  `h ∈ {2,4}` and `ℓ` prime, and the generator is `G = [h]P`. Order counting
  uses BSGS in the Hasse interval on one point, then a second point is checked.
  Neither step is charged to any arm, because both are public setup.
- **Panel size.** Four curves per rung and three targets `Q = [d]G` per curve.
  At n = 33 the panel is 2 × 2. Each target gets 16 rho seeds.
- **Curve names.** Curves are identified by their parameters in
  `data/curves.jsonl`. They are not ICV1-registered (see REPORT.md, "Gaps").
- **Factor base.** The subfield `V = F_{2^{n′}}` is computed as the kernel of
  `x ↦ x^{2^{n′}} + x`. The base is every `x ∈ V` that lifts to a rational
  point, one point per ± class, together with `T = (0, √b)`. `T` projects to
  `O` in the ℓ-part and has no column.

## Arms

All arms use the same field arithmetic: crypto `semaev_decomp::Gf2`, included
unchanged, with the `binary_radical_root::ArtinSchreier` tables. All arms go
through one counted wrapper, and all use Montgomery batch inversion wherever a
row or a set of lanes allows it. Every IC arm consumes the same ordered
relation-target stream. That stream is an 8-lane additive walk
`R ← R + J_{h(x(R))}` with 16 jumps, batched, and each target is
`R = [a]G + [b]Q`.

| arm | relation step |
|---|---|
| **G** (before) | m = 3 pairs-and-solve over base pairs, with the last summand found by the subspace gcd `gcd(f, L_V mod f)`. Leading-coefficient inversions are batched per row. This is a counted port of crypto `SubspaceOracle`. |
| **F** (after: the tuned root finder) | m = 3 pairs-and-solve, with the last summand found by the factored route on the target-independent pair table of `u = x(P₁ ± P₂)`. Stage-B inversions are batched per row. This is a counted port of crypto `FactoredSubspaceOracle` plus REPORT_1C's pair table. |
| F0 | F without the table: stage A is recomputed per target. Its cost is derived exactly as F + (targets − 1)·(table cost). |
| M3 | m = 3 meet-in-the-middle. The same `u` table is hashed, then probed with `x(R ± P₃)`, which costs one AS step per `P₃`. |
| M2 | m = 2 meet-in-the-middle: probe `x(R)` in the `u` table. |
| **RHO** | Negation-map r-adding rho, shaped like crypto `ic_boundary::RhoWalk::negation()`: jump count and coefficient bits as in `rho_jumps_for`, DP bits `⌊bits(ℓ)/4⌋`, the Wiener–Zuccherato look-ahead, a 16-point cycle window with escape by doubling, and a cap of `20·2^dp + 64`. It runs W lanes with one batched inversion per round. W was tuned per rung on held-out pilot curves (`data/pilot_rho.jsonl`). |

**Stopping and linear algebra (all IC arms).**

- After every target, every x-triple (or pair) the oracle returns is resolved
  into signs by the group law. This is the relation's replay check: the sum
  must equal ±R exactly.
- Once rows ≥ distinct columns + 4, Wiedemann (Berlekamp–Massey) mod ℓ finds a
  dependency `c`. The log is then `d = −Σc·a / Σc·b`.
- Each failure demands 4 more rows.
- The solve ends with `[d]G = Q`, which is counted. The evaluator then compares
  `d` with `d_true`.

**Cross-checks.** G runs on every target at n ≤ 24, and on the first 256, 128
and 64 targets at n = 27, 30 and 33. On every target it runs, its triple set
must equal F's (mismatches are counted). Where it does not run, G's root-find
cost is its measured per-target mean times the target count, flagged
`gcd_exact = false`. Everything except the root finder is identical between
F and G by construction: the same stream, and therefore the same relations.

## Cost unit

The primary unit is **Meq**, multiplication equivalents:

- 1 per field multiplication;
- 1 per squaring;
- per inversion, its Itoh–Tsujii census: `(n−1)` squarings plus
  `⌊log₂(n−1)⌋ + popcount(n−1) − 1` multiplications;
- 1 per F₂-linear table map (trace, AS solve, square root, membership);
- 1 per hash probe or insert;
- 1 per product or addition mod ℓ.

Band (AGENTS.md §5a):

- the non-field native units are also priced at 0 and 0.1;
- squarings (and the squarings inside inversions) are also priced at 0.25;
- an order between arms is stated only where it holds across the band.

Wall time is recorded and is indicative only, because the host is shared.

## Analysis (frozen)

- **Paired ratio.** For each case (n, curve, target), the arm's cost divided by
  the mean of the 16 rho runs on the same Q.
- **Per rung.** The geometric mean of the paired ratios, with a 95% CI from a
  hierarchical bootstrap: resample curves, then targets within each curve, then
  rho seeds within each target. B = 10000, percentile intervals, seed stream
  `boot`.
- **Root-finder effect.** Per rung, the geometric means of F/G and F/F0 over
  the same cases, with the same bootstrap.
- **Exponent.** The OLS slope of log₂ cost on log₂ ℓ per arm, the difference
  from rho's slope, and a bootstrap CI.
- **Gap.** Reported as the distance remaining (the ratio and its CI), never as
  pass or fail.

## Why pairs-and-solve cannot reach parity (stated before the data)

Let the base have F classes and let the group order be N. A target decomposes
with probability about `C(F,3)·2³/N`. The arm needs about F relations, so it
consumes `T ≈ F·N/(F³·4/3) = 3N/(4F²)` targets. Each target costs `F²/2`
root finds. The total is about `(3/8)·N·c_rf`, for any root finder and any F.

So the root finder can move only the constant `c_rf`, and the gap to rho grows
like N^{1/2}. The MITM arms replace the per-target `F²` loop with `F` probes
(M3) or one probe (M2). They cost `Θ(N^{2/3})` at `F ≈ N^{1/3}`, which is the
subfield size at k = 3, and are still above rho asymptotically.

The run measures where each arm actually sits, against H1–H3 in
`protocol.json`.

## Not tested

- **Koblitz rungs.** The subfield base degenerates there (`protocol.json`,
  "untested").
- **GHS descent.**
- **n > 33.**
