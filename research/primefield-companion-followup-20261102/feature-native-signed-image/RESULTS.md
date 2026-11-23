# Transported signed-image qualification

PCC-20261108-80318f is unsigned producer evidence; independent signed review pending. [pcc:unverified]

validation01 passed the first build, focused checks, inherited regression and Lean checks:

```
transported_products=96 wrong_chart_counterexamples=1749 exact_image_states=3640 public_cases=843 recovered=843 unresolved=0 subsequent_updates=735 norm_calls=7009 paired_calls=4292 coefficient_reads=277576
```

The image oracle constructs dense coefficients independently by repeated multiplication of X-1 and evaluates the original mode0 and signed-product constraints over fixture candidate images. It does not use reciprocal substitution to decide membership. Its monic support polynomial matches materialized canonical state at every tested update, including a repeated radius and zero scalar. The negative control compares the true scaled reciprocal product against old-chart conjugation and records1749 disagreements. [pcc:unverified]

The public fixture E/F257: y²=x³+x+7, P=(1,3), subgroup order281, recovers all281 targets at each cap8/16/64. This yields843 target/cap cases;735 subsequent updates exercise the integrated path. Returned values are checked against the fixture scalar and by the adapter's final point verification. This does not establish equal pruning or source-call totals versus mode2. Counters are coverage totals across the focused fixture workload, not timing results or a method ranking. [pcc:unverified]

Inherited stdout is byte-identical.109 predecessor source copies and124 compiler dependencies are hash-bound. No new compile/native failure occurred. Report terminology was corrected from “nonunit” to “scale s≠1” before final visual inspection; all nonzero scales are field units.

Lean4.34.1 proves conditional `SignedTransport.inverse_pair` and `SignedTransport.transported_trace` over arbitrary commutative rings. Both depend only on propext, Classical.choice, Quot.sound. Full finite-field weighted projection and runtime refinement are not established by these two lemmas; see DERIVATION.md.

The next task is a common frozen comparison of candidate retention and full coefficient/quotient/source work against the mode2 path. It must bind these sources on all arms. No cloud job, production panel, automatic method-selection change or original-cohort modification occurred. Historical full Cargo missing-fixture limitation remains.
