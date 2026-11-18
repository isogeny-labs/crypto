# H116 protocol v2 amendment receipt

Filed before candidate moments, predicate scores, timings or channel selection.

Protocol v1's deterministic selector found only two of five required curves at
the `10^8` maximum extension size. It printed no cohort. The failed condition is
retained in `COHORT_RULE.json` and the zero-output receipt is retained in
`SELECTOR_SOURCE_GATE.md`.

`FEASIBILITY_PLAN.md` was then committed before inspecting a relaxed-cap curve.
It fixed the cap ladder and required the smallest rung yielding five curves.
The exact feasibility output is `FEASIBILITY_RESULT.json`, byte-identical to
the raw selector output with SHA-256:

```text
f0311b610bde55b0d9402ba261e23ef409e40a848342db9cd832c1325dce6161
```

The smallest successful rung is `10^10`. Protocol v2 changes only that cap and
the filenames pointing to the amended rule/cohort. It retains the selector
order, five-curve count, arms, controls, hypotheses, metrics, stopping rules,
invalidation rules and decision rule.

The current protocol is a schema-valid draft. A later pre-data commit will bind
its draft revision and canonical SHA-256 before `COHORT_V2.json` is emitted.
