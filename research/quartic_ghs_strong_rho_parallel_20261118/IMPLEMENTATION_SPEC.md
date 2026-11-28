# GHS strong-rho and matched frontier implementation specification

Status: implementation specification; prospective work is not implemented or
measured unless the status tables below say otherwise. Numeric observations
from the producer-side controls remain `[pcc:unverified]`.

Date: 2026-11-19

Tracking branch: `codex/ghs-rho-parallel-20261118`

Tracking PR: `#4757`

## 1. Request, deliverables, and evidence status

The requested outcome is a concrete plan for what must be implemented and
added before the optimized GHS strong-rho comparator can support a new
ECBench frontier decision. This document is the implementation contract for
that work. It does not replace the prospective protocol that must be frozen
before a measured run.

| Requirement | Repository deliverable | Status on 2026-11-19 |
|:--|:--|:--|
| Identify every remaining rho optimization | Work packages R1-R9 | Specified; not implemented |
| Say exactly where and how to implement it | Source/API/test changes under each work package | Specified |
| Preserve correctness and complete accounting | Deterministic, differential, trajectory, IR, and failure gates | Specified |
| Measure the optimized comparator fairly | Measurement work packages M0-M4 | Blocked on admission; not run |
| Compare all live ECBench candidates | Frontier campaign F0-F5 | Specified; not run |
| Update the leaderboard continuously | Publication rules P0-P3 | No row currently eligible |
| Check the plan into a PR | This file and its links | Implemented in PR #4757 |

The current composed source is
`research/quartic_ghs_strong_rho_20261114/strong_rho.rs`, SHA-256
`724f1c433b465818da6b0ee6d0ad7deb01434c6bb114e87dae4e304fce4bd340`.
The bound control binary was built from code revision `d85752a9a0` and has
SHA-256
`8ce42ed8f774b9f1bd16ce455cf3eef2f6b6de2cf84b12d92b1fd92d117f3344`.
Those are producer records, not PCC-verified execution claims
`[pcc:unverified]`.

On the public deterministic `q=223`, seed 1, target 0, `strong-b8` control,
the composed source retained the archived 1,487-step trajectory and changed
the complete counters from 149,558 to 110,435 base-field multiplications and
from 2,619 to 782 base-field inversions `[pcc:unverified]`. This observation
prioritizes the work below. It is not a wall-clock or frontier result.

## 2. Scope and non-goals

### In scope

1. Optimize the restricted-family strong Pollard-rho implementation without
   weakening its collision, cycle, distinguished-point, or verification
   semantics.
2. Keep an exactly counted arm and a counter-free production arm in the same
   source, with explicit authority fields.
3. Expose the implementation through ECBench with cold, one-target phase
   accounting.
4. Run a frozen matched IC-versus-rho panel on the restricted GHS-weak
   `E_0/F_{q^3}` family after admission.
5. Re-run the broader ECBench frontier across rho, IC, F4/F5/F6, SAT-family,
   FES/Gray-code, and root-finding backends in their valid domains.
6. Publish only results that pass independent replay, validation, PCC, and
   leaderboard gates.

### Out of scope for this implementation sequence

- Treating the `q=223` control as throughput evidence.
- Generalizing a restricted GHS-weak result to random prime, random binary,
  Koblitz, or standard curves.
- Replacing the cold one-target comparison with an amortized or multi-target
  average.
- Calling parallel rho a reduction in expected total work. It is a latency
  implementation whose wall time, total CPU, overshoot, and total steps must
  all be reported.
- Claiming a Frobenius or order-three quotient without an explicit same-curve
  subgroup map and eigenvalue certificate.
- Promoting a solver-local F4/F5/F6 or root-finder reduction as an ECDLP
  reduction without complete pipeline accounting.

These are scope rules for prospective work `[pcc:unverified]`.

## 3. Source and artifact layout

### Existing files that remain authoritative inputs

| Path | Role |
|:--|:--|
| `research/quartic_ghs_strong_rho_20261114/strong_rho.rs` | counted and counter-free rho kernels |
| `research/quartic_ghs_strong_rho_20261114/main.rs` | arm parsing, execution, JSON records |
| `research/quartic_ghs_strong_rho_20261114/analyze.py` | fail-closed count/model analysis |
| `research/quartic_ghs_strong_rho_20261114/test_analyze.py` | analyzer authority and schema tests |
| `research/quartic_ghs_strong_rho_20261114/round-c-integration/` | composed deterministic evidence |
| `research/ecbench_e2e_frontier_20261031/` | previous cross-method frontier report and literature gate |
| `docs/ecbench/` | canonical measurement, method, replay, and comparison contracts |
| `docs/ic/LEADERBOARD.md` and twins | validated publication surfaces |

### Files to add in implementation PRs

Each implementation lane gets a prospective protocol and a never-overwritten
evidence directory before candidate output:

