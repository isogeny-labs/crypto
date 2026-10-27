# PROTOCOL — twist obstruction for summation-polynomial relations

Frozen 2026-10-26, before the `full` run. Harness: `src/main.rs`
(sha256 `b36e973d29e1c2afed16e66593d7030edee6a1f9b82588ea0974cc2ae389c4b4`),
`Cargo.toml` (sha256 `762f290c4259937c8274d3aba31247e33a222501503df50855aeed51bb71ef6d`),
seed `0x7715720261026` (`0x7715_7_2026_1026` in `main`).
Command: `cargo run --release -- full reports/run-<UTC>/`.

## 1. Question and the claims each outcome licenses

The setup: E_a : y² + xy = x³ + a x² + b over F_q with q = 2^n and n odd.
Its quadratic twist is E_{a+1}, and φ(x, y) = (x, y + s x) with s² + s = 1
is an x-preserving isomorphism E_{a+1} → E_a over F_{q²}. This is the
ECC2K-130 shape: n = 131, a = 0, b = 1.

The argument under test says three things:

- **(T1) Frobenius separation.** σ acts as −1 on φ(E′(F_q)). So if
  R = A + B, with R and A in E(F_q) and B a sum of embedded twist points,
  then 2B = 0.
- **(T2) Factor-base consequence.** Take any decomposition
  R = Q₁ + … + Q_m with every x(Qᵢ) ∈ V ⊂ F_q. Its off-parity sub-sum (the
  Qᵢ whose σ-parity is opposite to R's) lies in {O, T}, where T = (0, √b).
  For m = 2 no off-parity solution exists. For m = 3 the off-parity
  solutions are exactly:
  - (x_R, x, x), when x_R ∈ V;
  - (x(R+T), x, x(P_x+T)), when x(R+T) ∈ V,

  with x ranging over the off-parity elements of V.
- **(T3) Shared geometry.** The x-solution set of the decomposition problem
  is the same on E_a and on its twist E_{a+1}. Also, S₃ is independent of a.

Class (controlled-experiment §1): T1–T3 are theorems. This run is an
**exact computation** that checks them on every instance enumerated, and it
also checks the instrument. It licenses these statements only:
- "zero violations in N exact checks" for the stated census;
- "the observed off-parity set equals the predicted set on every target";
- "the x-sets are identical on every target".

A single violation or mismatch means either the argument is false in this
model or the harness has a bug. Either way it is reported, and nothing is
claimed until it is resolved.

The comparison against the independent-lift heuristic (§4) is a
**heuristic reference**, not a control. It is reported as a distance.

## 2. Populations

Prime n ∈ {7, 11, 13} for the census and n ∈ {11, 13, 17, 19} for the
decompositions. Fields use fixed irreducible polynomials, checked in code.

Curves per n:
- `koblitz_a0_b1`, the ECC2K-130 shape;
- four random (a, b), with b ≠ 0, drawn from the seed;
- for arm C2, constructed curves (below).

The factor base is V = span_F₂(1, t, …, t^{l−1}), the standard low-degree
choice. Sizes:

| n  | l (m = 2) | l (m = 3) | targets m=2 / m=3 per curve | planted (C1 per curve, C2 per n) | m=3 S₄ checks per curve |
|----|-----------|-----------|-----------------------------|----------------------------------|-------------------------|
| 11 | 7         | 6         | 240 / 60                    | 10                               | 10                      |
| 13 | 8         | 7         | 240 / 60                    | 10                               | 6                       |
| 17 | 10        | 8         | 160 / 40                    | 8                                | 3                       |
| 19 | 11        | 9         | 120 / 30                    | 6                                | 2                       |

Every m = 2 target gets the S₃ polynomial check.

## 3. Arms

- **census (T1).** Exhaustive over every P ∈ E(F_q) and every
  Q ∈ φ(E′(F_q)), for 5 curves each at n = 7, 11, 13.
  - Count P + Q fixed by σ, split by whether Q ∈ {O, T}.
  - Also check that |σ = −1 part| equals #E′(F_q), counted independently
    on the twist model over F_q.
  - Also check that φ maps E′(F_q) onto the σ = −1 part.
- **A_random (T2, T3).** R is a random point of E(F_q). Enumerate every
  solution by point arithmetic, on E_a and again on E_{a+1} with the same
  x(R). Then classify each solution and compare:
  - the x-sets on the two models;
  - the point-enumerated x-set against the roots of the explicit S₃ (all
    m = 2 targets) or of the explicit S₄ = Res_X(S₃, S₃) (a subsample of
    the m = 3 targets).
- **Positive control C1 (detector power).** m = 3, with x_R planted in V.
  The detector must report every predicted (x_R, x, x) solution. On the
  twist model, every coordinate of a pure solution has σ-parity −1, so the
  same detector must report twist lifts there.
- **Positive control C2.** Curves are built with √b = v₁v₂, where v₁ and v₂
  are off-parity elements of V, and the target is chosen with x(R+T) ∈ V.
  The detector must report the predicted off_T solutions.

The instrument is identical across arms: the same enumeration, the same
classifier, and the same predicted-set builder. The predicted set is built
from the target and V only, never from the observed solutions.

## 4. Metrics and decision rule

Recorded per target and per model:
- solutions, split into pure, pure_with_2tor, off_O, off_T, violation;
- witness_fail (signed points do not sum to R);
- |obs_off|, |pred_off|, |obs∖pred|, |pred∖obs|;
- xsets_equal;
- the S₃/S₄ set differences;
- p_E, the measured fraction of nonzero x ∈ V that lift to E(F_q).

Decision:
- T1, T2 and T3 are **consistent with the data** iff all of these are zero
  across the whole run: violation, witness_fail, census non-torsion
  rational, census torsion irrational, census count mismatches, both
  obs∖pred and pred∖obs, and xset mismatches.
- If the explicit-polynomial checks differ from the point sets, each
  difference is reported and classified. A known degenerate case is the
  formal resultant of S₄ when both leading coefficients vanish
  (x₁ = x₂ and x₃ = x_R). An explained degeneracy is not a violation. An
  unexplained difference is a mismatch.
- The positive controls must be detected: C1 needs a non-empty predicted
  set equal to the observed set on every target, and C2 needs off_T > 0 in
  aggregate. If either fails, the run is **invalid**, not "no effect".
- Heuristic reference: if lift types were independent of the solution
  condition, the fraction of solutions with at least one off-parity
  coordinate would be 1 − p_E^m. The observed fraction is reported beside
  it, with a 95 % Clopper–Pearson interval over arm A, per (n, m).

Power: with N solutions per family and zero violations, the 95 % upper bound
on the violation rate is about 3/N. The plan gives N of roughly 4·10³
(m = 2) and 3·10⁴ (m = 3), against a heuristic reference of about 0.3–0.9.

## 5. Runtime pre-flight

Measured with the `probe` mode on this container (4 vCPU, unisolated cloud
VM, single thread):
- census at n = 13: 3.1 s per curve;
- m = 3 at n = 19, l = 9: 0.50 s per target per model;
- one S₄ check at n = 19, l = 9: about 4 s.

Estimate for `full`: about 4–6 minutes single-threaded. Wall time is not a
result of this study.

## 6. What does not count

Changing n, l, the targets, the classifier, or the decision rule after
seeing `full` output requires a new, labelled protocol version, and both
versions are reported. The `dry` and `probe` outputs are scratch and not
reported. One fact the dry run revealed: the Koblitz p_E on the low-degree
V is far from ½ (9 of 31 at n = 7, l = 5). This is why the heuristic
reference uses the measured p_E.

## Amendment 1 (2026-10-26, after the run): formatting only

This amendment changes no experiment parameter and no decision rule.

`src/main.rs` was reformatted with `rustfmt --edition 2021`, as the
repository's "rustfmt (changed files)" CI check requires. The new sha256 is
`6bc1ae3d5e849785ead81492816f632a89b03e04422f1c528e9a370f0c3909f4`. The
reported run, `run-20261026T1456Z`, used the frozen source above (commit
`ff3bf9d48`).

The equivalence is checked as follows. The reformatted harness was rerun in
`full` mode. Its `rows.jsonl` with the `seconds*` fields removed is
byte-identical to the reported run's (sha256 `7686fa84…556f47` for both).
Its `summary.json`, aside from `run_dir`, is also identical
(`3d93afd8…aeb43c`). The summarizer had one clippy cast removed.
