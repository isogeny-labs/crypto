# Grevlex solving degree of the normalized two-summand Kummer chain systems over F_p

Study of 2026-11-15. Everything here is `[pcc:unverified]`; Lean theorems are kernel-checked on the
standard foundations; computed values carry replay certificates verified by an independent checker
where the report says so.

| file | content |
|---|---|
| [`REPORT.md`](REPORT.md) | the full report: pinned definitions, Theorems 1–7 with proofs, the admissible family, finite-depth certificates, the open linear lower bound and its missing lemma, proof map, reproduction |
| [`reports/report.pdf`](reports/report.pdf) | rendered summary (Typst) with the two graphs; source `reports/report.typ` |
| `reports/lfd_vs_k.svg`, `reports/dims_vs_k.svg` (+ PNG) | graphs of recorded values (`reports/values.json`, `reports/plot_lfd.py`) |
| `reports/pi_vs_box.svg`, `reports/lfd_universality.svg` (+ PNG) | §13 graphs (`reports/cofactor_data.json`, `reports/plot_cofactor.py`) |
| [`reports/evaltest.md`](reports/evaltest.md) | counterexample record: boundary-evaluation functionals are not the annihilator of the closure |
| `lean/` | Lean 4.34.1 / Mathlib v4.34.1 project: `ChainSd/Closure.lean` (abstract degree-bounded closure, transfers, `sd = ref = d_F`), `ChainSd/Unit.lean` (unitness of the family), `ChainSd/Popcount.lean`, `ChainSd/GroupAlgebra.lean` (the chain algebra, depth monotonicity), `ChainSd/Reach.lean` (reachability; Theorem 2 on the concrete system), `ChainSd/LowerThree.lean`, `ChainSd/Cofactor.lean` (Theorem A), `ChainSd/Axioms.lean` (axiom audit) |
| `native/` | Rust engine `chainsd` (closure, unit test, Z-model, evaluation test, certificates) and the independent C checker `native/checker/chaincheck.c` |
| [`certs/`](certs/README.md) | replay certificates (`chainsd-cert/v1`), their verdict JSONs, `SHA256SUMS`, regeneration script |

Headline: `4 ≤ LFD(k) ≤ 2k + 1` for every admissible unit instance (`k ≥ 2`, `A ≠ −3t²`); [pcc:unverified]
exact `LFD(k) = 4, 4, 4, 4, 5, 5, 5, 5` at `k = 2..9` with verified upper and lower certificates;
`LFD(k) ≥ 6` for `10 ≤ k ≤ 15` on the admissible instance `p = 65537, n = 3` by a certified
depth-10 closure (verified by the independent checker) plus the Lean-proved depth monotonicity; the linear lower bound `ck − C` remains open [pcc:unverified]. The 2026-11-22 follow-up (REPORT §13) reduces it, by a Lean-checked theorem (Theorem A, `lean/ChainSd/Cofactor.lean`), to a `k`-free bound on the binary weight of cofactors of low-weight multiples of `S` in `F_p[x₁,x₂]` (Missing Lemma Π). Measurements and a counting heuristic show slope ≈ 1/4 over every practical depth, but predict that Missing Lemma Π fails asymptotically (`w·log w` growth). The depth-15 upper certificate needs a closure on 2.6 M columns with near-full rank; no available host can run it.
