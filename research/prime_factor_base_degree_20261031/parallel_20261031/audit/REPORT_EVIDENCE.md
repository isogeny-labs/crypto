# Independent toy artifact audit, 2026-10-31

Observed run: `run-20261031T201044Z`, retained beneath the parent session's
raw run root. Frozen code revision:
`de5a3cfdc6bfd4744a15fa301a1a0b21c872e1b1`; runtime/protocol revision:
`ff9d54f3ad4030dbe4e8426a02a2c5e5fa806b2c`.

The separate native post-data reporter verified the instrument source lock,
all 36 audit cell hashes and both reference-file hashes, and 1,080 toy F4
result/input/solver-output hashes. All 36 audit cells and 180 F4 cases passed
their recorded checks. Recorded assignment totals recompute to 707,472
structured and 10,404 dense assignments; every cell preserves four sign
branches. The audit has zero false-positive/false-negative entries or replay
failures. This post-data check did not re-enumerate the assignment search.

Independent standard affine-group replay checked all 112 reported audit
model relations, all dense witnesses, 16 inverse pairs, and all 224 reported
toy F4 relation occurrences. Each base has 56 finite ordered relations,
8 doubles and 32 unordered pairs over the complete 18-target population.

| Toy arm | Actual target-max handled degree | All-case pairs reduced | All-case rows reduced | Maximum trace columns / rows | SAT targets |
| --- | --- | ---: | ---: | --- | --- |
| Candidate secant, all four branches | 3 on 18/18 | 976 | 2,102 | 16 / 18 | 18/18 |
| Nongraph control secant, all four branches | 4 on 18/18 | 1,186 | 2,522 | 28 / 28 | 16/18 |
| Same candidate base, dense | 6 on 18/18 | 344 | 736 | 21 / 17 | 18/18 |
| Same control base, dense | 6 on 18/18 | 330 | 706 | 21 / 17 | 16/18 |

Secant pair/row counters sum **all** branches; handled degree takes the
maximum over complete verified branches. The measured handled degree is
`max(input degree, maximum selected F4 pair degree)`. It is distinct from
the earlier homogeneous top-ideal values 3, 4 and 5 and from sugar degree.
In this toy population the graph base covers two additional targets, while
both bases still have 56 total finite ordered relations.

The candidate secant arm has lower handled degree, branch-summed pair/row
counters and maximum matrix dimensions than the matched secant control.
Compared with the same-base dense encoding, secant lowers degree and
columns but increases branch-summed pair/row counters. These statements
describe the recorded component proxies. Stock solver field-operation
totals and complete cost per usable relation remain **null**; L0 shared-host
clocks do not establish a performance ratio. Row independence is unmeasured.

Toy post-data report: `report-toy-native-v1.json` in the raw root, SHA-256
`ddfc10db1f002bfda0b39443d30d5a09a7e56c95f9e1e6a9f9be82c99e36da70`.
Its audit manifest SHA-256 is
`3219b02de3913ccd11448481362512fbe7c0253feaa22a33d237d4f140545e7e`.
The new reporter source lives in `../reporting/`; it changes no instrument,
protocol, original analysis, raw result or CAS output. The larger-field v1
capped cases and any later offline replay companions must be reported
separately; this toy audit makes no claim that they were already resolved.

The checks establish recorded rational-root and group-relation correctness.
They do not certify equality of the full input and exported-basis ideals or
scheme multiplicities, literature-wide novelty, larger arity or P-256 behavior.
