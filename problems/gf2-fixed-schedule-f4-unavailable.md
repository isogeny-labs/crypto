# GF(2) fixed-schedule F4 replay that decides the Semaev shootout systems

- **Status:** `open`
- **Kind:** access/tool limit (Julia unreachable); implementation gap (no
  multi-step GF(2) F4 learn/apply in Rust); empirical bottleneck (the
  single-degree replay that does exist does not decide the grid instances)
- **Affected work:** arm `trace-f4` (and its baseline `julia-f4-warm`) of
  `research/groebner_solver_shootout_20261116/` (protocol
  `GB-SOLVER-SHOOTOUT-20261116`, claims C-trace-warm, C-trace-msolve,
  C-rho-trace); adapter `groebner_compare/native_worker.py`
- **Next action owner:** unassigned
- **Last updated:** 2026-11-17

## Problem

The shootout cannot yet measure a fixed-schedule F4 (learn the F4 schedule on
some systems of a cell, then apply it to the others) that actually returns the
roots of the binary Semaev systems, because the frozen instrument (Groebner.jl
learn/apply, `groebner_compare/worker.jl`) cannot run here and the only native
GF(2) replay in the repository replays a single fixed-degree Macaulay step,
which does not decide these systems within its size limits.

The protocol's `trace-f4` arm is "Groebner.jl learn/apply (worker.jl), learn
cost amortised per cell", marked UNAVAILABLE at freeze. On 2026-11-17 the owner
ruled that AWS is unusable and Magma unavailable, so the study is a local,
non-admitted diagnostic in a 4 vCPU container. The task was to wire a native
(Rust) GF(2) trace/replay F4 if one exists.

## Evidence and uncertainty

All of the following comes from commands run on 2026-11-17 in the session container.
Everything is `[pcc:unverified]`. None of it is a performance claim.

- **Observed: Julia is blocked.** The protocol records that Julia is absent and
  that its download host returns proxy 403. This was not re-tested here.
- **Observed: a native GF(2) replay exists, and it is wired.**
  - The code is `src/cryptanalysis/f4_trace.rs`, which is
    `koblitz_groebner::{matrix_f4_f2_record_counted, matrix_f4_f2_replay_checked}`.
    It is the Joux–Vitse record/replay of the dense degree-D Macaulay step.
    It records which (input, multiplier) rows survive elimination, merges
    several recordings, and replays only those rows on a new system. The
    replay carries an audit of 16 random combinations of the dropped rows.
  - It is exposed as arm `trace-f4` in `groebner_compare/native_worker.py`
    through `examples/gc_native_solve.rs` (ops `learn` and `replay`):
    1. If the replay is refused, the full kernel runs instead, and both are
       charged.
    2. Roots are read off the affine part of the reduced Macaulay basis.
    3. The learning wall time and CPU time come back from `learn()` and are
       amortised by `charge_learning()`.
  - Unit tests `groebner_compare/test_native_worker.py::TraceF4` pass on the
    tiny cells (m = 3, n = 5, l = 2; 9 to 15 unknowns; D = 4). There the root
    sets equal brute force.
  - **It is not the protocol's instrument.** It runs one Macaulay matrix at one
    degree. It is not a multi-step F4 schedule and not a reduced Gröbner basis.
- **Observed: on the grid's smallest cells it does not decide.** The setup was
  seed 20261117, targets 0 to 2, training seed 20261118 with indices 0 to 2,
  and the binary from `groebner_compare/build_native.sh`.
  - **D = 4.** Learning succeeded on `m3-plain-n11` (23 unknowns),
    `m3-symmetrised-e-n11` (21) and `m4-plain-n9` (30). Every replayed or
    fallback basis had 0 to 2 linear elements, which leaves more than 20 free
    unknowns. All 9 target calls returned `unknown`.
  - **D = 4, replay refusals.** 3 of the 9 calls were refused with `Rank`
    (survivors dependent on the new target) and ran the full kernel instead.
    All 3 `Rank` refusals were in `m3-plain-n11` and `m3-symmetrised-e-n11`.
    `m4-plain-n9` was accepted 3 of 3.
  - **Size limits (default `F4_F2_MAX_ROWS` and column caps).**
    `m3-symmetrised-n11` (33 unknowns) exceeds them at D = 4.
    `m3-plain-n11` and `m4-plain-n9` exceed them at D = 5 and D = 6.
    `m3-symmetrised-e-n11` exceeds them at D = 6.
  - **e-only cell at D = 5.** Learning took 7.43 s wall. The replayed matrix
    had 15,883 rows × 27,434 columns. The call took 1.52 s wall and peaked at
    459 MB RSS (sampled). The basis had 0 linear elements, so the result was
    `unknown`.
  - On the same instances, inherited-f4 and msolve `-g 2` decide every target.
    - On the plain and symmetrised cells (6 to 84 roots), inherited-f4's root
      count equals msolve's quotient dimension on 9 of 9 targets. Its root set
      equals msolve's basis roots on 8 of 8 targets (one target, with 84 roots,
      was not extracted).
    - On the e-only cell (1,024 to 1,025 roots), inherited-f4's root set equals
      the exhaustive Rust enumeration on 3 of 3 targets.
