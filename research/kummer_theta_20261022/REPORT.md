# Base-field theta setup gate on 21 genus-two models

The frozen search of 60 eligible affine Rosenhain orders per model found
base-field theta constants on all 21 checked Scholten quintics. It counted
1,680 qualifying tuples with the fixed gauges `b=f=1`: one model had 48,
19 had 80, and one had 112. The independent lookup-based replay reproduced
every count and checked the original equations for each candidate. These
are setup counts, not DLP operation counts or costs.

For `p=739`, the first selected order has `(lambda,mu,nu)=(296,16,10)`
and `(a,b,c,d,e,f)=(369,1,255,605,719,1)`. Its associated
`(A,B,C,D)=(491,249,18,718)` and `alpha=40` pass all published
identities modulo 739. All 21 selections are in `results/theta-models.jsonl`.

The source model and exact root factorization are from
`research/four_followups_20261021/results/kummer-model-gate.jsonl`.
The transformation and theta equations follow
[Chung, Costello, and Smith, section 7.1](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf).
The bounded search, count meaning, and checks are specified in
`PROTOCOL.md`, `DATA_DICTIONARY.md`, and `VALIDATION.md`. The graph plots
each model's qualifying tuple count. Fast coordinate arithmetic, signed
arbitrary-jump relation labels, and all cost ratios remain unmeasured.
