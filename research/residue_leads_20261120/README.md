# Residue and character leads on Q = kG (R1–R4)

The full report is `REPORT.pdf` (source `REPORT.typ`). Also in this directory: the frozen protocol
(`PROTOCOL.md`, `protocol.json`, `frozen.sha256`), the amendments (`AMENDMENTS.md`; 1 and 2 were made before the
data they affect, 3 after it), the sources checked (`SOURCES.md`), the harness (`native/`, Rust, no dependencies),
the results (`results/`) and the run report (`reports/run-20261120T2050Z/`).

| lead | outcome |
|:--|:--|
| R1 homomorphic residues | Theorem: constant on a prime-order ⟨G⟩. Exact check: 0 violations on 9 binary and 3 prime curves; positive controls non-constant on E. |
| R2 residue predicate vs k | 0 of 729 tests significant. Characters attain max\|S\| = 2.000√q (Weil, never beaten). Heuristic bias for ECC2K-130 ≈ 2^-62. |
| R3 Semaev pre-filter | No useful pre-filter: prime lift ≤ 1.042; no linear functional of x(R) beyond the trace (WHT max \|z\| 5.13, p ≥ 1.2e-3). |
| R4 rho classes | Theorem s ≥ 2 log(√n−1)/log d, held on 20/20 D=−7 curves. Koblitz τ is extremal (s = m). No new cheap classes. |

Not a speed result: no leaderboard or benchmark row.