```text
research/quartic_ghs_strong_rho_20261114/
  ROUND_D_MUL5.md
  ROUND_D_SETUP_REUSE.md
  ROUND_D_COUNTERFREE.md
  ROUND_D_INV_SIGN.md
  round-d-mul5/
  round-d-setup-reuse/
  round-d-counterfree/
  round-d-inv-sign/
  round-e-profile/
  round-f-matched-grid/
```

Every result directory must contain the exact command transcript, source and
binary hashes, compiler/host capsule, raw JSONL, stderr, exit status,
`SHA256SUMS`, independent validator input, and a result file that retains
failures and negative arms.

The ECBench integration will add:

```text
docs/ecbench/specs/ghs-strong-rho-development.json
docs/ecbench/specs/ghs-strong-rho-holdout.json
docs/ecbench/specs/ghs-weak-matched-frontier.json
research/ecbench_ghs_strong_rho_<date>/
```

The exact date suffix is selected when the preregistration lands. Specs must
use canonical ICV1 curve identities rather than text aliases.

## 4. Dependency order

```text
R1 multiplication ─┐
R2 setup reuse ────┼─> C0 composed deterministic control ─> M1 profile/A-A
R3 counter removal ┤                                      │
R4 inversion ──────┤                                      v
R5 lazy sign ──────┘                              M2 frozen retune
                                                              │
M0 admission ─────────────────────────────────────────────────┤
                                                              v
                                             M3 matched GHS grid
                                                              │
                                       M4 replay/validation/PCC
                                                              │
                                                              v
                                           F0-F5 full frontier rerun
                                                              │
                                                              v
                                                P0-P3 publication
```

R6-R9 start only after M1 identifies their target phase. R1-R5 may be developed
independently, but only C0's recomposed binary may supply the next baseline.
Standalone deltas must never be added together.

## 5. R1 — five-product general `F_{q^3}` multiplication

### Goal

Replace the current six-product cubic Karatsuba multiplication with a
five-evaluation product for operands

```text
A(z) = a0 + a1*z + a2*z^2
B(z) = b0 + b1*z + b2*z^2
```

before reduction by `z^3 = c`. The existing six-product implementation stays
available as the differential oracle and measured baseline.

### Algebra to implement

Evaluate the product polynomial at `0`, `1`, `-1`, `2`, and infinity:

```text
v0   = a0*b0
v1   = (a0+a1+a2) * (b0+b1+b2)
vm1  = (a0-a1+a2) * (b0-b1+b2)
v2   = (a0+2*a1+4*a2) * (b0+2*b1+4*b2)
vinf = a2*b2
```

For `A*B = d0 + d1*z + d2*z^2 + d3*z^3 + d4*z^4`, interpolate:

```text
d0   = v0
d4   = vinf
even = (v1 + vm1) / 2
d2   = even - d0 - d4
odd  = (v1 - vm1) / 2
at2  = (v2 - d0 - 4*d2 - 16*d4) / 2
d3   = (at2 - odd) / 3
d1   = odd - d3
```

Reduce once:

```text
r0 = d0 + c*d3
r1 = d1 + c*d4
r2 = d2
```

The admitted family has prime `q = 7 mod 12`, so two and three are invertible.
The implementation must nevertheless check `gcd(q,6)=1` and fall back to the
six-product kernel outside that domain.

### Exact small divisions

Do not hide interpolation work inside an uncounted modular inverse.

- `div2(x)`: for canonical `x`, use `x/2` when even and `(x+q)/2` when odd.
- `div3(x)`: choose `k in {0,1,2}` for which `x+k*q` is divisible by three,
  then use the exact integer quotient.
- Add `fq_small_div` to `FieldCounts`. Increment it once for every logical
  `/2` or `/3`, even if the compiler strength-reduces the integer division.
- Keep all modular additions/subtractions in `fq_add`; keep the five
  evaluation products and any large-`c` reduction products in `fq_mul`.
- Export `fq_small_div` as separately unpriced work so the section-5a band
  and wall measurements can price it. It must not silently receive a zero
  cost.

### Source changes

In `strong_rho.rs`:

1. Rename the current body to `mul_raw_karatsuba6`.
2. Add `mul_raw_eval5` with the formulas above.
3. Add an `F3MulKernel` enum with `Karatsuba6` and `Eval5`.
4. Carry the kernel through `StrongOpts`; do not use a global mutable switch.
5. Keep `FastF3::mul` responsible for exactly one `f3_mul` increment, then
   dispatch to the selected raw kernel.
6. Add `fdiv2` and `fdiv3` helpers with explicit `fq_small_div` accounting.
7. Preserve the `COUNT=false` specialization so field counters compile away.

In `main.rs`:

- Add diagnostic arm suffixes `-mul6` and `-mul5`.
- Keep the current default bound to `mul6` until the measured promotion gate
  passes; changing the default is a separate reviewed decision.
