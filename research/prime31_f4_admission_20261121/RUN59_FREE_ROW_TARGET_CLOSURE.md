# Prime31 run59 — quotient-span target closure using all exact public homogeneous rows

**Date:** 2026-11-22. **Classification:** independently checked linear-algebra theorem and *conditional* query-model measurement. This is NOT a blind S3/F4 extractor, newly recovered Prime31 log, complete solver, or speedup against BSGS/rho.

The curve is unchanged: `p=2147483647`, `a=847514846`, `b=2087444724`, prime group order `n=2147446831`. This continues the existing [Prime31 PR #4901](https://github.com/isogeny-labs/crypto/pull/4901), extending run58 rather than starting a new arithmetic/F4 implementation.

## Checked previous work and literature

- Frozen run55–58 results and `research/prime31_f4_admission_20261121/run57_basis.json`: Dickson B=509 has 13 independent public zero relations; hybrid B=677 has 88 (32 pair-collision plus 56 horizontal cubic-fibre identities), exact original-curve supports excluding identity S=518084 and S=916468. The uploaded independent run56 Python verifiers reproduced all 13/88 ranks on 2026-11-22. The synthetic positive-query rank/graph model and its caveats are in `RUN58_ONE_TARGET_GRAPH.md` and original-Rust example `research/prime31_ic_20261115/native/examples/prime31_rank_completion_gate.rs`.
- Existing blind-input root extraction ownership: #4692 RR9/selector, #4812 Horner/F3, #4822 4-variable Krylov, #4809/#4846 S3 batch-root acceleration. **Do not duplicate their solvers.**
- Prior target-specific one-relation idea: A. G. Amadori, F. Pintore, M. Sala, *On the discrete logarithm problem for prime-field elliptic curves*, *Finite Fields and Their Applications* **51** (2018), 168–182, DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009). This run makes **no claim** to invent early individual-log recovery or to improve published asymptotic complexity.
- The shared research-KB MCP was not exposed to this runtime; the in-repo files, existing PRs, exact archived public evidence and cited primary publication are the checked fallback. Other related leads may exist.

## Exact algebraic refinement: quotient-signature counting

Let `e_i` be the coordinate vectors for `B` public factor-base logs `L_i` over `F_n`. Let `R` be the full row space of (1) known-scalar signed-pair collection rows and (2) the **13/88 independently certified public homogeneous free rows**. The value of a target affine signed pair `aL_i+bL_j` is determined by the known relation right sides **if and only if** `a e_i+b e_j ∈ R`.

Reduce all `e_i` in the quotient `V=F_n^B/R`. Let `A` count indices with zero quotient signature; for nonzero signatures, partition indices by their equivalence class **up to sign** `v ∼ ±v`. Let the resulting class sizes be `s_1,...,s_h`. As the group order is odd, the number of immediately evaluable **nonidentity unordered signed-pair templates** is exactly

```text
good = 2*A*A + sum(s_j*(s_j-1)).
total = 2*B*B.
p_target = good / total.
```

This strictly generalizes run58's graph-component sufficient event: where the earlier signed DSU modeled only queried affine pair rows, this quotient includes the 13/88 public free equations and detects additional log linear combinations **without waiting for complete factor-base rank**. The expression applies to uniform pair *templates*, not automatically to uniform `F_p` points or to biased one-witness algebraic extraction. It says which relation coefficients permit recovering a target, not how to extract those coefficients efficiently. A real solver must reconstruct all affine offsets and independently check `[t]G=Q`.

**Independent proof/check:** after reducing all supplied rows to echelon form, quotient signatures are normal forms for `e_i`. A signed pair is known exactly when its normal form is zero, hence either both signatures are zero or they are negatives after choosing signs. For each unordered distinct pair in one nonzero ±-class exactly two of four signed templates work; anchored pairs contribute `2A²` with the diagonal correctly counted. A second direct Gaussian-elimination implementation tested **101 separately seeded random small affine graphs with 1–4 sparse free-row coefficients** and checked *every* `2B²` pair's rank membership; all passed.

