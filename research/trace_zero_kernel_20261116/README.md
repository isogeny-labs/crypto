# Trace-zero oracle: frozen instruction-work goal met

On the ECC2K-130 field's uniform control, complete oracle instructions fall5.043× and the complete cold process falls3.608× against retained v1. The n83 control falls6.154×/4.069×. Both predeclared thresholds pass. All210 outputs/indices match, with140 saved relations replayed by separate polynomial arithmetic. Claim PCC-20261116-d3520e is unsigned producer evidence [pcc:unverified].

| Fixed d12 query | v1 eager cold Ir | Kernel lazy cold Ir | Cold ratio | Oracle vs v1 | Oracle vs compact |
|---|---:|---:|---:|---:|---:|
| scale-n83-d12-distinct-planted-0 | 96,758,357 | 31,440,436 | 3.078× | 5.238× | 1.441× |
| scale-n83-d12-uniform | 165,060,581 | 40,566,277 | 4.069× | 6.154× | 1.583× |
| scale-n83-d12-cancellation | 60,972,930 | 24,772,243 | 2.461× | 5.086× | 1.188× |
| scale-n131-d12-distinct-planted-0 | 108,446,027 | 39,756,339 | 2.728× | 3.747× | 1.217× |
| scale-n131-d12-uniform | 186,817,738 | 51,773,074 | 3.608× | 5.043× | 1.321× |
| scale-n131-d12-cancellation | 88,613,529 | 32,624,740 | 2.716× | 4.882× | 1.118× |

Oracle ratios fix eager context; the cold-process ratio compares v1-eager with kernel-lazy. The immediately preceding compact oracle is ALSO compared with eager context fixed. The full3algorithm ×2context crossing plus compact A/A is retained in [summary.json](summary.json), with all42 arm/case cells and five-repeat min/median/max. All six controls remain; no averaging across different queries [pcc:unverified].

The goal was fixed before the preceding two panels: BOTH uniform cells require oracle ratio>=5, complete cold-process ratio>=2, exact outputs and A/A<=1%. Earlier [product-projected](../trace_zero_projected_20261115/README.md) and [compact](../trace_zero_tuning_20261115/README.md) panels fell short and remain unchanged. This final result uses identical compact single-query inputs on EVERY arm, so removal of older all-workload parsing is not claimed as an algorithm improvement [pcc:unverified].

- [Inspected two-page report](report.pdf), [editable report source](report.typ), [instruction graph](instructions.svg), [recorded plotting values](plot-data.json).
- [Frozen protocol](protocol.json), [narrative](PROTOCOL.md), [freeze receipt](freeze-receipt.json), [source hashes](SOURCES.sha256).
- [Raw rows](rows.jsonl), [complete-grid/goal audit](grid-audit.json), [separate polynomial audit](independent-audit.json), [accepted export](export.json).
- [23 focused checks](checks/validation-tests.txt), [root library check](checks/root-check.txt), [pilot/resource gate](preflight-gate.json), [counter partition calibration](calibration.json).
- [Raw archive manifest](raw-tree-manifest.json), [digest](runs.tar.xz.sha256), [extraction/repack receipt](checks/archive-check.txt), [exact input hashes](input-index.json).

## What changed

For a finite full-trace-kernel residual(z,y), put t=y+zH(z); t²+zt=1. For trace-zero u, S3 is equivalent to v=t+zH(u²)+epsilon*z. Projecting modulo W=span(VV) leaves d+1 bits:13 instead of the ambient35 at d12. Packed parity rows reject contradictions; unique models must pass COMPLETE W remainder, and deficient systems retain old solver/model order. Actual supplied signs, monic roots, conservative caps and original-group replay remain.

The final private workspace specializes reduction only for the COMPLETE n83/n131 moduli; different tails retain general kernels. It generates projector multiplication maps by successive exact multiply-by-x and batches up to256 residuals in original order. Unused batch work is charged; the first residual stays scalar. Compact field storage and fixed-word x lookup are private to this oracle. General field storage and other callers remain unchanged.

Full rational trace-kernel admission retains both Tr(x)=0 and Tr(y+xH(x))=0. Numeric admission remains exact; symbolic q is materialized on demand or explicitly forced by every eager arm. The finite rational kernel is the established object used here; this is not a general scheme-chart completeness proof or a new Z[tau]-module advantage.

## Validation and measurement boundary

