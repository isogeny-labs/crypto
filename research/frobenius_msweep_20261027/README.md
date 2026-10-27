# Frobenius search-symmetry m-sweep (audit + prepared cells, not run)

Report: [`REPORT.pdf`](REPORT.pdf) (source [`REPORT.typ`](REPORT.typ)); coverage
figure [`coverage.svg`](coverage.svg) drawn by `plot_coverage.py` from
[`coverage.json`](coverage.json); full audit output [`audit_20261026.json`](audit_20261026.json).

**Status: prepared, not run.** No experiment ran because the repository's S3
resume-checkpoint gate needs `CHECKPOINT_UPLOAD_TOKEN`, which this container lacks.

Audit of LEAD-ICB-frobenius-search-symmetry:
- C1, C2 and C4 are proved for all m, but were measured only at m = 2 or 3.
- C3 ("orbit encodings ≥ 0.95× plain") is unsupported: its pre-registered
  census never ran.
- C5 was measured only on the m = 2 barrel.
- No Frobenius-symmetry solver experiment exists at m ≥ 3.
- The m = 2 orbit-union cost was priced against n^(m−1) systems instead of
  D(n,m) = C(n+m−1,m)/n.
- The 2-torsion translation x ↦ √b/x is a genuine stabiliser of order
  2^(m−1), but only on non-subspace ι-stable bases.

| cell | question | m | tests |
|:--|:--|:--|--:|
| [G1](g1-chain-sat-orbit/) | chained-S3 SAT, plain vs Frobenius-orbit domain (C3 grade) | 4–6 | 25 |
| [G2](g2-orbit-union-Dnm/) | orbit-union yield by shift-pattern class, priced at D(n,m) | 2–5 | 30 |
| [G3](g3-c3-census/) | the never-run C3 census (H-BINSTD-c72284 Stage 2) | 2–3 | 38 |
| [G4](g4-iota-stable/) | ι-stable non-subspace bases, (Z/2)^(m−1) stabiliser | 2–6 | 24 |
| [G5](g5-propT-census/) | Proposition T / Σ and sporadic-overlap census | 2–8 | 31 |
| [G6](g6-large-m/) | exact census at large m: normal-basis interleaved vs size-matched partitions; ι quotient | 9–20 | 32 |

Each cell went through two independent review rounds and three repair
passes. Run it from its `RUN_CARD.md` in a session that has the checkpoint
token.

G6 derived finding: normal-basis interleaved slots, the design of the n = 83, m = 10 gate, are Frobenius rotations of each other. A digit model predicts about 18× fewer distinct sums than a size-matched generic partition at n = 23, m = 12. See [REPORT.pdf](REPORT.pdf).

Companion notes: [lift-check audit](../ic_liftcheck_audit_20261027/) (lead 1, closed) and [p_E closure](../pE_density_closure_20261027/) (lead 2).
