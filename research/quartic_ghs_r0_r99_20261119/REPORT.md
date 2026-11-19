# R00–R99 parallel GHS strong-rho optimization review

**Status (2026-11-19):** all 100 lanes were registered and dispositioned in
parallel. Three production mechanisms are selectable and passed deterministic
controls. No new isolated timing, end-to-end frontier result, or leaderboard
row is claimed `[pcc:unverified]`.

This report answers two separate questions:

1. What did the R00–R99 implementation wave actually build or rule out?
2. Did that work move the admitted elliptic-curve discrete-log frontier?

The first answer is concrete source and control progress. The second answer is
not yet: execution admission was unavailable, so the existing measured
frontier remains the comparison boundary.

## Outcome at the working scope

The native closure tool accepted exactly 100 unique, ordered, contiguous IDs
from R00 through R99. The canonical states are:

| State | Lanes | Meaning in this campaign |
|:--|--:|:--|
| `executed-pass` | 30 | The registered deterministic gate passed |
| `executed-reject` | 2 | The tested form failed its gate or moved counts negatively |
| `implemented-unmeasured` | 12 | Production code/control exists; admitted timing is open |
| `screened` | 19 | Formula, source, profile, or toy-oracle screen completed |
| `blocked` | 15 | A named capability or prerequisite is unavailable |
| `queued-after-dependency` | 22 | The lane is specified behind a predecessor |

The free-form `status` on each lane retains the detailed disposition; the
six-value `state` supports aggregation without discarding that evidence.
Machine-readable records are in [`registry/`](registry/), and the global
validator receipt is [`receipts/r00_r99_closure.txt`](receipts/r00_r99_closure.txt).

## Frontier boundary carried into this round

This campaign did not rerun the admitted frontier panel. The current boundary
therefore remains the independently established 2026-10-31 result:

- **Generic prime, random-binary, and Koblitz curves.** Rho-led in the
  admitted cold one-target cells. R00–R99 has no admitted rerun.
- **Restricted GHS-weak $E_0/\mathbb F_{q^3}$ family.** Plane-quartic IC-led
  exception, reported as a separate problem class. This wave adds matched-rho
  implementation candidates but has no admitted wall result.
- **Prime P20, validated $N=45$ factor base.** 7.95 / 8.24 / 9.62 times
  matched rho across the frozen pricing band; unchanged.
- **F5/F6 and related solvers.** Solver-local reductions had not moved cold
  end-to-end ECDLP cost. The fresh matched campaign remains gated.
- **Optimized signed-Frobenius rho.** 1.246642× same-binary instruction
  reduction with unknown isolated L2 wall effect. R64 adds a new selectable
  counter-free path but no timing.

The source for that boundary is
[`../ecbench_e2e_frontier_20261031/establishment/README.md`](../ecbench_e2e_frontier_20261031/establishment/README.md),
with detailed evidence in
[`../ecbench_e2e_frontier_20261031/REPORT.md`](../ecbench_e2e_frontier_20261031/REPORT.md).
The restricted-family result must not be generalized to ordinary generic
curves.

## Parallel execution structure

Six disjoint agents began before any production candidate was promoted:

- **R00–R16 · provenance, correctness, accounting, composition:** native
  fail-closed record controls and exact 100-lane closure prerequisites.
- **R17–R33 · base/cubic-field arithmetic:** mul5 production candidate,
  exhaustive native arithmetic oracle, and explicit small-division accounting.
- **R34–R50 · setup and group arithmetic:** initial-walker setup-table reuse
  and a retry-preserving audit.
- **R51–R66 · quotient walk, symmetry, counters:** cyclic sign/collision
  oracles and a fully bare production walk.
- **R67–R83 · batching, SIMD, parallel DP rho:** deterministic scheduler
  screens and a frozen worker-scaling protocol.
- **R84–R99 · ECBench, validation, publication:** fail-closed registry
  closure and complete promotion obligations.

Independent exploration ran concurrently. Changes to the authoritative rho
source were composed serially, because standalone operation deltas are not
additive.

