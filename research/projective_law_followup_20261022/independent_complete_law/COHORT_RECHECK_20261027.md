# Independent complete-law cohort recheck, 2026-10-27

This is a read-only recheck of the frozen public known-answer cohort in
[`../results/`](../results/). No candidate, rho, arithmetic-gate, or proof
trial was rerun. The machine-readable [record](COHORT_RECHECK_20261027.json)
binds the 14 raw-file SHA-256 digests, protocol, three source files, replay
certificate, validator, price grids, and author-parity summary. Every bound
raw file was present and matched its recorded digest. The existing
[PDF report](../REPORT.pdf), [cost graph](../cost-ratios.svg),
[direct-price graph](../direct-price-grid.svg), and
[data dictionary](../DATA_DICTIONARY.md) remain the review outputs; this
recheck does not replace or alter them. The PDF is stored through the
study `.artifacts.json` pointer and was not hydrated for visual inspection
in this pass.

## Observations and independent calculation

Each arm has seven 180-row JSONL files, one each at source-order bits 16,
20, 24, 28, 32, 36, and 40. Within each arm, all 1,260
`(workload_id,target_index,bits)` keys are distinct; the two arms have the
same key set. Every candidate and same-process rho row has `verified=true`,
`recovered=planted`, `exhausted=false`, the same instrumented source-addition
counter, and all five cold phases. The recorded
[replay certificate](../replay-certificate.json) separately states that the
analyzer recomputed 5,040 candidate/rho source scalars across the arms.
This recheck did **not** independently rerun elliptic multiplication; its
direct scalar check is equality with the public planted value.

For each raw candidate and rho record, I independently computed

```text
GAE = (m + 0.5s + 10i) /
      (source_add.m + 0.5 source_add.s + 10 source_add.i)
    + 0.1 * (field_adds + bit_ops
             + sum(native values whose keys end in "_uncharged"))
```

I summed full cold target costs before dividing candidate by its matched
rho sum. This follows the frozen [protocol](../PROTOCOL.md), including
construction, transfer, walk setup, relation collection and signed
elimination, and source verification. The resulting values agree with the
independent native validator and the published grids:

| Arm | Candidate GAE | Same-process rho GAE | Candidate/rho | Boundary or routine normalization inversions |
| --- | ---: | ---: | ---: | ---: |
| Normalize after every weighted addition | 4,130,679,615.461902 | 1,088,802,370.3333328 | 3.7937827176086234 | 440,649,754 |
| Retain weighted state to a split boundary | 4,550,428,527.247615 | 1,088,802,370.3333328 | 4.179297043461173 | 222,375,456 |

The pooled 36-cell price-grid ranges are 2.097462–21.488710 for the
wrapper and 1.472855–28.015265 for retained state; neither has a pooled
cell below rho. Retained-state/wrapper spans 0.546762–1.303720, with 12
of 36 pooled cells below one. These ranges vary the specified operation
prices; they are not statistical confidence intervals. Some individual
size/price cells differ from the pooled ranking; the full grids preserve
them.

## Implementation and claim boundary

The [measured dispatcher](../src/bin/projective_law_gate.rs) implements
weighted 1+1, 1+2, and 2+2 arithmetic from the published Ozbay Gurler–Hisil
[law](https://link.springer.com/article/10.1007/s13389-024-00351-7) for
smooth quintics of the stated form in odd characteristic. The source-level
[audit](SOURCE_AUDIT.md), [finite author-translation parity](../public_source_parity/README.md),
and [written proof audit](../PROOF_AUDIT.md) support the implemented branch
mapping. The parity packet compared 34,704 finite author-translation,
compiled-Rust, and Cantor outputs; it did not execute Magma.

The runtime still contains `None` returns for exceptional predicates and
`expect` at the public weighted dispatch. The written proof audit argues
those predicates are unreachable for valid reduced inputs, but the compiled
source-to-certificate connection and universal branch reachability have not
been established by a faithful mechanized theorem or independent
mathematical review. This recheck records **implementation and finite
validation within the mapped family**, not universal completeness of the
compiled program. Weighted group operations avoid routine inversion;
the retained-state collector still normalizes split relation candidates,
and its 222,375,456 boundary inversions are charged. These ratios are
operation-priced cold counts, not isolated wall-time or fastest-external-tool
rankings.

## Validation

The existing repository evidence gate was run from the repository root:

```sh
python3 scripts/validation_gate.py evidence research/projective_law_followup_20261022
# valid
```

The recheck's raw-row calculation is independent of the native validator.
To reproduce it without running a target, use the formula above on the
14 SHA-bound JSONL files; [the native validator source](../src/bin/validate.rs)
and [its result](../results/independent-validation.json) provide a second
implementation of the same pricing calculation. The original raw target
records and price grids were not modified.
