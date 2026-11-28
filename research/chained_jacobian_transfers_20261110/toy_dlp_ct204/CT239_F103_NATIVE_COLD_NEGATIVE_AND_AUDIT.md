# CT239 — F103 cubic norm-fiber native cold cost: a measured negative result

**2026-11-19. Scope: one public known-answer source target, F103 toy, one GitHub-hosted runner, 3 alternating-order cold Rust solves. [pcc:unverified].** No single-target ECDLP advantage. No independent native timing replay or signed PCC.

## Provenance and original extraction

- Source native Rust test: [`tests/ct238_native_cubic_norm_compression.rs`](../../../tests/ct238_native_cubic_norm_compression.rs), exact GitHub checkout SHA `6a80febe14bf2a1aa6d34f08585813d95f56578c` (see `git log -1 --format=%H` in job log).
- Complete [hosted native run #44571660604](https://github.com/isogeny-labs/crypto/actions/runs/44571660604), job `135728052816`, **conclusion success**, one release Rust test passed. Execution used `cargo test --locked --release --test ct238_native_cubic_norm_compression ct238_paired_native -- --nocapture --test-threads=1`.
- Original log printed **three** `CT238_NORM_COMPRESSED_PAIR_JSON` rows inside the single Rust test and then repeated the same rows via CI `grep`. These are **3 original observations, not 6**.
- [Frozen original three records](ct239_f103_native_three_pairs.jsonl). Native CI lasted substantially longer to compile dependencies; **the approximately 27-minute compile is not part of either per-solve timer**.
- [Standalone record-only independent arithmetic auditor](ct239_f103_native_record_audit.py), with a [separate hosted audit #44581972007](https://github.com/isogeny-labs/crypto/actions/runs/44581972007), job `135764715600`, **passed** ordinary Python and `python3 -O`. Both runs recomputed all three integer-time ratios and independently verified `#E(F103)=109`, `[109]G=0`, `[73](1,51)=(42,71)`, phase-total reconciliation, full counter invariance, and rejection of all **16** corrupted/missing/duplicate panels. Neither the verifier nor this audit replayed native relation witnesses or measured an independent second runner. [pcc:unverified].

## Same-curve, same-host cost observations

`E/F103:y²=x³+x+24`, `C/F103:v²=u⁹+u³+24`, `G=(1,51)`, `Q=(42,71)=[73]G`, prime source order 109.

| Round | Order | Cold candidate, ns | Direct BSGS, ns | Baseline / candidate | Outcome |
|---:|---|---:|---:|---:|---|
| 0 | IC→BSGS | 14,814,392 | 1,978 | 0.0001335188 | complete known-answer verification |
| 1 | BSGS→IC | 14,638,794 | 1,758 | 0.0001200919 | complete known-answer verification |
| 2 | IC→BSGS | 14,633,619 | 1,905 | 0.0001301797 | complete known-answer verification |

**Median cold candidate** 14.638794 ms; **median direct BSGS** 1.905 μs; **median paired baseline/candidate** 0.000130179691 (approximately **7,682× slower** for the candidate). The median of paired ratios is correctly computed independently of the ratio of medians; only three nonisolated observations were obtained. Source scalar was public known-answer fixture, and the Rust test includes a source `[73]G=Q` precheck; it is not a blind or hidden-target test.

Every trial reached full compressed source-norm matrix rank **22/22**, with **27** accepted fully split relations among **381** candidates and **four** offsets. Observed Cantor additions **385**, target additions **28**, direct BSGS additions **17**; `FieldAdd=467035`, `FieldMul=374128`, `FieldSqr=5224`, `FieldInv=7247` per candidate round (primitives as exposed by the existing instrumentation). Relation collection consumed **91.08–91.19%** of each candidate wall time; all measured setup, collection, LA, recovery, and final verification phases were included.

## Scientific interpretation and explicit limits

This native result **reproduces the CT238 Python pilot's rank/candidate counts** in Rust and completes its cold single-target known-answer recovery. It is a different E/F103 curve from CT236's E/F101, so any 26ms versus 14ms comparison **would be cross-curve and cannot prove an improvement**. The appropriate paired direct E/F103 baseline is approximately 1.8–2.0 μs. At this size, further fixed full-base tuning of this collector is very unlikely to close a 7,000× gap; the data do not extrapolate to a general cryptographic lower bound.

The full rational norm-column count remains `p/6+O(sqrt(p))` for the stated smooth cubic-cover family; see [CT238 counting proof](CT238_NORM_FIBER_SCALING_BOUND.md). This orbit compression is a constant factor, not itself a subexponential construction. This is a family-specific obstruction, **not** a no-go theorem for variable-degree or reduced-basis methods.

The test does **not** measure peak memory, an optimized rho baseline, frontier index calculus on the identical source, multi-host confidence intervals, signed PCC, or the physical `C169`, `C13a`, `C13b`, `N2_actual`, `D` original 695/701 norm-reuse path. It does not authenticate original custody and is not a complete qualification for any claimed computational advantage. [pcc:unverified].

## Decision and next falsifiable experiment

Record the negative. **Do not spend additional iterations optimizing this same toy full norm-fiber collector.** Next distinct lead: a *reduced* source norm base with LP pairing / full target recovery, or an actually different supported cover with better complete cost scaling. Preregister the same curve/host single-target comparison and full setup/cold relation collection/linear algebra/recovery, peak memory, and matched optimized rho/BSGS/frontier IC before advertising a speedup. Native compilation/source and original norm-fixture auth gates remain separate. Reuse [crypto #4697](https://github.com/isogeny-labs/crypto/pull/4697) supported cover infrastructure and [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523) existing norm-reuse evidence rather than creating a competing copy.

No paid EKS/Spot, artifact-custody bypass, ledger rechain, protected merge, or unverified speedup claim was made.
