# Three-track split-graph evaluation

## Executed result

The three tracks have run. The complete toy audit passed with zero mismatches;
all nine construction cells accepted. Original F4 verification completed
14,194/14,220 cases and retained26 root-extraction caps. Separate exact offline
verification checked all14,220 saved finite-field root sets without a CAS rerun.
The observed degree drop is accompanied by higher branch-summed reduced-row
counts than the same-base dense encoding.

Read [RESULTS.md](RESULTS.md), the [comparison dashboard](comparison.html),
the [illustrated PDF](reports/report.pdf), and the
[independent data review](review/DATA.md). [REPRODUCE.md](REPRODUCE.md) explains
archive hydration, hash checking and native replay. The prospective protocol
below remains historical; post-data additions are explicitly versioned.

User request: execute (1) exhaustive toy correctness, (2) matched actual F4,
and (3) a d=3,4,5 split-graph ladder in parallel, then capture results in a PR.
The earlier positive result is a homogeneous top-ideal theorem; this study
tests completeness and actual solving behavior. No P-256 speed claim is implied.

## Frozen workload and controls

1. `audit/`: all 18 finite targets on the supplied p=17/A=B=2/order19 model;
   both four-abscissa bases, every sign branch, every assignment in F17³,
   exact dense/reference roots, independent group replay and the identity target.
2. `f4/`: actual stock msolve-neogb F4 on the same base/targets, candidate
   secant/dense and matched nongraph secant/dense. Retain SAT/UNSAT, all
   sign branches, traces, completed degrees, matrix widths, available counters,
   memory and failures. Missing complete field-operation totals remain null.
3. `ladder/`: fields 83,101,127 and graph degrees 3,4,5, all nine cells.
   Freeze outcome-independent hash curve proposals with exact prime full group
   order, nonsingularity, nonanomalous order and nonexceptional j. Freeze 256
   curve proposals per field and 512 graph proposals per cell. Interpolate h
   through d+1 distinct signed rational points, then require all 2d distinct
   rational roots of h²−f. Preserve every rejected attempt and its work.
   Export same-size low-x/hash controls and complete secant/dense inputs.

The F4 track consumes the toy export first and the ladder export after its
construction finishes; those dependencies are explicit. The initial toy
F4 panel, exhaustive audit and ladder construction can run concurrently.

## Concrete proposed local execution bounds

The owner gave a go-ahead after the concrete bounded-local request. This is
recorded as authorization for this specific local plan and its S3-checkpoint /
EKS-routing exception. The S3 gate itself remains failed; receipts must say so.
No other backend, resource expansion or policy exception is inferred.

| Track | CPU threads | Address-space hard cap | Outer wall cap |
| --- | ---: | ---: | ---: |
| Exhaustive audit | 1 | 2 GiB | 120 seconds |
| F4 panel | 1 per child | 2 GiB | 1,800 seconds total; 10 seconds per CAS child |
| Graph ladder | 1 | 2 GiB | 180 seconds; 120-second internal stop |

At most three experiment processes run concurrently, with a combined 6 GiB
address-space budget and no paid/cloud resource. Shared-host clocks and RSS
are diagnostic L0 data. Exact degree, correctness and native counted metrics
are primary; a timing speedup is not claimed from this fallback.

The local worker has 16 GiB cgroup memory and four CPU-equivalents. GitHub CLI
authentication now succeeds. The maintained main-only EKS workflow builds and
admits `ecbench.spec/v1`; it does not currently build/admit these standalone
tools or the external neogb adapter. There is no configured AWS identity/MCP
in this worker, and `scripts/checkpoint.sh gate` still fails for no S3 upload
route. The specific local exception is recorded separately from successful
S3 admission.

## Evidence gates

- Commit source, specs, protocol, controls, seeds, caps and binary hashes before
  affected data. A later change is a versioned amendment.
- Zero FP/FN for completed exhaustive cases; preserve partial records on failure.
- Group replay every reported relation, including doubles and inverse pairs.
- Distinguish original-generator top-ideal degree, actual F4 maximum batch
  degree, final basis degree and finite-field verification. None substitutes
  for another.
- Charge all four sign branches, duplicates, construction and failed attempts.
- Report actual matrix widths/counters and unavailable fields explicitly.
- Retain negative/censored cells; no success-conditioned curve replacement.
- Independent review inspects the theorem and instruments, without running
  an unadmitted scientific panel or making a novelty claim.

Source and protocol are frozen before scientific launch. Source, artifacts,
report, outstanding obligations and follow-up PR status are collected here.