- Reject duplicated, unknown, or contradictory suffixes.

In `analyze.py` and its tests:

- Parse and aggregate `fq_small_div` per phase.
- Refuse modeled comparisons that omit it or mark it authoritative when the
  producer says counts are non-authoritative.

### Correctness gates

1. Compare `eval5` and `karatsuba6` against a separately written nine-product
   schoolbook oracle for every coefficient tuple over small primes through a
   tractable exhaustive bound. The oracle must not call either candidate.
2. For every prime `5 <= q <= 257`, test multiple `c` values, including
   reducible quotient rings; multiplication equality does not depend on
   irreducibility.
3. Test boundary operands containing `0`, `1`, `q/2`, `q-2`, and `q-1`, then
   at least 10,000 deterministic random operand pairs per production-boundary
   modulus.
4. Check distributivity, identity, zero, commutativity, squaring agreement,
   and `a * inv(a) = 1` on nonzero field inputs.
5. Run the planted-log selftest at widths 1/4/8/16/32.
6. On q223, require identical answer, walk trajectory, DP sequence, retries,
   cycles, and native counts between `mul5` and `mul6`. Field counts may differ
   only by the preregistered multiplication/addition/small-division vector.

### Promotion and rejection rules

- Deterministic acceptance: all correctness gates pass and the exact counter
  delta matches the formulas.
- Throughput acceptance: the source-bound counter-free `mul5` arm is faster
  outside the A/A interval on the isolated ABAB panel, without a regression
  in complete cold E2E cost.
- Reject or retain as diagnostic if interpolation work removes the expected
  wall benefit, code generation regresses, or any trajectory/correctness gate
  fails. Preserve the negative records. These are prospective acceptance
  rules `[pcc:unverified]`.

The algebraic ceiling on the current q223 record is one fewer `fq_mul` for
each of 13,551 recorded `f3_mul` calls before interpolation costs
`[pcc:unverified]`. That ceiling is not a predicted elapsed-time speedup.

## 6. R2 — reuse the joint setup table for initial walkers

### Existing behavior

`initial_state` cold-builds a 16-entry width-two table and uses
`batch_window2_mul` for the 32 jump points. It then calls `new_walker`
sequentially. Each walker evaluates `[a]G` and `[b]Q` separately and adds the
results. The corrected implementation deliberately retained this path to
preserve retry order and charge every attempted point.

### Implementation

1. Factor the Horner logic in `batch_window2_mul` into a reusable core.
2. Add allocation-free `window2_mul_one(table, a, b, scratch)` for one exact
   simultaneous multiplication using the already cold-charged table.
3. Add `new_walker_with_table` that:
   - draws `(a,b)` in the same order as `new_walker`;
   - evaluates the joint multiplication;
   - retries immediately on infinity before drawing the next lane;
   - calls the same `canon` and initializes every `Walker` field identically.
4. Reuse one setup scratch allocation; do not allocate per scalar bit or per
   retry.
5. Keep `new_walker` as the in-source control until the measured gate passes.
6. Never restore counters or RNG state after tentative work. Every attempted
   operation stays charged.

The single-lane path intentionally preserves sequential RNG semantics. A
future all-walker batch is a separate candidate because drawing all lanes
before retrying an infinity changes the baseline RNG order.

### Required controls

- Exhaustive small scalar pairs against separate `[a]G + [b]Q` evaluation.
- Random scalar pairs on every public panel curve.
- Exact equality of coefficient draws, retry order, points, canonical signs,
  walker fields, and post-setup RNG state.
- Exact equality of the complete walk after setup.
- Complete setup counter vector, including table construction and discarded
  retry work.
- Allocation diagnostic showing no per-walker/per-bit heap growth.

The q223 composition retains 511 cubic setup inversions, 33 of them batch
inversions; the remaining 478 identify the count-level target
`[pcc:unverified]`. No required percentage reduction is assumed. The
candidate is retained only if its complete setup cost is componentwise no
worse under the preregistered pricing band or if a measured trade is reported
explicitly.

## 7. R3 — fully counter-free production mode

The current `COUNT=false` specialization removes `FieldCounts` updates but
still mutates `NativeCounts`. The production arm therefore has avoidable
diagnostic dependencies and cannot yet demonstrate a fully counter-free hot
loop.

### Implementation

1. Replace direct `NativeCounts` field mutations with inline methods on a
   `NativeCounter<const COUNT: bool>` wrapper.
2. Monomorphize counted and uncounted implementations from one mathematical
   body; do not maintain a forked walk.
3. Split record authority into:
   - `field_count_authoritative`;
   - `native_count_authoritative`;
   - retained compatibility `count_authoritative`, true only when both are
     authoritative.
4. Counter-free records emit zero diagnostic counts and false authority.
5. The analyzer must require the JSON boolean value `true`, not truthiness,
   before using either count family.
