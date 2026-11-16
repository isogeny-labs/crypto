# Isogeny null at `P1`: reading P-ISO-N (preregistration Amendment 28 = branch 21; relaunch Amendment 29 = branch 22)

Derived by `iso/score_iso.py --null` from the committed isogeny tower medians (`results/ISO/SCORE/score_iso.json`) and the
manifest-checked null rows in `final/runs/` (pod `zbt7oq1a34tqr0`, attempt 2, done exit 0, no OOM kill). Every number is
`[pcc:unverified]`.

## Gates

- G1 pass. G-AUX on the pod: "pass 16 draws, 0 mismatched".
- **Auxiliary curves.** Every null cell `t = 1..13` uses exactly the same `(aux_a, aux_b)` as the tower arm's cell: the cell
  seed does not depend on the control. So the 14 independent recounts of the tower arm cover the null too.
- Scorer: no conflicts, exit 0.

## Medians of `d_F` (lower median of 2 draws)

| `t` | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 |
|:--|--|--|--|--|--|--|--|--|--|--|--|--|--|
| isogeny tower (Semaev `S3` + Vélu tower) | 3 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 6 | 6 | 6 | ≥6 |
| isogeny null (random `S3` coefficients + the same Vélu tower) | 3 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 6 | 6 | 6 | ≥6 |

## Reading

**P-ISO-N: shape-driven.** 12 values of `t` have both medians exact and equal; none differs. The isogeny ladder's growth
(P-ISO-2 "grows", 5 → 6 at `t = 10`) is reproduced by a system that keeps the Vélu tower equations and replaces Semaev's `S3`
by random coefficients on the same support. At this resolution the solving degree is set by the tower ring and the shape
of the system, not by the summation polynomial. That matches the Kummer findings: the Kummer null read the same as the Kummer
tower at `m = 2` (stage A, on completed `N`) and at `m = 3` (stage B).

This is a solving-degree description, not an index-calculus cost or improvement claim.