![The parallel search converges on serialized composition and gated promotion.](figures/promotion_path.pdf)

## Production mechanisms

### R17/R18/R19/R20/R33: five-product cubic multiplication

The selectable `-mul5` kernel evaluates at $0,1,-1,2,\infty$, interpolates
with exact canonical `/2` and `/3`, and reduces modulo $z^3-c$. The default and
explicit `-mul6` arms retain six-product Karatsuba.

Controls completed `[pcc:unverified]`:

- 768,394 exhaustive operand pairs over $q=5,7$ and every nonzero $c$;
- 180,000 seeded pairs across 18 larger $(q,c)$ cells;
- 20,000 production pairs against Karatsuba and the frozen field;
- exact `5 fq_mul + 4 fq_small_div` per cubic multiplication;
- the q=223 known answer and complete trajectory across baseline, mul5,
  walker-table, composed, fast, and bare forms; and
- a native fail-closed analyzer with twelve corruption tests and mandatory
  rational small-division sensitivity.

On the public q=223 deterministic record, mul5 changes complete abstract
counts from 110,435 to 96,884 `fq_mul`, from 400,276 to 670,668 `fq_add`, and
from zero to 54,204 `fq_small_div`. These operation classes are intentionally
not collapsed into a speedup. The exact-division implementation is also
data-dependent and is not presented as constant-time `[pcc:unverified]`.

### R36/R50: reuse the setup joint table for initial walkers

The selectable `-walker-table` path reuses the already constructed width-two
joint table one walker at a time, retaining coefficient draw order, infinity
rejection, and discarded work.

The q=223 control preserves the scalar and full trajectory. Setup counts move
from 56,484 to 48,433 `fq_mul`, 511 to 288 `fq_inv`, and 1,455 to 1,224 group
additions, while logical batch-inversion calls rise from 33 to 258
`[pcc:unverified]`.

An independent review covered 15 q=223 cells and 40 q=7 retry seeds. Four
seeds exercised real infinity retries and retained equal RNG draws and
trajectories. Promotion still needs explicit setup attempt/discard/RNG-digest
fields, a permanent forced-infinity source fixture, allocation accounting,
and admitted wall measurement.

### R64: fully counter-free production walk

`bare-b*` uses `FastF3<false>` plus a zero-sized `BareNative` sink in the same
generic mathematical walk. `strong-*` remains fully counted; `fast-*` removes
field counters but retains native diagnostics; `bare-*` removes both and marks
both authorities false.

Deterministic controls match counted/fast/bare known answers and complete
trajectory fields at widths 1, 4, 8, 16, and 32, including the composed
mul5-plus-walker-table arm. Optimized LLVM-IR gates retain zero bare field or
native counter hooks while keeping counted positive controls. This proves
counter elision in the inspected binary, not elapsed-time improvement
`[pcc:unverified]`.

![Recorded q=223 operation classes for the counted candidates. Each panel has its own unit scale.](figures/counter_composition.pdf)

The figure sums setup, walk, and verification counters from
[`receipts/r17_r36_q223.jsonl`](receipts/r17_r36_q223.jsonl). Bars are raw,
unweighted events; they are not wall time and are not comparable across
counter classes without a frozen model.

## R67–R83: optimized rho comparison plan

The worker-scaling plan now freezes native `check-parallel`, `run-parallel`,
and calibration contracts; D00–D10 deterministic fixtures; masked tails;
disjoint starts; shared transition identity; sharding; bounded queues; stop
propagation; and useful/overshoot/total-work identities.

The future admitted panel uses q=4051, 16363, and 65479 with development seeds
101–103 and five interleaved P1A/P1B/P2/P4 blocks. Every run must report wall
latency, CPU seconds, useful and overshoot steps, DP traffic, queue work,
memory, verification, affinity, and NUMA readback. The existing launcher that
runs three independent seeds concurrently is explicitly disallowed as a
worker-scaling proxy. See
[`ranges/R67_R83_MEASUREMENT_PLAN.md`](ranges/R67_R83_MEASUREMENT_PLAN.md).