6. Extend the optimized-IR gate to reject field and native counter symbols,
   loads, stores, increments, and formatting paths in the counter-free walk.

### Gates

- Counted output is byte-compatible apart from prospectively declared schema
  fields.
- Counted and counter-free arms return identical mathematical output and
  trajectory fields for every test arm.
- The IR checker independently finds zero counter SSA markers in the
  counter-free arithmetic and walk wrappers.
- Assembly size and instruction counts are retained as diagnostics; only an
  isolated paired wall run may establish throughput.

## 8. R4 — division-free base-field and scalar inversion

The current research binary calls `common::inv_mod_u64`. The repository's
`src/cryptanalysis/prime_fast.rs::inv_mod_u64` implements the binary extended
Euclidean algorithm without divisions and is the first candidate to compare.

### Implementation sequence

1. Import or locally reproduce the reviewed helper with a source-revision
   comment; avoid a fragile relative include from production code.
2. Keep `InvKernel::{Current, BinaryXgcd}` selectable in `StrongOpts`.
3. Apply the candidate separately to:
   - base-field norm inversion inside `FastF3::inv`;
   - subgroup scalar inversion in collision recovery.
4. Do not combine those two switches in the first diagnostic; their input
   distributions and call frequencies differ.
5. Record iteration counts, branches, and worst-case loop length as native
   diagnostics in counted mode.

### Gates

- Exhaustive equality for every nonzero input for all small prime moduli.
- Boundary/random equality through the full `q < 2^31` production range and
  every registered subgroup order in the matched panel.
- Same inverses, points, collision scalar, and recovered logarithm.
- Profile first; then A/A and interleaved A/B. This is a wall-only candidate
  because both algorithms implement the same abstract `fq_inv` count.

## 9. R5 — lazy-sign negation quotient

This lead must start with a state-equivalence proof, not a code shortcut.
`canon` currently negates `y`, `a`, and `b` eagerly. Partitioning and DP keys
use `x`, but point addition, cycle state, and collision recovery consume the
sign.

### Proof obligations

1. Define a signed state `(x, y, a, b, s)` whose materialized point is
   `s*(x,y)` and whose coefficients represent the same group element.
2. Show that partition choice and DP classification are invariant under `s`.
3. Define signed-jump addition that produces the same quotient transition as
   eager canonicalization.
4. Define cycle equality and minimum-state selection on quotient states.
5. Define collision equations for equal and opposite representatives and
   check the recovered scalar against `[k]G = Q`.
6. Show how infinity, sterile collisions, look-ahead retries, and cycle
   escapes preserve their existing semantics.

### Implementation and gates

- Add a diagnostic `lazy-sign` arm; do not change the default.
- Count deferred and materialized negations separately.
- Require exact x/partition/DP trajectory, collision scalar, and final answer
  against eager canonicalization on the deterministic panel.
- If the full point trajectory intentionally changes, stop using exact
  trajectory as the promotion gate and preregister a statistical rho-constant
  panel before output. Do not decide this after seeing the result.
- A smaller negation counter without an isolated wall gain is a component
  diagnostic, not an E2E result `[pcc:unverified]`.

## 10. R6 — retune batch width and partition count

Retuning begins only after R1-R5 are composed. The q223 width rows are
sensitivity controls and cannot choose a winner.

### Development grid

- Batch widths: `1, 4, 8, 16, 32`.
- Partition counts: `16, 32, 64, 128`.
- At least three development sizes spanning the low, middle, and high
  runnable regions.
- At least three preregistered seeds per size.
- Counted mode for complete operation vectors; counter-free mode for wall
  measurements.

Use a sequential elimination design rather than silently dropping expensive
cells:

1. Correctness/count screen for the full width-by-partition grid.
2. Source-bound profile on one frozen cell.
3. A/A noise floor for every host class.
4. Interleaved ABAB development measurements.
5. Freeze one primary and at most one sensitivity configuration before the
   holdout panel.

Report steps, `steps/sqrt(ell)`, DPs, restarts, look-ahead retries, cycles,
cycle additions, total field/native work, peak RSS, elapsed time, total CPU,
and phase splits. A configuration with fewer steps but more total cost is not
ranked as lower cost.

## 11. R7 — SIMD/structure-of-arrays field kernel

Start only if M1 attributes material time to base-field lane arithmetic.

1. Add a structure-of-arrays batch representation for the three cubic
   coefficients across walkers.
2. Screen scalar u32, portable SIMD, and architecture-specific vector paths
   behind one trait and a scalar oracle.
3. Preserve the exact reducer domain `q < 2^31`; widen products before
   reduction.
4. Vectorize complete batches and handle partial/retry lanes with explicit
   masks. Never read uninitialized lanes.
