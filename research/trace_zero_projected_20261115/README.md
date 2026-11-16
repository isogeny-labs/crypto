# Product-projected oracle: preserved partial comparison

Both uniform cells improve, but this panel **does not meet** the frozen5× oracle threshold. Full cold-process ratios exceed2×. This is simulated instruction work on six disclosed controls; claim PCC-20261115-8d7f73 remains unsigned [pcc:unverified].

| Fixed query | Unit | v1 eager | projected candidate | v1/candidate |
|---|---|---:|---:|---:|
| scale-n131-d12-uniform | oracle_instructions | 128,444,524 | 55,037,220 | 2.334× |
| scale-n131-d12-uniform | instructions | 187,976,209 | 81,842,368 | 2.297× |
| scale-n83-d12-uniform | oracle_instructions | 133,216,763 | 54,086,964 | 2.463× |
| scale-n83-d12-uniform | instructions | 165,786,662 | 73,498,596 | 2.256× |

The oracle ratio fixes eager symbolic setup; the whole-process ratio uses projected-lazy against v1-eager. The full three-algorithm/two-context crossing and identical A/A are retained. No inference from removed all-workload parsing: EVERY arm reads the exact same single-query payload. See all42 arm/case cells, five-repeat min/median/max, phase costs and field ledgers in [summary.json](summary.json). No cell is dropped [pcc:unverified].

- [Two-page results report](report.pdf), [editable source](report.typ), [instruction graph](instructions.svg).
- [Frozen protocol](protocol.json), [narrative](PROTOCOL.md), [freeze receipt](freeze-receipt.json), [source hashes](SOURCES.sha256).
- [Raw rows](rows.jsonl), [strict grid/goal audit](grid-audit.json), [separate polynomial audit](independent-audit.json).
- [Accepted canonical export](export.json), [archive manifest](raw-tree-manifest.json), [archive digest](runs.tar.xz.sha256), [archive replay receipt](checks/archive-check.txt).
- [Pilot](preflight.json), [resource gate](preflight-gate.json), [partition calibration](calibration.json), [all exact input hashes](input-index.json).

All210 planned executions complete with no invalid, missing, excluded or retried row. Each execution uses two fresh processes: native field tally and Callgrind with tally disabled. All decisions, witness indices/order, models and common diagnostics match;140 saved point tuples are replayed by separate schoolbook-polynomial arithmetic. That checker also rechecks all65 input truths, including19 negative truths and all base admissions. The checker is producer-written; there is no independently signed verdict [pcc:unverified].

## Algebra and implementation

For a finite full-trace-kernel residual(z,y), t=y+zH(z) satisfies t²+zt=1. With trace-zero u, the S3 condition is equivalent to v=t+zH(u²)+epsilon*z. Project modulo W=span(VV): d+1 unknown bits, thirteen at d12 instead of the ambient35. A reconstructed full-rank model must pass COMPLETE W membership. Deficient cases retain old solver and model order; roots, actual supplied signs, caps and original-group replay remain. Exact rational membership keeps both Tr(x)=0 and Tr(y+xH(x))=0. Symbolic q is lazy with an explicit eager method, crossed in every algorithm arm.

Projection avoids unnecessary ambient product variables and uses packed parity rows with early inconsistency. Lazy q setup is a separately crossed factor. Eleven focused checks plus prior full-variety10, prior20, terminal2 and earlier-linearized3 pass; root library check passes. Evaluated certificate IDC1hc5e3d9a8fc4a9f9f covers the integral translation expansion only [pcc:unverified].

## Identity, reproduction and custody

- Source commit: `47a8408d441411293547237242e584d83112669e`.
- Protocol SHA256: `beb988d1972342fdbb422b209eb895089395abdde7729deb1d2b33c750848c58`.
- Executable SHA256: `768193f99099dfa6f8cfcc3b87a658fc4fe186c40ab61f001f9c707318d19757`.
- Original65-case workload SHA256: `2a1712a97cb882ddd26e0a6c37a17c70a5ba9fc8fb57bb0a2d681331e0cabeb5`.

E0 is y²+xy=x³+1. Moduli: z^131+z^13+z²+z+1 and z^83+z^45+z²+z+1. Original subgroup/generator, targets, seeds and point order are in the source workload. These are known-answer finite-base decompositions; no factor-base logarithms or public challenge recovery is asserted.

To reproduce the executable, use an isolated checkout at the recorded SOURCE commit, hydrate its required artifact manifests, then build the recorded locked manifest with VALGRIND_INCLUDE set to installed Valgrind headers. Rebuilding an old study against newer production sources is not a reproduction. At this result head, use the saved executable or verify its digest before running frozen native analysis. The phase shim uses Valgrind client requests; no perf/PMU/native-time claim.

~~~sh
python3 tools/artifacts/artifacts.py pull research/trace_zero_projected_20261115
python3 ../crypto-autoresearcher/tools/experiment_protocol.py check-export research/trace_zero_projected_20261115/export.json --protocol research/trace_zero_projected_20261115/protocol.json --verify-files
cd research/trace_zero_projected_20261115
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
~~~

The deterministic archive contains 1679 exact files: primary, pilot, final calibration and available discovery counter files. Every member was extracted/byte-hashed and repacked identically. Individual hashes use archive-member paths, not promised loose Git paths. Large rows/archives move through the existing same-repository artifact-autofix workflow to content-addressed .artifacts.json custody; hydrate identical bytes before export checking. Older source-bound studies and failed outcomes remain unchanged [pcc:unverified].

The local exception uses CPU0/2GiB/600seconds combined calibration/pilot/freeze/primary, zero paid resources/retries. The28-call engineering estimate is planning only. A/A is below1%; all primary intervals sum exactly to whole process and phased/unphased calibration passes0.1% [pcc:unverified].

Sources checked (KB): unavailable, wrapper127. Fallback: repository S3/Semaev/Diem, half-trace/Itoh–Tsujii and Montgomery batching references. No new Z[tau] relation-density/module result or novelty claim. Native-time, fastest-external M4RI/FLINT/Magma comparison and signed independent verification remain open. Existing scoreboard/timeline/native runtime values stay historical. All research and cryptographic statistics are Rust; Python only binds/packages bytes or renders already recorded values.