## Frozen conditional query-model result

Use *identical* 256-trial Mersenne-Twister positive-pair streams, sample seeds and hit-budget grid from run58; no other randomness. Public free rows from frozen run57 fixture are included from the start. Positive-pair rows are uniformly **synthetic** and therefore assume a hypothetical perfect, unit-cost, uniform-root extraction oracle. Mean target wait is `E[1/p_target]`, **not** `1/E[p_target]`. Scale successful-hit counts by the **exact** original-curve distinct-support fraction `S/n`, including expected negative support probes but **excluding** real S3/F4 extraction, group shifts, F4 failures, memory, factor-base setup, witness verification, exceptional branches and linear-algebra time.

| Arm | Batch | Run58 graph-only model uniform calls | Run59 + free rows model calls | Query reduction |
|---|---:|---:|---:|---:|
| Dickson B=509, rank13 | 1 | 1,335,005 | **1,328,194** | 0.51% |
| Dickson B=509, rank13 | 1,024 | 7,907,205 | **7,822,629** | 1.07% |
| Hybrid B=677, rank88 | 1 | 988,031 | **957,500** | 3.09% |
| Hybrid B=677, rank88 | 32 | 1,405,729 | **1,321,024** | 6.03% |
| Hybrid B=677, rank88 | 128 | 1,980,307 | **1,769,329** | 10.65% |
| Hybrid B=677, rank88 | 1,024 | 5,002,567 | **4,699,949** | 6.05% |

At **400 hybrid successful synthetic base relations**, mean immediate target-positive coverage increases from **0.07682** graph-only to **0.17990** with exact free rows, a **2.34× coverage increase at this fixed stopping point**, not an end-to-end speedup. Average independent free-row rank remaining at this point is 84.99 of 88; the free rows are not simply discarded when a graph component becomes anchored.

## What this does and does not establish

- **Established:** exact quotient-signature theorem, independently tested general direct linear-rank equivalence; 13/88 verified archived public relations; paired 256-trial model improvement and target-batch sensitivity on the unchanged Prime31 group.
- **Not established:** a polynomial-time non-generic decomposition oracle or extraction cost below the measured generic/index-calculus frontier. Even the improved model implies ~4.70 million potential public shifts for a 1,024-target batch. A same-host extractor must price those shifts and failed S3 probes; the old shared-BSGS 170.113ms/1024 cross-run control, if still representative, would imply only ~36ns *all-in per probe* even before other work, a sensitivity threshold, **not** a comparable measurement.
- **No benchmark promotion:** the figures are conditional query-count predictions. They are *not* measured complete ECDLP wall-clock results.

## Registered next experiment, in order

1. **Existing native Rust:** extend `prime31_rank_completion_gate.rs` with quotient-signature normal forms and exact uniform signed-pair counts, without modifying its verified original curve arithmetic, DSU, full-rank logic or reimplementing rootfinding; exercise release GitHub CI. Test `p_augmented >= p_graph` for all registered checkpoints and keep original run58 output unchanged.
2. **Real unblinded relation oracle check:** combine free 88 rows with *actually returned*, public-only, unknown-index S3 decompositions from #4809/#4846. Run the frozen m=300/400/500/600/700/800/900 grid. Independently verify signed witnesses and update the affine right sides; log valid/invalid/unknown and all negative probes.
3. **Predicted bias/primitive cost falsifier:** compare exact observed `S/n` and signed-template output bias to the synthetic model; pair the full Rust extractor on the same host against existing optimized index-calculus, BSGS and rho, with identical public Q and all setup/target counts for T=1,32,128,1024. Required success: verified complete logs, rigorous total-cost ledger, favorable paired advantage confidence interval. Otherwise classify this as a useful partial-rank optimization only.

**Key blocker:** blinded polynomial root extraction and fully charged timing, not the current linear-algebra stopping rule.
