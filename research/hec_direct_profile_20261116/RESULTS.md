# Direct weighted-Cantor rows: integration and complete-cost observations

The supplied-target solver now uses cold anchor-multiple caches and Gray sign traversal to replace Gaussian interpolation with exact weighted-Cantor reduction. It consumes sign batches in the reference integer order and charges unused prepared outputs. Source and target descent remain catalogue equations. The implementation and production projection argument are in solver.rs and STRUCTURE.md; these use classical group and divisor identities, without a novelty claim.

The frozen correctness gate covers 18 paired input/profile cases and 72 calls on genus 2 / p251, genus 3 / p211 and genus 4 / p61. All six selected-profile inputs recover. Across all families, 64 calls verify and eight genus-2 polynomial controls exhaust their class budget. Every consumed original equation, scalar/status, visited class/profile, admission, rank and descent count agrees between Gaussian/Direct and gate/production paths. Fields are disjoint from profile selection, but appeared in earlier correctness work. Targets are new and constructed outside the public-divisor API.

## Complete-cost comparison

Record: [20261116T0611Z-hec-direct-profile](../../benchmarks/records/20261116T0611Z-hec-direct-profile.json). Scientific source: 5efdf3e7ec5d7f15a10fd521c7a840a09b5c2f69. Pre-run checkpoint: 20261116T061139Z; registration commit 0200efe908a6 and checkpoint 20261116T061305Z. The dirty flag refers to the unrelated preexisting .resume/RESUME.md edit.

The once-only charged run completed 510 cells and 8,160 verified calls. Each cell contains sixteen fresh algorithm setups; five batch means per input/arm produce the native Rust median and min/max. The measured boundary includes tables/catalogue, caches, descent, every prepared sign output, rows/filtering, linear algebra, allocations/result drops and final original-source verification. Common fixture selection/order admission and target construction are excluded equally. Code, allocator and OS state remain warm in one shared local process: Apple M4 Pro, 14 logical CPUs, macOS 27.2 ARM, Rust 1.99.1, L0 without CPU isolation.

Seventeen arms include Direct and its A/A, Gaussian with the same profile, Compact, twelve native/table controls and a table A/A. The primary incumbent is the minimum per-input median among thirteen complete existing controls. It is Compact on every input. This is a comparison of recorded implementations, not an exhaustive external-tool comparison. All arms and raw cells are retained; no target or host pooling.

| Genus / p / target | Direct wall_s | Gaussian wall_s | Compact wall_s | Compact / Direct | Gaussian / Direct |
|---|---:|---:|---:|---:|---:|
| 2 / 251 / 0 | 0.000696510375 | 0.001472088563 | 0.000104872437 | 0.150568378 | 2.113519935 |
| 2 / 251 / 1 | 0.000591815062 | 0.001285661437 | 0.000120726562 | 0.203993731 | 2.172404048 |
| 3 / 211 / 0 | 0.001412171875 | 0.003201302125 | 0.000385765625 | 0.273171865 | 2.266935195 |
| 3 / 211 / 1 | 0.001108880187 | 0.003202747437 | 0.000367825500 | 0.331708966 | 2.888271856 |
| 4 / 61 / 0 | 0.000979958313 | 0.002315776000 | 0.000323950563 | 0.330575861 | 2.363137258 |
| 4 / 61 / 1 | 0.000929976563 | 0.002324174500 | 0.000299070313 | 0.321589086 | 2.499175349 |

## Frozen decision

All answers verify, but qualification fails: 20 of 102 arm/input spreads exceed 1.25, with worst spread 1.745383804. One Direct A/A ratio is 1.222747831, outside its frozen bounds; table A/A passes. The numerical all-six tenfold criterion also fails: minimum incumbent/Direct is 0.150568378. These are unqualified local producer observations [pcc:unverified]. The observed Gaussian/Direct ratios are 2.114–2.888; Compact has lower observed cost on all six inputs. No qualified gain or formal regression estimate follows from this cohort.

Every cell remains. There was no threshold change, first-cell removal or adaptive rerun. The constructor replacement does not establish a tenfold total-cost gain on this panel. It does not prove a universal lower bound. Direct still uses generic Cantor additions and a full-catalogue incremental echelon; their separate wall shares were not measured. A successor needs a new complete-cost case against the established incumbent. The mathematical tenfold objective remains active.

## Evidence and custody

[Full 17-arm table](timing/ALL-ARMS.md), [raw cells](timing/CELLS.tsv), [exact input identities](timing/INPUTS.txt), [native gate outputs](timing/GATES.txt), [report PDF](reports/run-20261116T0611Z/REPORT.pdf), [SVG](timing/COST.svg) and [150 dpi PNG](timing/COST.png). Native Rust computes all statistics; plot.py only renders PLOT.tsv. All three PDF pages were visually inspected.

Full hyperelliptic polynomials and Mumford source/target coordinates identify inputs; elliptic ICV1 slugs are not fabricated. Finite equivalence checks and written structural proof are separate from a machine-checked group certificate. Classification: engineering implementation plus negative/unqualified complete-cost evidence. No independent VALIDATION.json or leaderboard promotion is supplied. Benchmark progress is bookkeeping; historical ledger issues remain separate. The PR remains unmerged and the unrelated resume edit is preserved.