## Solver and pipeline coverage

R84–R99 preserve the broader ECBench work rather than treating rho kernel work
as the entire frontier:

- **R87 · F4 and Faugère–Lachartre F4:** matched
  construction/elimination/extraction campaign queued.
- **R88 · matrix/signature F5:** registered engines screened; fresh
  equal-system run queued.
- **R89 · F6, F6-IC, pair and PKM forms:** nested engines screened; complete
  geometry/recovery accounting queued.
- **R90 · SAT-CDCL and WD-SAT:** SAT registered; WD-SAT blocked until its
  retained false-UNSAT soundness failure is repaired.
- **R91 · FES, GPU/wide FES, Gray code, guessing:** source coverage screened;
  matched campaign queued.
- **R92 · MITM, memoryless, subtract/window, negation-folded and tower
  roots:** source coverage screened; full table/group/lookup accounting queued.
- **R93 · Magma, msolve, FGb, M4GB, FLINT/NTL controls:** blocked until
  identical-input, equal-output external tools are available.
- **R94–R99 · accounting through publication:** cold accounting, holdout,
  uncertainty, replay, PCC, and publication are specified; execution or
  evidence remains gated.

The static source audit saw 111 optimization-policy declarations, 21 solver
registry entries, nine PDP3 engine strings, and four PKM strings. Those counts
are source declarations, not a fresh live inventory or solver execution.

## Rejections and open dependencies

- R29 rejected unrestricted `u64` lazy accumulation at the largest admitted
  modulus: only four maximal products fit. A staged or `u128` design remains.
- R56 retained the retry-slab candidate as count-negative and excluded it from
  production.
- Projective/x-only, cross-walker batching, SIMD, cache layout, packed DP, and
  parallel table lanes remain behind their named formula, retry, profile, or
  execution prerequisites.
- Checkpoint upload, approved isolated pool, budget/image identity, and raw
  evidence readback remain unavailable as recorded in
  [`../../problems/ghs-strong-rho-measurement-admission.md`](../../problems/ghs-strong-rho-measurement-admission.md).
- The first retained R17/R36 JSONL is internally consistent but identifies a
  worktree rather than a commit. A commit/source/binary-bound rerun is required
  before promotion.

## Promotion sequence

1. Bind the composed source and binary to a pushed commit and rerun the native
   record/analyzer/corruption gates.
2. Restore checkpoint and isolated-host admission; pass A/A calibration.
3. Measure baseline, R17, R36, R64, and the selected composition with
   interleaved A/B and untouched holdout targets.
4. Run the matched GHS IC/rho grid and the live `--require-all` ECBench solver
   inventory with fastest equal-output external controls.
5. Replay every recovered scalar, independently recompute headlines, obtain
   validation/PCC standing, close benchmark records, and only then update the
   canonical leaderboard.

## Reproduction and evidence

Start with [`PROTOCOL.md`](PROTOCOL.md). Each range report links its native
source and exact receipt:

- [`ranges/R00_R16.md`](ranges/R00_R16.md)
- [`ranges/R17_R33.md`](ranges/R17_R33.md)
- [`ranges/R34_R50.md`](ranges/R34_R50.md)
- [`ranges/R51_R66.md`](ranges/R51_R66.md)
- [`ranges/R67_R83.md`](ranges/R67_R83.md)
- [`ranges/R84_R99.md`](ranges/R84_R99.md)

The research-KB executable and Conductor were unavailable at campaign start.
The fallback review used the repository literature inventory, the reviewed
Round C integration, the measurement-admission problem, and the collaborative
Pollard-rho design. No novelty or absence inference is based on tool
unavailability.

Canonical leaderboard and frontier graphs were checked and intentionally left
unchanged: this round produced no admitted performance record. This report,
its editable diagram, its recorded-counter figure, and the rendered PDF are
the visual record for the implementation/search wave.