Twenty-three release checks pass, including every product-basis monomial for both specialized moduli, every times-x basis vector, same-degree/different-tail fallback, independent wide random operands, exact65-case outputs/indices across six algorithm/context modes, complete F32 arithmetic/admission/exceptional group comparisons, and projected inconsistent/unique/deficient cases. Root library check passes [pcc:unverified].

Every210-query arm execution has fresh native tally and Callgrind processes. The strict audit checks the complete6×7×5 grid, common exact outputs and instruction phase sums; all positives/cancellations Found/replay and both uniforms Refuted. Separate schoolbook polynomial arithmetic with Euclid inversion imports no production field/group/solver code, rechecks all65 truths/19 negatives and base admissions, then replays all140 new saved relations. Prior corrupted-point and false-refutation canaries remain. This producer-written checker is not independently signed verification [pcc:unverified].

Whole-process Ir includes common single-input parsing, fixture/context construction, numeric admissions, all per-query field/projector preparation, search, roots/sign lifting, replay, serialization and exit. Setup/oracle/post partitions sum exactly; final unphased/phased calibration passes0.1%. Field tallies stay separate; informational inversions already contain their multiplications/squares. No free batch tail or factor-base precomputation.

These are simulated instructions, not native-time measurements. The local explicit exception was CPU0/2GiB/600seconds combined final calibration/pilot/freeze/primary, zero paid launches/retries. All28 largest-class pilot calls pass; source and final protocol were pushed before primary. No primary row was missing, interrupted, excluded or retried [pcc:unverified].

## Identity, reproduction and custody

- Source commit: `a4cf78f30eb7222a8293cdc15b8825e85e1ab0fd`.
- Protocol SHA256: `ffc10a4b8048865359e9248ccf6016241d4db653eff0cc5a952df74b5af0977c`.
- Executable SHA256: `bda296f1f344fbbab46816d33ac0fbdc55cb49a3c9d1e9548b43ce052c97972d`.
- Original workload SHA256: `2a1712a97cb882ddd26e0a6c37a17c70a5ba9fc8fb57bb0a2d681331e0cabeb5`.

E0: y²+xy=x³+1. Moduli z^131+z^13+z²+z+1 and z^83+z^45+z²+z+1; original subgroup/G identities, seeds, target and point order remain in the source workload. This is known-answer finite-base decomposition.

Build in an isolated checkout at the recorded SOURCE commit after hydrating required artifact manifests. Set VALGRIND_INCLUDE to installed headers and use cargo build --locked --release with this native/Cargo.toml. The retained compact solver and private word-field snapshots compile inside the research backend facade, preserving crate-only kernel visibility. A current-head rebuild of an old study is not reproduction; verify the executable hash before analysis.

~~~sh
python3 tools/artifacts/artifacts.py pull research/trace_zero_kernel_20261116
research/trace_zero_kernel_20261116/native/target/release/trace-zero-kernel analyze research/trace_zero_kernel_20261116/rows.jsonl
python3 ../crypto-autoresearcher/tools/experiment_protocol.py check-export research/trace_zero_kernel_20261116/export.json --protocol research/trace_zero_kernel_20261116/protocol.json --verify-files
cd research/trace_zero_kernel_20261116
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
~~~

The deterministic archive has1678 exact primary/pilot/final-calibration file members. Every member was extracted and byte-hashed; repacking reproduces the archive exactly. The manifest gives archive-member paths. Large rows/archives move through the existing artifact-autofix workflow into content-addressed .artifacts.json custody; hydrate identical bytes for export checks [pcc:unverified].

All two final-report pages, four prior result pages and six corrected discovery-profile pages were visually inspected. Discovery profile tables were rewrapped after inspection; underlying raw profile bytes and counts are unchanged. Failed compile/adapter/launch receipts remain labeled and retained; no trial failure is erased.

Sources checked (KB): unavailable, wrapper127. Repository fallback: S3/Semaev/Diem, half-trace/Itoh–Tsujii, Montgomery batching, f2m word-fold/CLMUL and wide_gf2m reduction implementations. No mathematical novelty claim. Native-time, fastest-external M4RI/FLINT/Magma and signed independent verification remain open. Historical scoreboard/timeline/native-runtime records are unchanged because this component instruction panel does not meet their admission contract. All cryptographic/statistical work is Rust; Python only packages/binds byte metadata or renders recorded values.