- **Observed: supports differ across targets.** Five seeded targets per cell
  were compared for m3-plain n11, m3-symmetrised n11, m3-symmetrised-e n11,
  m4-plain n9 and m3-plain n17.
  - No target's per-equation monomial supports equal target 0's (0 of 4 in
    every cell).
  - The per-equation degrees are identical (4 of 4).
  - The union of monomials is identical, for example 340 for m3-plain n11.

  The target abscissa x_R enters the coefficients, so a schedule learned on one
  target names products whose support changes on the next. Groebner.jl's
  learn/apply assumes a fixed support. `f4_trace` fingerprints only the degrees,
  which is why it accepts these systems and then sometimes refuses on rank.
- **Derived or proved:** not established. Whether a degree exists below the
  size caps at which the affine part pins the roots is not proved.
- **Suspected explanation (conjecture).** The single-degree Macaulay basis at
  D ≤ 5 does not reach the degree at which linear relations appear. msolve's
  F4 needs higher-degree steps, and inherited-f4 compensates by splitting. In
  the e-only cell the 1,024 Boolean solutions mostly fail re-addition, so no
  linear-part extraction can pin them. A replayed root step followed by
  inherited-f4's split tree would distinguish these explanations.
- **Missing capability.**
  1. Any one of: a Julia runtime with Groebner.jl, a Rust multi-step GF(2) F4
     learn/apply, or a replayed root step feeding inherited-f4's tree.
  2. Support-tolerant schedule matching. Products are keyed by (input,
     multiplier) and tolerate support changes. That is already true of
     `f4_trace`, but not of Groebner.jl's trace.
- **What this evidence does not establish.** It does not show that
  fixed-schedule F4 is slower or faster than any arm. It also does not show
  that Groebner.jl learn/apply would fail on these systems. These are
  development timings on a shared VM.

## What would unblock it

Any one of the following.

- [ ] Groebner.jl learn/apply runs (a Julia runtime is reachable) and
  `worker.jl` decides the root sets of the smoke cells. Those root sets must
  equal msolve `-g 2` plus `certificate.basis_roots`, or exhaustive search for
  24 unknowns or fewer.
- [ ] A native GF(2) learn/apply F4 is added. It records the full F4 pair and
  degree schedule over several training systems and replays it. On the four
  smallest grid cells it returns root sets equal to the reference on every
  target, with the learning cost reported.
- [ ] Cheapest path in this repository: a `trace-f4-tree` variant. The replayed
  root reduction (`matrix_f4_f2_replay_checked`) becomes the root node of
  `koblitz_groebner::solve_boolean_system`'s inherited tree, so the children
  specialise the replayed basis. Note that this arm measures "replay plus
  inherited-f4", not a pure fixed-schedule F4.

Meeting any one of these makes `trace-f4` runnable. The study would still need
the warm baseline (`julia-f4-warm` or the native equivalent), and the
comparative claims remain preregistered per protocol amendment.

## Next discriminating step

- **Action:** implement the cheapest path above as a new op of
  `examples/gc_native_solve.rs`, keeping `native_worker.py`'s interface.
- **Inputs and controls:** the same seeded smoke cells. The reference root sets
  come from msolve `-g 2` and exhaustive search. The positive control is the
  planted root. Report the replay acceptance rate.
- **Execution limits and stop condition:** local container only, smoke cells
  only, 600 s per call.
- **Outcomes and next actions:**
  - It decides every smoke target with root sets equal to the reference: the
    arm can enter the local diagnostic.
  - It still returns `unknown`: record the obstruction and keep the arm
    UNAVAILABLE.
- **Recheck trigger:** Julia becomes reachable, or a multi-step GF(2)
  learn/apply lands.

## Resolution and history

- 2026-11-17: opened. `trace-f4` (single-degree Joux–Vitse replay) is wired
  and unit-tested, but on the grid it returns `unknown` (see above). The
  protocol arm (Groebner.jl learn/apply) and `julia-f4-warm` remain
  UNAVAILABLE. Unresolved.