5. Keep counted work defined in scalar abstract operations. Record vector
   instructions separately; do not reinterpret one vector instruction as one
   field operation.
6. Require exact lane results, trajectory, sanitizer/UB checks, scalar
   fallback parity, and source-bound host-feature records.

## 12. R8 — parallel distinguished-point rho

Use independent worker batches with the same walk function and a shared DP
table. Follow the repository's van Oorschot-Wiener design rather than
partitioning the group into incompatible walk functions.

### Required design

- Deterministically derive disjoint walker-start indices from the job id.
- Keep the jump table and transition function identical across workers.
- Shard or lock the DP table by key prefix; measure contention and table
  memory.
- Check every collision candidate and final scalar independently.
- Stop all workers on a checked solution while retaining work performed
  during detection/stop latency as overshoot.
- Make worker count, CPU affinity, NUMA placement, DP bits, table policy, and
  queue depth part of the run identity.

### Required reporting

Wall latency, total CPU seconds, total steps, useful steps, overshoot, DP
count, sterile collisions, table operations, peak memory, and scaling
efficiency are all mandatory. The headline one-target comparison remains the
single resource envelope frozen by the protocol; worker-count scaling is a
separate table.

## 13. R9 — projective or x-only redesign screen

Do not implement a full rewrite before formula accounting.

1. Enumerate complete projective/mixed-addition formulas for the actual
   generalized curve model with nonzero `a2` and `a4`.
2. Price partitioning: the walk needs a stable x-derived partition every
   step. Include any normalization or projective hash construction.
3. For an x-only/Kummer state, provide a functional transition, coefficient
   update, collision equation, and recovery proof.
4. Compare formula-level operation vectors with the composed affine batch
   kernel across the pricing band.
5. Implement only a candidate that survives that screen. A formula count is
   still not a throughput result.

## 14. C0 — composition and independent integration review

Composition order is R1 arithmetic, R2 setup, R3 counter removal, then any
accepted R4/R5 candidate. After each merge:

1. Build a source-hash-bound binary.
2. Run the standalone algebra/curve/planted-log selftest.
3. Run counted and counter-free arms at widths 1/4/8/16/32.
4. Recompute the q223 JSON with an independent jq/Python validator.
5. Check every expected phase delta from raw counters.
6. Run the analyzer adversarial tests and optimized-IR gate.
7. Have a reviewer inspect hidden work, authority booleans, retry accounting,
   rare branches, and whether standalone deltas were improperly summed.

The composed binary becomes the baseline only after all checks pass. Rejected
lanes retain their code diff, raw output, and reason for exclusion in their
own evidence directory.

## 15. M0 — restore execution admission

The frozen matched panel cannot launch until every item in
`problems/ghs-strong-rho-measurement-admission.md` is satisfied.

Required before build or job submission:

- authorized AWS identity and current daily/monthly budget readbacks;
- at least the preregistered unreserved experiment headroom;
- all-tenant pool, node, namespace-quota, and active-job receipt;
- passing checkpoint push/gate and exact revision acceptance;
- digest-pinned image containing the composed source;
- authorized raw-object retrieval and checksum verification.

An access failure records an infrastructure outcome. It does not count as a
failed algorithm experiment or permit a smaller substitute to discharge the
panel.

## 16. M1 — profile and host-calibration round

For the counted and counter-free composed binaries:

1. Pin source, compiler, flags, CPU model/microcode, kernel, governor, NUMA,
   affinity, and environment.
2. Run A/A until the frozen noise criterion is met; do not choose the sample
   count after seeing candidate variance.
3. Capture whole-run and online-only profiles with setup/walk/verify markers.
4. Record cycles/instructions, cache misses, branch misses, allocations, RSS,
   elapsed time, CPU time, and operation counters where the host supports
   them.
5. If PMU access is unavailable, mark those fields unavailable rather than
   substituting development-clock values.
6. Use the profile to decide whether R6, R7, R8, or R9 is the next target.

No profile percentage establishes lower E2E cost; it identifies where to test
for one `[pcc:unverified]`.

## 17. M2 — frozen retune and holdout

The development grid selects parameters; a distinct, preregistered holdout
confirms them.

- Development and holdout curve/seed identities must be disjoint.
- A/A, A/B, B/A, and repeated A/B ordering is frozen to expose drift.
- At least five interleaved paired observations per admitted development cell;
  the final target count follows `docs/ic/SUCCESS_CRITERIA.md` for a frontier
  claim.
- Report all cells, including timeouts, OOMs, invalid answers, interrupted
  jobs, and regressions.
- Freeze the primary width/partition/kernel before running the holdout.
- A failed holdout rejects promotion; it does not trigger tuning on the
  holdout.

## 18. M3 — matched restricted-family frontier grid

Retain the preregistered family and exact size grid:

| k | q | primary repetitions per seed |
|--:|--:|--:|
| 12 | 4,051 | 5 |
| 13 | 8,191 | 5 |
| 14 | 16,363 | 5 |
| 15 | 32,719 | 5 |
| 16 | 65,479 | 5 |
| 17 | 131,071 | 5 |
| 18 | 262,111 | 5 |
| 19 | 524,287 | 3 |
| 20 | 1,048,507 | 3 |
| 21 | 2,097,091 | 3 |
| 22 | 4,194,271 | 3 |

Each size uses instance seeds 1, 2, and 3. The target policy remains
`ic-repeat`: each rho run reconstructs and solves the exact target used by the
frozen IC unit, with an independent deterministic rho walk seed. The primary
arm is the composed `strong-b8`. One-repetition `strong-b4`, `strong-b16`, and
legacy `old-neg` diagnostics run only at q=4,051, 65,479, and 262,111; they
cannot replace the primary after output is seen. The historical IC candidate
set remains `B0`, `S`, `L0`, `L1`, `L`, `D`, `DL1`, and `DL`; the lowest
complete mean at each size is selected under the frozen rule, not by new
post-result tuning `[pcc:unverified]`.

The execution split remains:

| split | q values | active deadline |
|:--|:--|--:|
| low | 4,051 through 262,111 | 3,600 s |
| mid | 524,287 and 1,048,507 | 3,600 s |
| high | 2,097,091 | 7,200 s |
| top | 4,194,271 | 10,800 s |

Each job requests at most 4 vCPU and 4 GiB, with zero automatic retries. Run
low+mid as wave one and high+top as wave two after both first-wave jobs are
terminal. The round retains its USD 5 all-attempt ceiling and one documented
manual retry only for infrastructure failure when the same ceiling still
holds. These are frozen limits, not authorization to launch.

Both methods must include setup, walk/relation collection, solving, recovery,
and verification in the cold interval. Online-only and reusable setup are
reported separately. The primary cost is the complete counted pricing band;
wall time is reported as practicality evidence under the matched resource
envelope.

Required per phase:

| Rho | IC |
|:--|:--|
| jump/table and walker setup | factor-base and solver setup |
| walk and look-ahead work | relation generation |
| DP/collision work | relation filtering/deduplication |
| scalar recovery | linear algebra/Gröbner/SAT/FES solving |
| independent `[k]G=Q` check | target recovery and independent check |

The comparison reports the band at 0, 0.1, and 1 GAE per unpriced operation,
plus calibrated weights where available. The matched rho receives every
usable same-curve symmetry.

The counted unit remains:

```text
M-eq = fq_mul + r_inv*fq_inv + r_lmul*mod_ell_mul
S    = cold_M-eq / sqrt(ell)
GAE  = 56 + r_inv
cold_M-eq(w) = cold_M-eq + w*GAE*band_ops
```

At each `(q, seed)`, `r_inv` and `r_lmul` come from 600 interleaved calibration
triples using the exact frozen kernels. The median over the three seed
calibrations prices both IC and rho at that size. Confidence intervals use
10,000 deterministic clustered-bootstrap resamples of the three seed-level
units; repetitions are averaged within a seed first.

Admission and conclusions remain conjunctive:

1. Every primary run has the expected source, curve, seed, target, width, and
   successful independent scalar check.
2. The primary mean `steps/sqrt(ell)` lies in `[0.75, 1.15]` at every size
   admitted to an IC/rho conclusion.
3. Counts, phase sums, repetitions, schemas, manifests, and the duplicate
   small-cell replay pass the independent validator.
4. The restricted-family exception is supported on the measured grid only if
   every size is admitted and the 95% upper bound of IC/`strong-b8` is below
   one at all three band weights. Missing sizes make the conclusion limited;
   an interval crossing one rejects or leaves it unresolved under the frozen
   rule.
5. The rho-engineering question is separate: `strong-b8` must have lower cold
   counted cost than `old-neg` on every diagnostic cell and band weight. It
   cannot by itself establish an IC result.

## 19. M4 — replay, validation, and proof-carrying claims

Before publication:

1. Seal raw data and manifests.
2. Independently replay selected and adversarial runs, including every
   recovered scalar.
3. Recompute headline values from raw records with a validator not written by
   the producer.
4. Produce `VALIDATION.json` naming replay certificate, validator report, and
   matched rho reference.
5. Create PCC claim records for the exact scope and proof hashes.
6. Wait for CI-main signed standing; producer-authored booleans do not verify
   a claim.
7. Run a second host-class audit where the claim tier requires it.

## 20. F0-F5 — comprehensive ECBench frontier rerun

The GHS matched panel answers one restricted-family question. The complete
frontier rerun must audit and execute the live method registry rather than
hard-code an old method count.

### F0. Inventory and coverage manifest

Run `ecbench optimize inventory --json` at the source revision and freeze:

