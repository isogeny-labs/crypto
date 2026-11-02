# IC campaign frontier implementations and validation remain incomplete

- **Status:** `open`
- **Kind:** implementation gaps; validation gaps; access/tool limits; measured resource limits
- **Affected work:** [20-item campaign](../research/ideas_execution_20261018/CAMPAIGN.md), [requirement ledger](../research/ideas_execution_20261018/requirements.json)
- **Next action owner:** campaign coordinator; implementation and independent-replay owners still to be assigned per frontier trial
- **Last updated:** 2026-10-18

## Problem

The campaign cannot yet establish its requested realistic solver crossovers or
complete the original frontier trials because the executed controls lack full
closure/F4 integration, wide-variable adapters, certified large-case replay,
credible scaling data and an accessible GPU launch path.

These are specific execution and evidence gaps. They are not mathematical
impossibility results. The [existing 64-variable problem](binary-boolean-system-64-variable-cap.md)
already records the shared builder obstruction and its separate u128 study
workaround; this campaign does not duplicate or supersede that record.

## Evidence and uncertainty

| Obstruction | Observed evidence and exact scope | Missing capability / limit on inference |
|---|---|---|
| Disk reduction is not full Boolean F4 or MutantXL closure | [Eight degree-4 passes](../research/ideas_execution_20261018/bounded_memory/README.md), four small exact dense-RREF controls, diagnostic n17/m4/ell6 rank 30,888 from 31,110 rows under 1 MiB pivot cache | No closure/critical-pair scheduler or integration into the incumbent oracle; no solving degree was measured |
| Current u64 descent exporter refuses ell7 example | [Retained refusal](../research/ideas_execution_20261018/bounded_memory/runs/run-20261018-native-v2/ell7-representation-limit.stderr): n19/m4/ell7 requires 66 Boolean variables; canonical n23/m4/ell6 needs 70 | Wider monomials/assignment types and exact solver/encoder adapters, not a mathematical variable bound; a separate wide solver exists but is not integrated here |
| Large disk passes lack independent certification | [Status](../research/ideas_execution_20261018/bounded_memory/status.json): n15 reference refused its explicit 256 MiB construction budget; n17 original phases hit 64 MiB trace caps, then copied checkpoints continued without the dense oracle | Completion and cross-policy rank agreement do not replace independent exact large-case row-space/closure certificates; all failures remain recorded |
| Campaign F5/order adapter stops at twelve variables | `cryptanalysis:experiments/ideas_boolean_20261018/runs/native-orders-r1/native-orders.cpp` rejects N>12; both scalar and native S5 arms retained twelve 20-second timeouts, while M4RI completed its tiny certificate controls | This adapter cap is not a claim that the repository has no F5 engine. Full Joux-Vitse symbolic transfer, larger nondegenerate m4 cells and strongest external references remain missing |
| No credible Boolean-solver exponent c fit | `cryptanalysis:experiments/ideas_boolean_20261018/REPORT.md`: three tiny sizes mix two arities and measured input degrees six/twelve; Dinur's quoted quadratic exponent does not apply directly | Need a fixed declared descended family with all auxiliaries counted, realistic ladder, fully charged search, uncertainty and matched baselines. A completed tiny decision/parity control does not close the paradigm |
| Archive surrogate does not supply a finite frontier bound or full replay | `cryptanalysis:experiments/ideas_learning_20261018/REPORT.md`: 1,718 aggregate receipts, four incomplete; only three whole-curve calibration groups give an unbounded 90% group interval; individual historical inputs/executable snapshots not reconstructed | Need more independent calibration curves, actual F4 degree/per-cell memory labels, prospective validation and individual receipt replay before buying frontier runs from calibrated upper bounds |
| GPU/distributed execution inaccessible on the current launch path | Local host has four CPUs, 16 GiB and no GPU; [campaign resource note](../research/ideas_execution_20261018/CAMPAIGN.md) records missing local runtime credentials. Fresh read-only GitHub query on 2026-10-18 returned workflow id 367505054, `cloud-ecc2k130.yml`, state `disabled_manually`; the root's earlier doctor dispatch was refused | No GPU job or distributed rho completion ran. Repository-held secret availability is unknown; missing local credentials do not establish their absence. A working authorized launch/checkpoint/stop path is needed; the manually disabled workflow was not enabled |

