# Prime31 run 51 — preregistered F4 admission and failed-search cost gate

**Scope:** Feasibility analysis and reproducible combinatorial admission gate, not blind F4 roots or a cheaper ECDLP solver. Same ordinary curve (p=2147483647), (n=2147446831). Reuse the Rust `src/cryptanalysis/f4_fp.rs` engine; do not introduce another EC arithmetic engine.

## Sources checked and overlap

- `AGENTS.md`, `docs/ACTIVE.md`, `src/cryptanalysis/f4_fp.rs` on repository main (read 2026-11-21).
- Existing public-only selector/RR9 project: PR #4692 (separate predicate; its own registered panels and blocked checkpoint).
- Earlier Prime31 Horner / F3 work: PR #4812, and the exact x-only doubling result: PR #4810.
- Archived local run49/run50: `prime31_run50_research_20261121.zip` and its `experiments/prime31_run50/REPORT.md` (not part of main); run50 supplied the published 4,096-target control and source-only Rust MQ7 chart.
- Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves*, FFA 51 (2018), DOI 10.1016/j.ffa.2018.01.009 (one-relation strategy; not a demonstrated victory against optimized Prime31 controls).
- *Complexity bounds on Semaev's naive index calculus method for ECDLP*, J. Math. Cryptol., DOI 10.1515/jmc-2019-0029 (conditional lower-bound analysis; not a theorem excluding every specialized hybrid).

## New exact admission arithmetic

Let `h=[2327,983,421,207,87,34,22,9]` be archived run50 first-hit counts for the eight seven-leaf omission arms, with six misses requiring an eight-leaf fallback, on exactly 4,096 public targets.

- Seven-leaf algebraic calls: `sum((i+1)*h[i])+8*6=7297`; mean **7297/4096 = 1.781494140625** calls per public target.
- Thus with comparable preprocessing and identical per-call successful/negative pricing, seven-leaf must cost **less than 4096/7297 = 0.56132657** of a single eight-leaf call to win. This is a **necessary** condition under that simplified pricing; failed calls may differ in cost and the actual complete-solver gate is stricter.
- Calls for each arm (including misses) are `[4096,1769,786,365,158,71,37,15]`. Negative-probe expense must be reported separately, not averaged away.
- Six misses out of 4,096 do **not** prove universal seven-leaf coverage. The eight-leaf fallback is only known to succeed on the archived corpus.

## Formal dense Macaulay envelopes (NOT observed F4 matrices)

For `v` variables, all total-degree-at-most-`d` monomials number `C(v+d,d)`; a *dense square* `u32` matrix on that envelope costs `4*C(v+d,d)^2` bytes. These are **representation upper-envelope diagnostics**, not lower bounds on sparse F4 or actual rows.

| Variables | d=4 monomials | d=5 monomials | dense-square d=5 TiB |
|---:|---:|---:|---:|
| 16 (archived W9) | 4,845 | 20,349 | 0.001506 |
| 31 (compact oriented) | 52,360 | 376,992 | 0.5170 |
| 64 (MQ7) | 814,385 | 11,238,513 | 459.49 |
| 74 (MQ8) | 1,426,425 | 22,537,515 | 1,847.87 |

The existing F4 engine's own module documentation states that it is designed for *few unknowns, small primes, degrees in the tens*; its fast packed-monomial route only covers at most 16 variables and bounded exponents. MQ7/MQ8 therefore use slower generic exponent-vector handling. `p<2^32` is nominally in scope, but **field-size eligibility is not a solvability or memory guarantee**. The solver's documented fallback may enumerate all `p` values of a variable if no univariate polynomial is found, which is unacceptable at this p without an external fail-closed cap.

## Registered Rust experiment (NOT executed)

1. Before F4, compile `cargo test --release` against the live exact Rust source, verify MQ7/MQ8 coefficient serialization parity with independently generated public-only systems, and run arithmetic boundary tests near p. No secret witness coefficients enter solver inputs.
2. Gate one (16 previously unplanted public targets): run existing `f4_fp` with fixed CPU/memory/deadline, log `F4Progress` rows/columns/degree/field multiplications/RSS, and enforce `timeout|OOM|unsupported|no-solution-proven|solution-verified` as distinct outcomes. Never call an incomplete degree-bound run UNSAT.
3. Compare same backend, monomial order, and host for W9 16-variable/coset7, oriented compact31, MQ7 64, MQ8 74; start with W9 because it fits the engine's fast monomial path and lower combinatorial envelopes. Use the frozen 46+/18− B8 corpus only when the predicate truly matches; run50 seven-coset corpus is a different relation language.
4. Promote only if blinded public Q yields a verified signed decomposition without enumerating generic group sums or importing scalar/witness indices. Then integrate *the existing* rank-112 recovery pipeline, including factor-base acquisition, relation rank, negative probes, exceptional charts, fallback, and independently verified unknown target logs.
5. Compare cold and amortized 1, 32, 128, 1,024, 4,096 targets against best same-runner native Rust index-calculus, negated BSGS and optimized negation-safe rho. Report isolated paired repetitions and uncertainty; no claimed win before that.

**Unresolved:** there is still no verified blind sparse-F4 root recovery on Prime31 and no established fully charged non-generic solver advantage.