- every registered method id and parameter role;
- every system solver/backend;
- every root-finding engine;
- every factor-base builder and symmetry mode;
- every curve-family compatibility rule;
- external reference availability.

Run `ecbench optimize audit --development ... --holdout ... --require-all`.
Any uncovered method remains explicitly missing; it is not silently omitted.

### F1. Curve families

Use separate, identity-pinned panels for:

1. generic prime-field curves;
2. random binary curves;
3. Koblitz curves with the strongest registered Frobenius/negation rho;
4. the restricted GHS-weak `E_0/F_{q^3}` family;
5. any transfer/Jacobian family only under its own typed route and cost model.

Never cross-rank different families or units as though they were the same
instance.

### F2. Algorithm axes

The campaign must include every compatible combination in the frozen
registry, including where present:

- rho: plain, negation, Frobenius/automorphism fold, optimized strong rho,
  batch widths, and parallel DP diagnostics;
- IC factor bases: established best base plus every admitted shape/degree
  candidate and exact quotient/symmetry choice;
- algebraic solvers: F4, Faugere-Lachartre F4, F5, F6, and registered hybrids;
- Boolean solvers: SAT/CDCL and WD-SAT only where their soundness controls
  pass;
- enumeration: FES, Gray-code, guess-and-determine, and registered table/MITM
  engines;
- root finding: the established baseline and the new root-finding engine;
- linear algebra and relation filters as separately identified backends.

Historical WD-SAT results with the retained false-UNSAT conflict remain
excluded from promotion until the conflict is resolved and replayed.

### F3. Common record contract

Every execution records:

- canonical curve, subgroup, generator, and target identity;
- method/backend/factor-base/symmetry configuration;
- exact source, binary, input, factor-base, and container hashes;
- cold and online phase intervals;
- complete abstract operation vector and unpriced work;
- wall and CPU time, peak memory, status, exit code, and timeout/OOM data;
- recovered scalar and independent verification;
- resource envelope and isolation level;
- setup reuse count, which is one for the cold headline.

### F4. External references

- Compare Gröbner work with the fastest available same-input Magma/msolve
  reference; if Magma is unavailable, report the published result with its
  hardware and label it unreproduced.
- Compare SAT-family work with the fastest compatible registered external
  solver on the identical CNF/XOR instance and verify assignments.
- Compare root-finding work with the fastest compatible FLINT/NTL/Magma or
  registered reference on identical polynomials and verify the root set.
- Inventory external rho implementations that accept the exact curve/field;
  absence of a runnable one is recorded, not converted into a fastest claim.

### F5. Analysis

For each valid family and size, report:

- cold one-target ratio to the strongest matched rho;
- online-only ratio beside it;
- absolute complete cost and distance remaining;
- confidence interval and tail diagnostic;
- setup/walk-or-collection/solve/verify phase shares;
- memory and success/failure rate;
- external-tool ratio at component scope;
- whether the result is a component, pipeline, restricted-family, or general
  statement.

## 21. P0-P3 — publication and leaderboard rules

### P0. During development

Keep prospective protocols, raw nulls, failures, and deterministic controls in
their research directories. Do not change leaderboard rows for plans, toy
counts, profiles, or unvalidated wall measurements.

### P1. Measured component result

When a source-bound, independently checked component delta is large
enough to meet the repository publication rule, update the performance index
and regenerate the leaderboard twins in the same PR, even if Tables A-C do
not move. Label its component scope and distance to E2E.

### P2. E2E frontier row

Add or move a row only when the complete cold one-target evidence has:

- all phases and failures charged;
- the strongest matched rho on the same target/resource envelope;
- complete pricing band and calibration where applicable;
- replay certificate, validator report, and `VALIDATION.json`;
- PCC standing from CI-main;
- generator output reproduced without hand edits.

### P3. Restricted-family wording

Any winning GHS row must say that it applies only to the named GHS-weak
`E_0/F_{q^3}` family and does not establish a result for generic prime,
random-binary, Koblitz, prime-order extension-field, or standard curves.

## 22. Test matrix and required commands

Every implementation PR runs, at minimum:

```sh
rustfmt --edition 2021 research/quartic_ghs_strong_rho_20261114/main.rs \
  research/quartic_ghs_strong_rho_20261114/strong_rho.rs

cd research/quartic_ghs_strong_rho_20261114
rustc --edition 2021 -O main.rs -o /tmp/ghs-strong-rho
/tmp/ghs-strong-rho selftest
python3 -m unittest -v test_analyze.py
bash round-c-counterfree/verify_counterfree_ir.sh

cd ../..
cargo test --release --lib
git diff --check
python3 tools/pcc/pcc.py lint-prose --stdin < PR_BODY
```

The implementation adds lane-specific validators and q223 comparison commands
to this list. A repository-wide test failure already present on `main` is
recorded with its exact diagnostic and separately reproduced on the base; it
does not authorize suppressing the test.

