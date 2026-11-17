# Inline-singleton binary pair index: bounded parity record

Recorded 2026-11-16. This report records source-bound correctness and
requested-backing observations for a standalone index. It makes no
complete-call timing or ECDLP cost determination. The frozen
[protocol](PROTOCOL.md), [source](inline_page_index.rs),
[actual-curve test](../../tests/rootfind_inline_page_parity.rs),
[raw final output](curve-final.stdout), [machine record](RESULTS.json),
[data-flow diagram](diagram.svg), [quantitative chart](requested_backing.svg)
and [PDF](report.pdf) are retained together. The final source commit is
`c3ad96f4315ed4452ab9e3fbc4862392eb1eef62`. [pcc:unverified]

## Mechanism and controls

A four-byte directory word stores a page's first pair witness in place
when its 16-key page has one occupied slot. A second distinct key in that
page promotes both witnesses into a 16-word spill page. Subsequent equal
keys return the original pair. Each witness uses two nine-bit point
indices, so the implementation refuses indices at or above 512, keys
outside `1..=2^n`, degrees outside `1..=23`, and an exhausted spill limit.
The [editable diagram](diagram.dot) shows the paths. The final unit binary
passed four tests, including all 16 valid slots of one page and failure
controls that left earlier entries intact; see [unit stdout](unit-final-test.stdout)
and [UTC/resource log](unit-final.meta). [pcc:unverified]

The actual-curve test reconstructed the ordered torsion pair-sum stream
on Koblitz E0 K13/K19/K23 from pinned ICV1 curve IDs and factor-base SHA-256
digests. Every insertion matched a standard hash map and the previous
[`PagedPairIndex`](../rootfind_paged_pair_index_20261116/paged_index.rs).
All retained and boundary keys, plus every K13 key, matched. Every later
same-key pair passed a full signed group-equation check, and the complete
collision vector matched the production torsion builder. Nine harness
tests passed; [final stdout](curve-final.stdout), [stderr](curve-final.stderr)
and [UTC/resource log](curve-final.meta) preserve the exact run. The
[result derivation](derive_results.py) asserts these invariants and checks
all counts against the earlier [paged parity result](../rootfind_paged_curve_parity_20261116/RESULTS.json).
This is a same-producer comparison, not an independent reviewer run.
[pcc:unverified]

| Cell | Base points | Inserts | First keys | Checked collisions | Singleton / spill pages | Inline requested backing | Earlier paged backing |
|:--|--:|--:|--:|--:|--:|--:|--:|
| K13 | 33 | 272 | 261 | 11 | 161 / 43 | 11,332 B | 39,172 B |
| K19 | 139 | 4,830 | 4,800 | 30 | 4,159 / 314 | 163,844 B | 758,404 B |
| K23 | 275 | 18,906 | 18,842 | 64 | 18,158 / 340 | 2,129,924 B | 4,534,916 B |

The page counts and first-witness counts above are observed in
`curve-final.stdout`. Backing bytes are derived from
`4 × directory_words + 64 × reserved_spill_capacity`, with 145/512/512
spill pages reserved on K13/K19/K23. The earlier paged requested bytes
come from its committed result. The [chart source](make_visuals.py) plots
these values on a logarithmic byte axis. These requests exclude `Vec`
headers, allocator overhead and whole-process RSS. The run's `time -l`
counters cover the Cargo/test launch path and do not measure the collector.
[pcc:unverified]

## Run custody and limits

The first dependency-free source unit run passed three tests. The first
Cargo launch then exited 101 before compilation because its copied lockfile
named the prior harness package; [refusal output](curve.stderr) is kept.
After the one-line lock identity correction was committed at `4589d65ea6`,
the [first actual-curve run](curve-locked.stdout) passed eight tests. A
review found that its synthetic control used 15 slots in one page and one
slot in the next. The additional full 16-slot control was committed at
`c3ad96f431`, and the [final unit](unit-final-test.stdout) and
[actual-curve](curve-final.stdout) runs passed four and nine tests. The
structured K13/K19/K23 parity records were identical between the two
passing curve runs. `RESULTS.json` binds source, harness, binary and raw
output SHA-256 values, UTC seconds, toolchain and exact commands. No
failed admission or earlier output was overwritten. [pcc:unverified]

The repository-wide release library validation compiled, then reported
2,796 passing tests and five ecbench band/compare failures before a
recorded interrupt. It is **incomplete and failing**, not a passing suite.
An isolated rerun of one failure showed that
`research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl`
is absent from this worktree even though its artifact manifest names it;
see [diagnostic stderr](lib-validation-diagnostic.stderr). That diagnosis
does not establish the cause of every failure. Conductor T-6701 holds the
fixture path, so it was not hydrated here. The
[suite output](lib-validation.stdout), [stderr](lib-validation.stderr),
[run metadata](lib-validation.meta) and isolated diagnostic output are
retained. The interrupt followed a red suite and sustained high CPU use
by long-running unrelated tests. [pcc:unverified]

`dominated_by`: the production hash table, opt-in flat table and prior
paged prototype remain comparison arms. `sota_delta`: no measured
complete-call or relation-yield change. The index has not been installed
in the collector; the tested domain is `n ≤ 23` with fewer than 512 base
points. The primary [rootfinding paper](../../papers/rootfinding-decomposition/rootfinding_decomposition.tex)
and the internal [program review](../../papers/ecdlp-index-calculus-program/sections/rootfinding.tex)
already cover the pair-table method and dense engineering; this screen
claims no new decomposition method. Shared KB source IDs and its stale
refresh boundary are recorded in `PROTOCOL.md`. [pcc:unverified]

Conductor T-6708 still reserves the production collector and ecbench
adapter, so integration is a later step. The frozen
[K13 F4/F5/F6 panel](../rootfind_f456_batch_combined_20261115/PROTOCOL.md)
requires L2 execution and independent replay; the live monthly EKS budget
readback exceeds its limit. The existing
[complete-cost problem](../../problems/rootfinding-complete-cost-calibration.md)
retains those obligations. Canonical performance graph sources were checked
for scope; this storage/correctness record supplies no complete-call timing
edge to add. [pcc:unverified]
