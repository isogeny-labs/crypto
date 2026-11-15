# Folded-walk continuation, 2026-11-15

The user requested continued work on a transfer that produces a net speedup.
The preceding mixed-image construction is a candidate, not that outcome.
This continuation must implement the missing destination solver, check it on
public complete toy DLPs, and prepare a source-matched cost experiment.

## Review and hypotheses before implementation

- Read the latest shared log and prior transfer_search work log/records.
- KB unavailable: wrapper status127, no MCP catalog. Prior ecdb token/AWS
  access remains absent. Reuse the documented exact P-256 parameters and
  proof/identity sources; do not present unavailable lookup as a negative.
- Re-read Galbraith, primary book section14.4, particularly representative
  coefficient updates, complete operation cost and cycle collapsing. The
  prior exact D+SD=-S^2D control excludes an endomorphism-only walk.
- Read the EKS skill, current GitHub workflow, HARNESS, ACCESS and COST_CONTROLS.
  The dispatch builds ecbench, not arbitrary standalone binaries. A matching
  adapter/image and current shared budget/capacity receipt are prerequisites
  for timing. Direct AWS absence alone is not a blanket access conclusion.
- Existing root ecbench already has cycle-window/Brent and lane variants;
  use these as the source baseline when integrating. This new implementation
  supplies correctness mechanics and does not replace the fastest baseline
  with textbook rho to claim a gain.

Hypothesis H1: a 32-entry independent-increment walk with canonical signed
orbits and exact coefficient transport recovers public toy logs in the mixed
subgroup. A wrong scalar, changed point/coefficient invariant, failure to
canonicalize an orbit consistently, or hidden retry falsifies this implementation.

Hypothesis H2: any orbit-step advantage survives the extra arithmetic and
folding. This remains untested until a matched complete-cost run. Toy solves,
step counts and local clock readings cannot establish H2 for P-256.

## Frozen correctness scope

- Same prime order r, generator G and target Q for source and destination.
- 32 random independent jumps with known coefficients, derived deterministically
  from a public seed. No unknown scalar is used to select jumps or recover logs.
- Canonicalization multiplies both coefficients by the exact signed eigenvalue.
- Distinguished-point collision table plus a 16-state local cycle window.
  On a fruitless short cycle choose its canonical minimum, try successive
  doublings until outside that cycle, then bounded independent escape jumps.
  Preserve escape/restart counters; a cap produces an explicit failure.
- Complete toy fields p=1009,4093,16381, a=-3, b=P256_b mod p. Select the
  largest prime factor r congruent to1 modulo3 and at least31. If absent,
  report the fixture inapplicable rather than silently substitute another.
- Actual P-256: generator/image order, coefficient invariant, orbit invariance
  and a bounded transition trace only; no full-size DLP is attempted.
- Negative controls: degenerate orbit-only steps, corrupted orbit multiplier,
  invalid group input, impossible collision denominator and exhausted budget.

Correctness commands have a two-minute timeout; the initial local development
budget is30 CPU-minutes. No local timing is admitted as a performance result.

## Later measurement contract

Source negation walk, identical A/A copy, destination generic control and
destination signed-rotation walk; retain input identity, scalar replay, setup,
mapping, group work, folding/hash, cycle handling, recovery and verification.
Use at least one warmup and five interleaved measured rounds on RunPod.
Wall-time admission still requires proven isolation and a passing A/A control;
shared-host timing alone cannot establish a speedup.
The preliminary threshold is destination total step cost below sqrt(3) times
the strongest applicable source step; complete totals still decide advantage.

### 06:20Z amendment: explicitly selected RunPod

The user selected RunPod after the EKS route was investigated. Use one small
CPU pod, no GPU; at most four vCPUs, 30GB disk, a 30-minute allocation cap and
USD1 total cap including storage. Check actual rate before allocation, fail
closed above either cap and terminate the identified pod after fetching evidence.
Freeze exact source/package, image, spec and request identity before execution.
Record provider receipt, host/CPU/cgroup/affinity and all failures. RunPod's
documented proxy route returned HTTP401; no cloud spend has begun. Authentication
is pending. The obsolete EKS budget question is not a RunPod requirement.

## Scope

New folded_walk directory and its evidence/report, the related problem record,
PCC registrations and shared log. Preserve pinned prior source, results and
claims. Conductor is unavailable. Extend PR4258 if still open; otherwise make
a regular follow-on under the repository workflow.
