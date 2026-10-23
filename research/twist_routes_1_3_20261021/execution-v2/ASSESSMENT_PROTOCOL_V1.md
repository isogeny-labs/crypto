# Follow-up inventory and fresh correctness assessment

This freezes a native Rust assessment of the completed 2026-10-23 local
panels. It adds no timed engines, targets, curve search or performance claim.
It reads the existing `execution-v2/runs/` tree and writes a fresh directory
outside that tree. Commit the source and this protocol before executing it.

Command from the repository root:

```sh
runner/target/release/twist-route-experiments followup assessment \
  execution-v1/fixtures.json execution-v2/assessment-20261023-v1 execution-v2
```

Paths in the command are relative to the study directory when abbreviated
above; the actual receipt records the complete arguments. Rust 1.90.0,
one worker, 4 GiB address-space bound, 3,600 second cap, isolation L0.
No wall time is admitted. Native source checks and compilation precede the
source freeze; the running binary is copied to a revision-specific path.

The assessment hashes every raw member, rechecks the transport and native
admission gate for every historically accepted PARI replay, and retains all
historical rejections. It checks unchanged E18 certificate rows across the
GP-emitter repair; degree-three polynomial/map certificates; every recorded
source scalar; higher-prime point/graph cardinalities; identical E21 projected
rows; all E22 pair/triple histograms and energies by fresh group arithmetic;
all unique E23 raw and eliminated equations and source scalar checks; E24
class totals; E25 engine exit/certificate inventories and negative controls;
and the durable E29 Lean audit. Targets sampled with replacement are counted
both as labeled attempts and as distinct fixture/point pairs.

E25's manual S-pair run printed its complete 49-row summary, but its mutable
shell wrapper subsequently failed while parsing the receipt-writing tail.
There is no original JSON receipt for that run. Preserve that gap; do not
reconstruct an invented success receipt. The separately frozen leading-ideal
certificate replay has its own complete, successful receipt. Historical
solver results and failed/incomplete checker outputs remain unchanged.

All twelve follow-up experiments retain their original acceptance criteria.
E18–E25 and E29 have partial evidence; E26–E28 are not yet executed. No funded
backlog state, complete-experiment flag, speedup, novelty, NIST logarithm,
or independent-host result is promoted by this inventory. All statistic
calculations and mathematical replay run in Rust; document rendering may
read these recorded results without calculating new scientific quantities.
