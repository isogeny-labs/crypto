# Conjugate-pair charts and deeper radix grading

Status: prospective development protocol; no new scientific run has started.
Freeze this document, worker script, exact source, lockfiles, binary hashes and
launch envelope in GitHub before admission. Changes after any affected result
require a dated amendment. Historical protocols and results are unchanged.

## A. Independent charts on the remaining shortfall

Question: does changing only the projective chart remove the residual quotient
dimension shortfall on the two original curve-1 conjugate-pair targets while
preserving every four-point decomposition?

Inputs: the unchanged generator of `examples/conjpair_decomp_cost.rs`,
`p=29`, curve stream seed `0x20261106_c0de`, first accepted curve (`curve=1`),
conj arm, exact targets `common0` and `planted[5, 6, 12, 19]`. Those are historical
fixture labels, not new curve identities. Field is F_(29^4), with full two-torsion
and j outside F_(29^2); this is a decomposition study, with no selected DLP subgroup.
Reconstruct from source rather than select a new target after viewing an outcome.

Reference: chart offset zero reproduces the original chart seed rule
`0x9150 + 31*curve + target.len() + arm_offset`. Candidate offsets are 1 and 2;
the source adds the offset with wrapping u64 arithmetic. Store every actual 5x5
matrix and solver seed. The solver seed is unchanged (`0xdec0 + curve`).
All solver invocations use Macaulay degree 29, chains and sparsest pivots. Each invocation
runs both a plain and a transformed solve; plain repeats are the A/A controls.
The exact meet-in-the-middle checker remains unavailable to the candidate solver
and checks the complete returned decomposition set afterwards.

Census: two targets x three offsets x plain/transformed = 12 solver invocations. Selection
filters apply to every set, and `--no-r3` excludes additional planted targets.
Do not pass a multi-offset file to the historical `--summarize-inf` command:
that summarizer's population is the original R1/R2/R3 protocol only.

Prospective rules, per target and offset [pcc:unverified]:
- A false solver-completion flag is a failure/unknown even when an empty returned set matches an
  empty oracle. Report solver success and exact-set match separately.
- A/A requires identical plain dimension, total_muls and decomposition set.
- Completed transformed calls must match the exact four-point oracle set.
  Any disagreement stops interpretation; preserve all raw data.
- Dimension 4096 in an offset-1/2 successful solve removes that target's observed
  shortfall in that chart. Continued deficits are an inconclusive mechanism
  diagnostic, not proof of multiplicity or a positive-dimensional component.
- Record rational points at infinity separately. Do not count them as verified
  three-point relations without extraction and public group-sum checks.

Primary outputs: exact set equality, solve status, quotient dimension, points at
infinity, F_p multiplications separated into transform and existing solver counters.
Extraction/oracle work is not fully counted by the historical instrument; this is
not a complete-cost comparison. Clocks are diagnostic only. Follow-up: if chart
changes affect the deficit, derive and formalize the relevant projective statement;
otherwise inspect multiplicity/components before increasing Macaulay degree again.

## B. Matched deeper radix qualification

Question: does radix-8's counted-work reduction persist at depths 11 and 12 on
identical Kummer membership systems? Depth 10 is a regression anchor.

Inputs: existing `pkm_tower_pilot --engine tower`, p=2013265921, m=2,
t=10,11,12, controls tower/null, two random draws, no planted study rows,
`--ladder-t none --kinds kummer --random 2 --planted 0`, four Rayon threads,
radix 2/4/8. All arms use the same binary and construction stream. Per target,
require equal curve, tower parameters, g, x_r, control and target index. These are
solver systems, not complete DLP workloads. The worker retains their exact source
identities and coefficients. Graded solving degrees are not compared as costs.

Census: 3 depths x 3 radices x 2 controls x 2 draws = 36 systems.
Rotation: even t runs 2,4,8; odd t runs 8,4,2. Before that panel, run the existing
radix study smoke positive controls and G0 using its original four-thread settings;
check their recorded planted/correctness fields and G0 reference counts before
reading the deeper panel. These qualification gates are separate from the 36
systems. Stop if any control fails. A final depth-10 radix-8 repeat is the A/A
control; its four counts/outcomes must match the initial depth-10 radix-8 file.
Do not auto-resubmit any failed worker or censored cell.

