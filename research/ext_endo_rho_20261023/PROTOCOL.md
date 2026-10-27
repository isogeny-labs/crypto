# Protocol: endomorphism-orbit rho on odd-characteristic extension-field curves

2026-10-23. Frozen before any data. Item 1 of the extension-field programme
(the strongest-available rho reference for curves over `F_{p²}` and
`F_{p³}`; AGENTS.md validation gate, "the strongest available for that
curve (negation, Frobenius or the automorphism fold)").

## Question

Does ecbench now carry a rho reference that uses the full symmetry of two
odd-characteristic extension-field families, and does it achieve the
predicted step reduction over negation rho?

- **subfield_fp3**: `E / F_p` (`j ≠ 0`) on `E(F_{p³})`, the prime subgroup
  outside `E(F_p)`. Frobenius `π` has order 3 on it, so the classes
  `{±π^i P}` have `A = 6`. Method `rho.frobenius_orbit`.
- **gls_fp2**: the GLS twist over `F_{p²}` with `ψ`, `ψ² = −1` on the
  subgroup, so `{ψ^i P}` has `A = 4`. Method `rho.gls`.

This is infrastructure for a matched reference. It is not an
index-calculus result and does not move the IC baseline.

## Instrument

`ecbench` at the commit that freezes this file. Specs:
[`specs/SPEC-subfield-fp3.json`](specs/SPEC-subfield-fp3.json) and
[`specs/SPEC-gls-fp2.json`](specs/SPEC-gls-fp2.json). Every arm is the
library's tuned walk `ic_boundary::rho_walk_with` with
`RhoWalk::negation()` (look-ahead, cycle escape, distinguished points),
differing only in its classes. Unit: counted group operations (`S` =
operations / `√r`).

- Workloads: `p_bits ∈ {14, 16, 18}`, seeds `{1, 2, 3}`, 8 planted targets
  per curve, 72 solves per arm per family.
- Arms: `rho-plain` (positive control, `A = 1`), `rho-neg` (reference,
  `A = 2`), `rho-neg-aa` (A/A control: the same method under its own arm
  identity), and the candidate.
- Audit: `ecbench verify --replay 16` on each session.

## Predictions and decision rules (numeric, fixed now)

Ratios are of mean `S` over the 72 paired solves (same curve, target and
algorithm seed), with a 95% paired bootstrap interval (`ecbench compare`).

| check | statistic | accept when |
|---|---|---|
| A/A | `S(rho-neg-aa) / S(rho-neg)` | interval contains 1 **and** lies inside [0.80, 1.25] |
| positive control | `S(rho-plain) / S(rho-neg)` | point estimate in [1.15, 1.75] (theory √2 = 1.414) |
| correctness | every solve of every arm | `verified`, and the audit reports 0 problems |
| candidate, subfield | `S(rho-frob) / S(rho-neg)` | interval upper end < 1 **and** point estimate in [0.45, 0.72] (theory 1/√3 = 0.577) |
| candidate, GLS | `S(rho-gls) / S(rho-neg)` | interval upper end < 1 **and** point estimate in [0.57, 0.85] (theory 1/√2 = 0.707) |

If a control fails, the candidate's ratio is reported as withheld, not as
a result. If a candidate's interval reaches 1, the method is reported as
not yet shown to help at these sizes.

## §5a band

Unpriced native counters on every arm: `canonicalisations_uncharged` (all
folded walks, negation included) and `endomorphism_applications_uncharged`
(candidates only: 2 per step for Frobenius, 3 for `ψ`). Each is priced at
0, 0.1 and 1 GAE per operation, the same way on every arm. A ranking of
candidate against reference stands only where it holds at all three
weights.

## What would count as a failure

- the candidate is not faster than negation rho at the 95% level;
- its ratio lies outside the windows above (a walk that is too good is a
  bug in the accounting, a walk that is too slow is a bug in the classes);
- any unverified solve.

---

## Amendment A1 (2026-10-23, after the first sessions; the text above is unchanged)

The first sessions (`sessions/subfield-fp3`, `sessions/gls-fp2`, card
`CARD-ext-endo-rho-20261023-2`) are kept as recorded. Reading them showed
two accounting gaps and one control-design flaw, and the study is re-run
in `sessions-a1/` with them fixed:

1. **GLS canonicalisation applied `ψ` three times per step.** Since
   `ψ² = −1`, the class `{P, ψP, −P, −ψP}` needs one application; the other
   members are negations. `EndomorphismClasses` now applies `φ` only up to
   the first power with `λ^h = −1`. The representatives, multipliers and
   therefore the walk and every group-operation count are unchanged; only
   `endomorphism_applications_uncharged` changes (3 → 1 per step on GLS,
   2 per step on the subfield family, unchanged).
2. **No field-level price.** `ExtCurve` now tallies every `F_p`
   multiplication and inversion of its group law into the ledger
   (`counts_field_ops`), and the classes report the multiplications of
   their endomorphism applications as `endomorphism_field_muls_uncharged`.
   This adds a calibrated point (total `F_p` multiplications, inversions
   counted beside them) next to the §5a band; it does not replace the band.
3. **The A/A arm was degenerate.** `rho-neg-aa` is the same method with the
   same algorithm seed, so it reproduced `rho-neg` exactly (ratio 1.0000):
   a determinism check, not a sampling-noise control. Its acceptance rule
   is met as written, but it is reported as a determinism check only. The
   sampling noise of the ratios is carried by their paired bootstrap
   intervals over 72 workloads.

Specs, curves, targets, seeds and decision rules are unchanged. A1 is
committed before the `sessions-a1/` data exist.

### A1 execution note (appended after the A1 runs started)

The container was reclaimed at about 07:35-08:18 UTC while the agent session was idle; every process died (uptime reset), the disk survived. Card `CARD-ext-endo-rho-20261023-4` completed `sessions-a1/subfield-fp3` (session, audit, compares, table, band) before the interruption and never started `gls-fp2`; it wrote no receipt. A new card runs `sessions-a1/gls-fp2` alone with the same spec and binary. The completed subfield session is kept; its audit is in `sessions-a1/subfield-fp3.audit.json`.