The specification's five-evaluation algebra has a separate producer-side
schoolbook check:

```sh
python3 research/quartic_ghs_strong_rho_parallel_20261118/validate_mul5_formula.py
```

Its output is retained in `mul5-formula-validation.txt`. This check does not
exercise a Rust candidate or establish elapsed-time performance
`[pcc:unverified]`.

Before measurement, also run:

```sh
scripts/checkpoint.sh push
scripts/checkpoint.sh gate
cargo build --release --bin ecbench
target/release/ecbench plan --spec <frozen-spec>
target/release/ecbench optimize audit \
  --development <development-specs> \
  --holdout <holdout-specs> \
  --require-all
```

After a session:

```sh
target/release/ecbench verify --dir <session> --replay-all
target/release/ecbench compare --dir <session> --a <rho-arm> --b <candidate>
target/release/ecbench band --dir <session>
target/release/ecbench resources --dir <session>
```

Exact flags, hashes, seeds, resource limits, and output directories belong in
the prospective protocol; these templates are not launch authorization.

## 23. PR sequence

| PR | Scope | Completion gate |
|:--|:--|:--|
| A | This specification plus the reviewed Round C composition | Docs/tests pass; PR ready; no performance promotion |
| B | R1 five-product multiplication | differential algebra, q223 trajectory, counts, IR |
| C | R2 setup-table reuse and R3 native counter elimination | RNG/retry identity, full composition, zero counter IR |
| D | R4 inversion and R5 lazy-sign diagnostics | correctness plus isolated profile/A-B; retain losses |
| E | ECBench adapter, schemas, development/holdout specs | method inventory/audit and smoke replay |
| F | M1-M3 admitted measurements | complete raw sessions; no leaderboard edit yet |
| G | independent replay, PCC, report, leaderboard regeneration | all validation/publication gates pass |
| H | profile-selected R6-R9 follow-up | separate prospective protocol and evidence |

Do not combine an unreviewed algorithm change with the confirmatory holdout or
repair a failed holdout in the publication PR.

## 24. Definition of done

### Specification done

- Every work package has a source location, algorithm, counters, tests,
  acceptance rule, and failure disposition.
- Measurement and publication requirements preserve cold one-target and
  matched-rho accounting.
- The plan is committed in the tracking PR.

### Optimized comparator done

- Accepted R1-R5 changes compose in one counted/counter-free source.
- Deterministic controls and independent integration review pass.
- The counter-free binary is source-bound and profile-ready.

### Frontier remeasurement done

- The full frozen matched grid completes or every failed cell is retained.
- Both arms solve and independently verify the same targets.
- Complete costs, resources, uncertainty, replay, and validation exist.
- The resulting frontier statement is limited to the evidence's family and
  interval.

### Comprehensive ECBench frontier done

- The frozen live registry is covered or every missing compatible method is
  explicitly listed.
- External references are reproduced or labelled unavailable/published-only.
- Validated rows and generated leaderboard surfaces agree byte-for-byte with
  their source records.

## 25. Immediate next action

The expanded R00–R99 execution registry and its six parallel implementation
batches are tracked in
[`../quartic_ghs_r0_r99_20261119/`](../quartic_ghs_r0_r99_20261119/).
R17–R33 owns the selectable five-product diagnostic and independent
schoolbook oracle. Other batches may screen their disjoint leads in parallel,
but accepted mutations are recomposed one at a time against the frozen
baseline. In parallel at the project level, an authorized operator can resolve
M0; no measurement job should launch until its receipts pass.

## 26. Sources checked

Primary literature and boundaries were taken from
`research/ecbench_e2e_frontier_20261031/LITERATURE.md`, including Shoup's
generic-group lower bound, Gallant-Lambert-Vanstone symmetry treatment,
van Oorschot-Wiener parallel collision search, Faugere's F4/F5 papers,
Semaev summation polynomials, Gaudry small-dimension IC, and the
Faugere-Perret-Petit-Renault binary-field analysis.

Repository sources checked:

- `research/quartic_ghs_strong_rho_20261114/round-c-integration/RESULTS.md`;
- `research/quartic_ghs_strong_rho_parallel_20261118/NEXT_LEADS.md`;
- `research/quartic_ghs_strong_rho_20261114/ROUND_C.md` and
  `ROUND_C1_SETUP.md`;
- `research/ecbench_e2e_frontier_20261031/REPORT.md` and `LITERATURE.md`;
- `docs/ecbench/README.md`, `docs/POLLARD_COLLAB_DESIGN.md`,
  `docs/ic/SUCCESS_CRITERIA.md`, and `docs/VALIDATION_GATE.md`;
- `src/cryptanalysis/prime_fast.rs`.

The research-KB wrapper and Conductor executable were unavailable in this
runtime. No novelty or absence claim relies on that failure.