Primary: F_p multiply-adds, per draw. Secondary: max_cols, max_dense_entries,
completion flags and memory where the worker measures it. Primary ratio is
candidate_muladds / unit_muladds on an exactly matched, completed refutation.
`native/.../radix_compare.rs` refuses missing/duplicate draws, input mismatch,
wrong grading, missing flags, and completed non-refutations. Capped rows remain
censored with ratio=null; an entirely missing row is incomplete, not a zero cost.

Report each depth/draw/control. Call a factor-two reduction at a depth only when
both draws complete and each ratio <= 0.5. Report increasing/decreasing ratios
as finite-panel descriptions only. No asymptotic constant-factor or exponent
verdict is inferred. Compare the null arm before attributing a benefit to elliptic
structure. An msolve/Magma/FGb comparison is not implemented by this panel; any
claim about external-tool performance requires a separate equal-output experiment.

## Execution, preservation and stop conditions

Default: command-capable Iso Lab EKS Spot route; no provider or on-demand fallback.
Use source/image/binary binding and UID-1000-compatible image containing the two
examples, native comparator, bash/coreutils, and Cargo only if building is admitted.
The current GitHub `isolab-experiment.yml` spec input is ecbench-only; it does not
submit these arbitrary example commands. An authenticated command-job route and
built image remain required; do not dispatch the integration fixture instead.

Proposed finite envelope (must pass live admission): charts 4 CPUs / 16 GiB,
absolute deadline 3 hours including provisioning; radix 4 CPUs / 64 GiB,
absolute deadline 10 hours including provisioning; one worker at a time. Radix
per-system budget is 900 seconds, max-nnz=2000000000, process VM cap 60000000 KiB.
Chart invocation timeout is 1800 seconds. No scientific retries; failures and
partial rows remain in the record. These are bounds, not runtime estimates.
Both workstreams together have a proposed USD 20 ceiling including builds,
idle/failed/Spot work, storage and transfer, within the existing shared lab budget.
Do not launch if live pricing, charged/reserved spend or remaining headroom is
unknown. Existing global quota and pool caps remain in force.

Before any cloud build/trial: successful `scripts/checkpoint.sh gate`, committed
and pushed protocol/code/configuration, publication check receipt, live budget/
capacity check and immutable source/image digest. The worker script is a payload,
not a submitter. It is never authorization to skip these admission checks.
Checkpoint completed rows after each invocation and upload all outputs to the
canonical artifact store before the worker exits. A partial JSON line is retained
as failure evidence, not silently trimmed; resume requires a separately preserved
valid prefix. Fresh output paths refuse overwrite. Reconcile an ambiguous launch
against its original ID before considering another attempt.

After execution: raw hashes and experiment.export/v1, native analysis, descriptive
run report with plot/PDF, independent review/replay and PCC claim/standing. Extend
progress metrics and affected diagnostic/dashboard views only from admitted
outputs. New measurements and scientific claims are currently pending.

## Sources and search limits

- KB unavailable: no loaded tools; `scripts/research-kb status` exits 127, no
  executable/catalog in this environment. No novelty or corpus-wide absence claim.
- `research/lift_dense_conjugate_pair_{decomp,infinity}_20261106/`: existing
  protocols, exact-set checks, remaining chart question and exceptional base-point
  target. The proposed panel does not claim to solve that exceptional case.
- `research/pkm_radix_grading_20261104/{PROTOCOL,REPORT}.md` and worker script:
  keep four threads, shape null, same-input grading and counted metric.
- Petit–Kosters–Messeng, PKC 2016, sections 2.4, 3, 4.2 and 5:
  https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf (read directly).
  Low-degree membership encodings leave a dedicated-solving problem; step-degree
  conventions alone do not establish cost.
- Gaudry primary PDF https://eprint.iacr.org/2004/027.pdf returned HTTP 403;
  `src/cryptanalysis/gaudry_quartic.rs` and its committed study are the accessible
  implementation source. No new theorem is asserted by these interface changes.
