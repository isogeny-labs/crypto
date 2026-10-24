# Frozen protocol: native factor-base shape leads

Date: 2026-10-18. Scope: exploratory public-fixture diagnostics, not an admitted
speed comparison or complete discrete-log run. This protocol and both plans
are committed before execution. The user requested new leads after delivery of
the unified CLI; this is a separate bounded study, not execution or promotion
of the draft experiment card from #1963.

## Questions and references

1. Does the prior trace-zero/Frobenius recipe's successful decomposition yield
   persist beyond the original two holdout probes? Compare coordinate and random
   controls, trace-zero GP and Frobenius-generated spans on fixed fresh probes.
2. Can changing the basis of exactly the same factor-base point set reduce
   observed successful solving degree or native F4 reduction work? Compare
   coordinate, reversed and triangular bases. Check inventory equality from
   native records; do not assume equality from their labels.
3. Does a true proper-subfield base provide useful projected relations, or
   merely easy systems inside the wrong subgroup? Keep this composite-degree
   control separate from prime-degree findings.

The primary instance is icv1-f2m31-tm90707-c95f16f5 (a=0, degree31).
Secondary exploratory fixtures are icv1-f2m7-tm13-ac10a42c (a=1, degree7),
icv1-f2m13-t181-515ee569 (a=0, degree13), and the explicit composite-degree
control icv1-f2m9-tm5-4a3ea183 (a=1, degree9). All models already exist in
the unchanged registry. Exact field, subgroup, generator and EC1/UID are
recorded by the native inventory; representation mismatches remain failures.

A Frobenius-generated prefix need not be invariant under Frobenius.
Trace-zero is not a regularity certificate. The degree9 subfield control
cannot support a prime-degree claim. Degree31's nontrivial Frobenius block
splits, unlike the degrees53/83/131 described in AGENTS.md section8b.
The degree83 confidence gate and degree131 challenge remain unperformed.
Tiny inventories at degree31 are not sized to establish random relation
collection at practical scale; zero hits in this panel is not nonexistence.

## Frozen executable, inputs and commands

- Native source snapshot: b3ed888964100f1bce4780a4c0500a399c7aee50.
- Installed executable: /home/agent/.local/bin/factor-base-sweep.
- Executable SHA256: dd9c1e582effe751777081fc4d7fc6ba1acd90e1696797ef210d20da58c3c3f0.
- Plan files: plans/primary.json and plans/secondary.json; SHA256SUMS beside them.
- Fresh target seed2026101801, identical by phase/trial within each curve.
- Constructor seeds17 and29 in the secondary shape panel. Coordinate seeds
  intentionally repeat a deterministic inventory: report duplicates explicitly.
- Primary: five configurations, 8training+16holdout+1positive control each.
- Secondary: 43configurations, 16training+32holdout+1positive control each.
- Requested total: 48configurations, 2232probes if every candidate completes.
- Per-candidate limits: primary30s, secondary20s; degree8, 2million observed
  matrix cells, 2048MiB, one worker thread, at most64abscissae.
- Serial plan execution; maximum nominal candidate budgets total1010s.
  No adaptive retries, hidden larger budgets or smaller-case substitution.
- Run in a new scratch tree, retaining stdout/stderr, supervisor events and
  every immutable native attempt. Archive the raw tree deterministically.

From the repository root:

```sh
factor-base-sweep run --plan research/factor_base_shape_leads_20261018/plans/primary.json --out /workspace/fb-shape-leads-20261018/runs/primary
factor-base-sweep verify --run /workspace/fb-shape-leads-20261018/runs/primary
factor-base-sweep run --plan research/factor_base_shape_leads_20261018/plans/secondary.json --out /workspace/fb-shape-leads-20261018/runs/secondary
factor-base-sweep verify --run /workspace/fb-shape-leads-20261018/runs/secondary
```

## Observations, acceptance and stop rules

Preserve completion/failure/cap status, exact inventory and cardinality,
phase-specific true/verified outcomes, training independent rank, successful
and negative observed solving degrees, per-target F4 reduction multiplications,
and all verification receipts. Positive control and holdout never enter
training rank. Record actual distinct targets: the small group can repeat
points. Do not count repeated coordinate recipes as independent bases.

A shape remains a follow-up lead only when its positive control qualifies,
it has useful independent training progress and at least four verified
holdout decompositions on each of two constructor seeds at a fixed curve and
dimension. This is a preregistered exploration filter, not a significance
test. Do not optimize against the holdout or select a winner by training
alone. Compare yields only while displaying actual inventory size; different
sizes or field models are not matched speed arms.

A representation-only component lead additionally requires identical point
sets, identical target truth/outcomes, no omitted independently known solution,
and a lower successful procedural degree or lower F4 reduction count on
matched successful targets. This does not certify regularity and is not an
end-to-end gain. If no lead meets these criteria, report that result and
retain the unresolved hypotheses.

Stop on the frozen budgets, including all failures and caps. A global native
verification error prevents any result promotion. The independently enumerated
finite point census checks success and rejection; it is deliberately costly
and is not an oracle available free to a large-scale solver.

## Accounting and limitations

Counted unit: F4 reduction multiplications in GF(2), with original systems,
point-query and verification operation counters retained. Construction,
root extraction, census, lookup and all verification overhead lack complete
native pricing. Therefore symmetric weights0/0.1/1 cannot establish a total
ranking: missing counts remain unknown on every arm. No total-cost winner,
full-pipeline speedup, rho comparison or fastest claim is admitted.

Runtime lines/ETAs are operational telemetry on the existing shared Linux
x86-64 workspace, without an isolation qualification or A/A timing protocol.
Do not publish their ratios as accepted speed evidence. Magma, msolve, FGb
and Singular are not available on PATH; matched external solver comparison
is not run. No external fastest comparison is replaced by an internal claim.

Record and report all negative and incomplete outcomes, with a graph/PDF
and source-linked run report. The study will not change canonical performance
values absent an admitted measured improvement. Any benchmark browsing copy
must reproduce the same diagnostic values and caveats. No new paid host,
S3 operation, historical Python generator, point-log claim or frozen-evidence
rewrite is authorized or needed. Conductor check returned127 (unavailable).