The exact solver controls are in the sibling repository's
[`ideas_boolean_20261018`](../../cryptanalysis/experiments/ideas_boolean_20261018/REPORT.md)
and [`ideas_learning_20261018`](../../cryptanalysis/experiments/ideas_learning_20261018/REPORT.md)
directories. `cryptanalysis:` paths identify sibling repository evidence; local
sibling links work in the coordinated workspace and are not a claim that those
directories live inside this repository.

The fresh workflow-state observation used the read-only command
`gh api repos/aburan28/cryptanalysis/actions/workflows/cloud-ecc2k130.yml --jq '{id,name,state,path}'`.
Its returned fields were id `367505054`, name `Cloud ECC2K-130`, path
`.github/workflows/cloud-ecc2k130.yml`, and state `disabled_manually`.

**Derived or proved:** the chained layout has `m*ell+(m-2)*n` variables. The
66/70 counts above are direct layout arithmetic. No crossover, hardness or
general solver-degree theorem is established by the bounded runs.

**Suspected explanation:** sparse fill-in, closure growth and policy transfer
may control frontier cost; their relative effects remain unmeasured on the
canonical larger cells. Memory engineering alone has not been shown sufficient.

**Additional downstream gaps:** [Fp²/prime m4 fixtures](../research/ideas_execution_20261018/targets_prime/status.json)
use generic table/MITM decomposition controls, not a FourQ Gaudry or prime-field
small-root method. [Oracle integration](../research/ideas_execution_20261018/oracle_integration/status.json)
retains 72 exhausted factorial attempts and unpriced oracle/bookkeeping work,
does not supply four complete alternative descended presentations, and leaves
n83 subgroup projection/useful rank/full solving open. The separate
[n23 native run](../research/ideas_execution_20261018/runs/a2-n23-20261018/REPORT.md)
has two timeouts, two completed but degree-unresolved cells and an unsupported
encoding. None of those outcomes establishes a general obstruction.

## What would unblock it

- [ ] Integrate exact disk-backed elimination into a closure-preserving Boolean
  F4/MutantXL scheduler. Complete canonical frozen m4 ell6/7 and guessed ell8/9
  cells, retaining the original requested parameters and construction,
  preprocessing, elimination, checkpoint, I/O, inference and verification costs.
- [ ] Widen the canonical builder/solver/SAT path beyond 64 variables and pass
  the existing problem record's exact n23/m4/ell6 regression/floor controls.
  Lift the campaign's independent twelve-variable adapter limit with exact
  certificates; do not silently change its old evidence.
- [ ] Independently certify the large completed row spaces and then closure
  semantics, including checkpoint/restart replay. Agreement of final rank alone
  is insufficient. Retain both policy traces and failed attempts.
- [ ] Run a prospectively frozen realistic size ladder for one solver/system
  family, include every auxiliary and search call, and report a confidence-bounded
  c fit. Reconstruct historical per-target launches before promoting surrogate
  receipts; obtain enough independent groups for finite stated conformal bounds.
- [ ] Provide an accessible CUDA/distributed runtime with exact device identity,
  accepted launch authority, durable checkpoint/DP ingestion, and bounded
  lifecycle/shutdown accounting; execute the original authorized workload there.

These conditions enable the requested measurements. Meeting them alone does not
establish an index-calculus improvement: complete verified same-target costs,
strong matched references, independent replay and the original claim gates remain.

## Next discriminating step

- **Action:** export one canonical n23/m4/ell6 cell through the existing wider
  representation, bind an exact source/input snapshot, and compare a closure-aware
  disk path against the incumbent on certified smaller controls before frontier execution.
- **Inputs and controls:** original frozen frontier base/target/seed, exact
  small dense/in-memory row-space controls, positive/negative group-law replay,
  first/minimum-weight policy controls, and interrupted checkpoint continuation.
- **Execution limits and stop condition:** retain each source protocol's limits;
  obtain the runtime its requested cell needs. Current 512 MiB disk-pass diagnostics
  and 120-second phase guards are not replacements for the original frontier budget.
- **Outcomes and next actions:** exact agreement enables the declared larger
  trial; resource exhaustion remains a censored record with counters; a semantic
  mismatch blocks claims and requires a corrected new implementation/version;
  unavailable launch access leaves the original trial open.
- **Recheck trigger:** a wider canonical adapter, exact closure certificate,
  enough new calibration curves, or a usable GPU/distributed runtime.

## Resolution and history

- 2026-10-18: recorded after actual native, Boolean and learning controls. Disk
  reduction, tiny polynomial methods, legal pivot hooks and aggregate surrogates
  executed; original realistic/frontier acceptance requirements remain unresolved.
  Adding this record or merging the campaign does not resolve them.
