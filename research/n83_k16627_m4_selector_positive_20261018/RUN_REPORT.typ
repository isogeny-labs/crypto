#set page(paper: "a4", margin: 12mm)
#set text(font: "New Computer Modern", size: 9.3pt)
= N83 K = 16,627: selector-pinned four-summand control

*Run:* 2026-10-18 08:32:07–08:32:43 UTC. *Worker result:* `PASS_SELECTOR_PINNED_WITNESS`.
This is a constructed relation control on `icv1-f2m83-tm6151469093347-debefd74`.
The exact prebuilt signed-Frobenius base has 16,627 columns and 2,760,082
stored points. The accepted source is commit `b4beefcb55f05713d1d63468fcbcd21e4339fa50`;
`source-files.sha256`, `inputs-before-run.sha256`, `worker-binary.sha256` and
`BUILD_RECEIPT.md` bind the executed worker and public inputs.

#image("selector-positive-stages.svg", width: 90%)

== Frozen arm and observations

#table(
  columns: (auto, auto), inset: 5pt,
  [Measure], [Recorded value],
  [Resource envelope], [one CPU; hard 4 GiB; zero swap; 1,800 s outer cap],
  [Full model before choice pins], [201,598 variables; 5,915,238 clauses; 3,901 XOR rows],
  [Base import], [35,356.152683 ms],
  [Model construction], [187.394292 ms],
  [Root], [`Fixpoint` in 146.397875 ms; all 201,598 variables assigned],
  [Root XOR work], [3 passes; 134,226,874 ns; 4,067 row scans],
  [SAT call], [`Sat` in 8.861792 ms; one model, zero conflicts],
  [Worker wall], [35,821.806933 ms],
  [Container], [exit 0; no OOM; 2,662,318,080-byte peak],
)

== Verification and scope

The worker checked all seven x blocks against the known public points,
re-evaluated the original-field S3 chain, lifted the four summands to the
stored base, matched the preflight row, obtained rank one and rejected an
`R+G` target tamper. The raw worker, phase and progress records are in
`positive/`, bound by `positive-files.sha256`. The small direct/observed
controls passed; their timing spread is recorded in `VALIDATION.md`.
For 20 AB/BA pairs of 100 checked solves per arm, the direct batch median
was 8.842667 ms (8.514958–288.700333 ms range) and the observed batch
median was 14.477375 ms (8.521667–115.515958 ms range). This shared-host
spread is not used as an overhead correction or speed claim.

The separate native `replay/receipt.json` recomputed curve and subgroup
membership, seven group x blocks, three original-field S3 equations and
the phase-zero relation row from the compressed object. Changed-x and
changed-row receipts were rejected. The worker did not export SAT assignment
bits, so the separate replay does not check those bits. It also shares the
library's field and curve arithmetic with the worker; it is a separate
execution path, not independent arithmetic or second-host replication.

The earlier seven-x-block control reached root fixpoint with 63,910
propagated assignments, then reached its outer cap while searching. This
choice-pinned arm reached root fixpoint with 201,598 assigned variables.
The earlier arm used a different source revision and lacked the choice pins,
so these
wall values are not a paired speed ratio. This positive control does not
measure natural relation yield, useful natural rank or complete cold IC cost.
The next experiment separates selector and phase information under a
common source. Canonical end-to-end cost and curve graphs remain unchanged.
