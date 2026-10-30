# Weighted slots on the Weil restriction: native larger-curve experiments

This tracks the requested PR and larger experiments for
`R = B1 + B2 + [c]B3`, with `c = 1,2,4`, directly on the cubic Weil
restriction. The solver uses native S3 resultant/root calculations after
enumerating one signed slot. It constructs no covering curve. All three
arms use the same factor-base logarithm unknowns.

The primary sweep completed 12 prime-order curves over cubic extension
fields with base primes 31 through 1009 and two construction seeds.
Group orders range from 29,629 to 1,027,286,801. All 24 planned synthetic
DLP recoveries through p=241 passed. The 11,370 independently checked PDP
decisions had no dictionary disagreement, and every returned relation was
checked in the group. A full replay reproduced `records.jsonl` byte for
byte. PARI 2.17.2 separately checked all curve cardinalities, generator
orders and planted targets.

At p=1009, seed 20261019, the c=1/2/4 arms verified 21/45/47 of the same
128 targets, using 110,487/99,211/100,052 algebraic pair queries. At the
holdout seed 936451 they verified 22/42/52, using
109,779/106,812/100,150 pair queries. These are observed stage counts.
There is one 128-target panel per curve; no confidence interval or
asymptotic inference was preregistered or computed.

The [follow-up protocol](FOLLOWUP.md) completed all 12 additional full
recoveries on the four larger fixtures, with zero budget exhaustion.
Across the two runs this gives 36 verified recoveries on 12 distinct curves.
At p=1009 the c=1/2/4 arms took 3044/1247/1152 residuals for seed 20261019
and 3063/1302/1177 for seed 936451. The follow-up recorded 18,344 dictionary
validation calls, including repeated stage panels. It is exploratory after
seeing the primary results; its curves and panels overlap the primary run.

## Evidence

| Request or check | Status and evidence |
|---|---|
| Native larger-curve matched experiments | Complete: [primary raw records](runs/native-20261019-primary/records.jsonl), [configuration](runs/native-20261019-primary/config.json), [protocol](PROTOCOL.md) |
| Correct decompositions and known-answer recoveries | Complete for primary scope: per-arm verification in raw records; solver receives an instance with secret scalar set to zero |
| Independent fixture arithmetic | Complete: [PARI checks](runs/native-20261019-primary/pari_checks.jsonl), [C verifier](pari_check.c) |
| Deterministic replay | Complete: [replay records](runs/native-20261019-replay/records.jsonl), [receipt](validation/replay.txt) |
| Larger full recoveries | Complete: 12/12 verified; [frozen follow-up](FOLLOWUP.md), [raw records](runs/native-20261019-followup/records.jsonl), [report](reports/run-20261019T0531Z/REPORT.md) |
| Re-runnable experiment tracking | [Native example](../../examples/weil_weighted_decomposition.rs), [manual GitHub workflow](../../.github/workflows/weil-weighted-experiments.yml) |
| Reports, figures and PDFs | [Run reports](reports/), [benchmark copies](../../benchmarks/20261019-weil-weighted-decomposition/) |
| Complete cost / comparison with rho | Open: [named measurement gap](../../problems/weil-weighted-complete-cost.md) |

## Reproduce

See [control arrangement](PROTOCOL_NOTES.md): PDP A/A runs are inside each
case; full primary DLP A/A repetitions are supplied by the byte-identical
whole-run replay. Its 24 duplicate recoveries are additional verification
and are excluded from the 36-case headline.

```sh
cargo test --release --example weil_weighted_decomposition
cargo build --release --example weil_weighted_decomposition
target/release/examples/weil_weighted_decomposition \
  --primes 31,61,127,241,499,1009 --seeds 20261019,936451 \
  --queries 128 --dlp-max-p 241 --max-residuals 10000 \
  --max-pair-tests 5000000 --out /tmp/weil-primary-new
target/release/examples/weil_weighted_decomposition --summarize /tmp/weil-primary-new
# Follow-up: --primes 499,1009 --dlp-max-p 1009, with other limits unchanged.
```

The output directory must not exist. Full recoveries stop at the first
applicable budget, including inside a partially scanned residual. The
independent signed-pair dictionary is only a correctness reference; the
algebraic solver cannot query it. It is quadratic in factor-base size,
not a whole-field/group operation table. This diagnostic verifier is not
a proposed scalable relation-collection algorithm.

To repeat the external checks, compile `pari_check.c` against a native
PARI 2.17.2 library and pipe the run's generated `pari_checks.gp` into it.
The checker requires `[1,1,1]` for every fixture. `render.py` is document
layout only: it plots values already computed by Rust into `summary.json`.
Python does not construct curves, run experiments or compute result metrics.

## Interpretation and open measurements

The tested weighted arms give more verified decompositions on these
fixtures. This extends the earlier conversation's small Python evidence
with a new native matched suite; those older figures are historical,
not native runtime baselines. Publication novelty is not established.

Pair queries, legacy field multiplication counters, group operations and
modular elimination operations are distinct units. Fixture generation,
reference dictionary work and internal solver work are not fully priced.
Complete S, IC/rho ratio and end-to-end speedup remain null. There is no
matched ecbench rho run, independent-host replay, admitted wall timing or
external fastest-tool comparison. No cryptographic-size, binary-field or
ECC2K-130 result follows. The complete-cost leaderboard and progress
timeline receive no new ranked point from this stage study.

Native curve identities and exact field/subgroup/generator metadata are
exported to [the curve registry source](../../docs/curves/sources/weil-weighted-20261019.json).
The local validation record preserves both successful checks and initial
environment/toolchain failures; see [validation](validation/README.md).
